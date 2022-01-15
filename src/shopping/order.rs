use uuid::Uuid;

use crate::core::aggregate::AggregateEntity;
use crate::error::result::{Result, Error};

pub struct Order {
    id: Uuid,
    payment_id: Option<Uuid>,

    uncommitted_events: Vec<OrderEvent>
}

#[derive(Debug)]
pub struct OrderEvent {
    pub order_id: Uuid,
    pub kind: OrderEventKind
}

#[derive(Debug)]
pub enum OrderEventKind {
    OrderCreatedEvent,
    OrderPaymentSubmittedEvent { payment_id: Uuid }
}

impl Order {
    pub fn new(id: &Uuid) -> Order {
        let mut order = Order {
            id: id.to_owned(),
            payment_id: None,
            uncommitted_events: vec![]
        };

        order.apply_event(OrderEvent {
            order_id: id.to_owned(),
            kind: OrderEventKind::OrderCreatedEvent
        });

        order
    }

    pub fn submit_payment(&mut self, payment_id: &Uuid) -> Result<()> {
        if self.payment_id.is_some() {
            return Result::Err(
                Error::Input {
                    message: format!("Payment with id {} was already added", self.payment_id.unwrap())
                })
        }

        self.apply_event(OrderEvent {
            order_id: self.id,
            kind: OrderEventKind::OrderPaymentSubmittedEvent { payment_id: payment_id.to_owned() }
        });

        Result::Ok(())
    }
}

impl AggregateEntity<OrderEvent> for Order {
    fn uncommitted_events(&self) -> &Vec<OrderEvent> {
        &self.uncommitted_events
    }

    fn uncommitted_events_mut(&mut self) -> &mut Vec<OrderEvent> {
        &mut self.uncommitted_events
    }

    fn apply(&mut self, event: &OrderEvent) {
        match &event.kind {
            OrderEventKind::OrderCreatedEvent => {
                self.id = event.order_id.to_owned();
            }
            OrderEventKind::OrderPaymentSubmittedEvent { payment_id } => {
                self.payment_id = Some(payment_id.to_owned());
            }
        }
    }
}

