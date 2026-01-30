use axum::{Extension, extract::Multipart, response::{IntoResponse, Redirect}};
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    constants::{errors, file_upload, pricing},
    data::{commands, errors::DataError},
    handlers::errors::HandlerResult,
    models::OrderNumber,
    paths,
    session::FlashMessage,
};

struct ParsedUpload {
    filename: String,
    file_size: i32,
    text_content: String,
}

enum ParseResult {
    Success(ParsedUpload),
    FileTooLarge,
}

async fn parse_file_upload(mut multipart: Multipart) -> Result<ParseResult, DataError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {}", e);
        DataError::InvalidInput(format!("Failed to process multipart data: {}", e))
    })? {
        let field_name = match field.name() {
            Some(name) => name,
            None => {
                tracing::warn!("Multipart field without name, skipping");
                continue;
            }
        };
        if field_name != "file" {
            continue;
        }

        let filename = field.file_name().map(|s| s.to_string());
        let data = field.bytes().await.map_err(|e| {
            tracing::error!("Failed to read file: {}", e);
            DataError::InvalidInput(format!("Failed to read file: {}", e))
        })?;

        if data.len() > file_upload::MAX_FILE_SIZE {
            return Ok(ParseResult::FileTooLarge);
        }

        let text_content = String::from_utf8(data.to_vec()).map_err(|e| {
            tracing::error!("Invalid UTF-8 in file: {}", e);
            DataError::InvalidInput("File must be valid UTF-8 text".to_string())
        })?;

        return Ok(ParseResult::Success(ParsedUpload {
            filename: filename.ok_or(DataError::NotFound(errors::NO_FILE_PROVIDED))?,
            file_size: data.len() as i32,
            text_content,
        }));
    }

    Err(DataError::NotFound(errors::NO_FILE_PROVIDED))
}

pub async fn post_forms_text_analyzer(
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    multipart: Multipart,
) -> HandlerResult {
    let (user_id, user_email) = match &current_user {
        CurrentUser::Authenticated { user_id, email, .. } => (user_id.clone(), email.clone()),
        CurrentUser::Guest => unreachable!("Protected route accessed by guest"),
    };

    let upload = match parse_file_upload(multipart).await? {
        ParseResult::FileTooLarge => {
            return Ok(FlashMessage::error(format!("File too large. Maximum size is {} MB.", file_upload::MAX_FILE_SIZE / 1024 / 1024))
                .set_and_redirect(&session, paths::pages::TEXT_ANALYZER)
                .await?);
        }
        ParseResult::Success(upload) => upload,
    };

    let text_length = upload.text_content.chars().count() as i32;
    let calculated_price = text_length * pricing::PRICE_PER_CHARACTER;
    let price_amount = calculated_price.max(pricing::MINIMUM_ORDER_AMOUNT);

    let order_number = OrderNumber::generate(&user_id);

    let order = commands::order::create_order(
        commands::order::CreateOrderParams {
            user_id,
            user_email,
            filename: upload.filename,
            file_size: upload.file_size,
            text_content: upload.text_content,
            text_length,
            price_amount,
            order_number,
        },
    ).await?;

    Ok(Redirect::to(&paths::helpers::quote_path(&order.id)).into_response())
}
