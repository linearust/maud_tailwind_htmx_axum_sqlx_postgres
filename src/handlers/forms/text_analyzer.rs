use axum::{Extension, extract::{Multipart, State}, response::{IntoResponse, Redirect}};
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    constants::{errors, file_upload, pricing},
    data::{commands, errors::DataError},
    flash::FlashMessage,
    handlers::errors::HandlerResult,
    models::order::Order,
    paths,
};
use tower_sessions::Session;

pub async fn post_forms_text_analyzer(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    mut multipart: Multipart,
) -> HandlerResult {
    let (user_id, user_email) = match &current_user {
        CurrentUser::Authenticated { user_id, email, .. } => (*user_id, email.clone()),
        CurrentUser::Guest => unreachable!("Protected route accessed by guest"),
    };

    let mut filename: Option<String> = None;
    let mut file_size: Option<usize> = None;
    let mut text_content: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {}", e);
        DataError::InvalidInput(format!("Failed to process multipart data: {}", e))
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        if field_name == "file" {
            filename = field.file_name().map(|s| s.to_string());
            let data = field.bytes().await.map_err(|e| {
                tracing::error!("Failed to read file: {}", e);
                DataError::InvalidInput(format!("Failed to read file: {}", e))
            })?;

            file_size = Some(data.len());

            if data.len() > file_upload::MAX_FILE_SIZE {
                return Ok(FlashMessage::error(format!("File too large. Maximum size is {} MB.", file_upload::MAX_FILE_SIZE / 1024 / 1024))
                    .set_and_redirect(&session, paths::pages::TEXT_ANALYZER)
                    .await?);
            }

            text_content = Some(String::from_utf8(data.to_vec()).map_err(|e| {
                tracing::error!("Invalid UTF-8 in file: {}", e);
                DataError::InvalidInput("File must be valid UTF-8 text".to_string())
            })?);
        }
    }

    let filename = filename.ok_or(DataError::NotFound(errors::NO_FILE_PROVIDED))?;
    let file_size = file_size.ok_or(DataError::NotFound(errors::NO_FILE_PROVIDED))? as i32;
    let text_content = text_content.ok_or(DataError::NotFound(errors::NO_FILE_CONTENT))?;

    let text_length = text_content.chars().count() as i32;
    let calculated_price = text_length * pricing::PRICE_PER_CHARACTER;
    let price_amount = calculated_price.max(pricing::MINIMUM_ORDER_AMOUNT);

    let order_number = Order::generate_order_number(user_id);

    let order = commands::order::create_order(
        &db,
        commands::order::CreateOrderParams {
            user_id,
            user_email,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            order_number,
        },
    ).await?;

    Ok(Redirect::to(&paths::helpers::quote_path(&order.order_id)).into_response())
}
