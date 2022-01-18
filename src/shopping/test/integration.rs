use std::str::FromStr;
use crate::core::command::CommandHandler;
use crate::shopping::application::order_command::{CreateOrderCommand, OrderSubmitPaymentCommand};
use uuid::Uuid;

#[rocket::async_test]
async fn order_create_command_should_create_order() {
    // Act

    let create_order_result = CreateOrderCommand {
        customer_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    // Assert

    assert!(create_order_result.is_ok());
}

#[rocket::async_test]
async fn order_submit_payment_command_should_returnerror_when_orderdoesnotexist() {
    // Act
    let order_id = Uuid::new_v4().to_string();

    let submit_order_payment_result = OrderSubmitPaymentCommand {
        order_id: Uuid::from_str(&order_id).unwrap(),
        payment_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    // Assert

    assert!(submit_order_payment_result.is_err());
}

#[rocket::async_test]
async fn order_submit_payment_command_should_submitpayment_when_orderexists() {
    // Arrange

    let create_order_result = CreateOrderCommand {
        customer_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    let order_id = create_order_result.unwrap();

    // Act

    let submit_order_payment_result = OrderSubmitPaymentCommand {
        order_id,
        payment_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    // Assert

    assert!(submit_order_payment_result.is_ok());
}
