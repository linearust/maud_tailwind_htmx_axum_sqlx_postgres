use crate::{
    constants::errors,
    data::errors::DataError,
    db::DB,
    models::{
        order::{Order, OrderSummary},
        OrderId, OrderNumber, UserId,
    },
};

pub async fn get_order(order_id: &OrderId) -> Result<Option<Order>, DataError> {
    let order: Option<Order> = DB.select(order_id.clone().into_record_id()).await?;
    Ok(order)
}

pub async fn get_order_by_order_number(order_number: &OrderNumber) -> Result<Option<Order>, DataError> {
    let mut result = DB
        .query("SELECT * FROM order WHERE order_number = $order_number LIMIT 1")
        .bind(("order_number", order_number.to_string()))
        .await?;

    let order: Option<Order> = result.take(0)?;
    Ok(order)
}

pub async fn get_orders_for_user(user_id: &UserId, limit: i64) -> Result<Vec<OrderSummary>, DataError> {
    let mut result = DB
        .query(
            "SELECT id, filename, file_size, text_length, price_amount, payment_status, order_number, created_at
             FROM order
             WHERE user = $user
             ORDER BY created_at DESC
             LIMIT $limit",
        )
        .bind(("user", user_id.clone().into_record_id()))
        .bind(("limit", limit))
        .await?;

    let orders: Vec<OrderSummary> = result.take(0)?;
    Ok(orders)
}

pub async fn get_order_for_user(order_id: &OrderId, user_id: &UserId) -> Result<Order, DataError> {
    let order = get_order(order_id)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;

    if order.user != *user_id {
        return Err(DataError::Unauthorized(errors::NOT_YOUR_ORDER));
    }
    Ok(order)
}

pub async fn get_order_by_order_number_for_user(
    order_number: &OrderNumber,
    user_id: &UserId,
) -> Result<Order, DataError> {
    let order = get_order_by_order_number(order_number)
        .await?
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;

    if order.user != *user_id {
        return Err(DataError::Unauthorized(errors::NOT_YOUR_ORDER));
    }
    Ok(order)
}
