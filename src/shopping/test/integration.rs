use crate::core::command::CommandHandler;
use crate::core::test::infrastructure::EventStoreFixture;
use crate::shopping::application::order_command::{CreateOrderCommand, OrderSubmitPaymentCommand};
use crate::shopping::domain::order::{OrderEvent, OrderEventKind};
use std::str::FromStr;
use uuid::Uuid;

#[rocket::async_test]
async fn order_create_command_should_create_order() {
    // Arrange
    let fixture = EventStoreFixture::create().await;

    // Act

    let initial_customer_id = Uuid::new_v4();
    let initial_session_id = Uuid::new_v4();

    let create_order_result = CreateOrderCommand {
        customer_id: initial_customer_id,
        session_id: initial_session_id,
    }
    .execute()
    .await;

    // Assert

    assert!(create_order_result.is_ok());

    let order_id = create_order_result.unwrap();

    let events: Vec<OrderEvent> = fixture.events(&order_id.to_string()).await;

    assert!(!events.is_empty());

    let event = events.first().unwrap();
    assert_eq!(order_id, event.order_id);
    assert!(matches!(
        event.kind,
        OrderEventKind::OrderCreatedEvent { customer_id: _ }
    ));

    // TODO: is there a cleaner way?
    if let OrderEventKind::OrderCreatedEvent { customer_id } = event.kind {
        assert_eq!(initial_customer_id, customer_id);
    }
}

#[rocket::async_test]
async fn order_submit_payment_command_should_returnerror_when_orderdoesnotexist() {
    // Arrange

    let fixture = EventStoreFixture::create().await;

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

    let events: Vec<OrderEvent> = fixture.events(&order_id).await;
    assert!(events.is_empty());
}

#[rocket::async_test]
async fn order_submit_payment_command_should_submitpayment_when_orderexists() {
    // Arrange

    let fixture = EventStoreFixture::create().await;

    let create_order_result = CreateOrderCommand {
        customer_id: Uuid::new_v4(),
        session_id: Uuid::new_v4(),
    }
    .execute()
    .await;

    let order_id = create_order_result.unwrap();

    // Act

    let test_payment_id = Uuid::new_v4();

    let submit_order_payment_result = OrderSubmitPaymentCommand {
        order_id,
        payment_id: test_payment_id
    }
    .execute()
    .await;

    // Assert

    assert!(submit_order_payment_result.is_ok());

    let events: Vec<OrderEvent> = fixture.events(&order_id.to_string()).await;
    assert!(!events.is_empty());

    // TODO: this is bad!
    let event = &events[1];
    assert_eq!(order_id, event.order_id);
    assert!(matches!(
        event.kind,
        OrderEventKind::OrderPaymentSubmittedEvent { payment_id: _ }
    ));

    // TODO: is there a cleaner way?
    if let OrderEventKind::OrderPaymentSubmittedEvent { payment_id } = event.kind {
        assert_eq!(test_payment_id, payment_id);
    }
}
