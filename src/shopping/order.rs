use uuid::Uuid;

use crate::core::aggregate::AggregateEntity;

pub struct Order {
    id: Uuid,
    payment_id: Option<String>,

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
    OrderPaymentSubmittedEvent { payment_id: String }
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

impl AggregateEntity<OrderEvent> for Order {
    fn uncommitted_events(&self) -> &Vec<OrderEvent> {
        &self.uncommitted_events
    }

    fn apply_event(&mut self, event: OrderEvent) {
        self.apply(&event as &OrderEvent);
        self.uncommitted_events.push(event);
    }
}

