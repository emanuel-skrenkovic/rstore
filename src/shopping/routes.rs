use std::str::FromStr;
use uuid::Uuid;

use crate::core::command::CommandHandler;
use crate::shopping::order_commands::{CreateOrderCommand, OrderSubmitPaymentCommand};

#[get("/shopping/actions/create-order")]
pub async fn create_order() -> String {
    let create_order_result = CreateOrderCommand {
        customer_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    format!("{}", create_order_result.unwrap())
}

#[get("/shopping/orders/<order_id>/actions/submit-payment")]
pub async fn submit_order_payment(order_id: String) -> &'static str {
    let submit_order_payment_result = OrderSubmitPaymentCommand {
        order_id: Uuid::from_str(&order_id).unwrap(),
        payment_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    match submit_order_payment_result {
        Ok(()) => "Ok",
        Err(_) => "Error",
    }
}
