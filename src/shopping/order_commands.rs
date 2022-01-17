use ::eventstore::EventData;
use uuid::Uuid;

use crate::core::aggregate::AggregateEntity;
use crate::core::command::CommandHandler;
use crate::core::eventstore;
use crate::error::result::Result;
use crate::shopping::order::Order;

pub struct CreateOrderCommand {
    pub customer_id: Uuid,
    pub session_id: Uuid,
}

#[async_trait]
impl CommandHandler<Result<Uuid>> for CreateOrderCommand {
    async fn execute(&self) -> Result<Uuid> {
        let client = eventstore::client().await?;

        let order = Order::new(&Uuid::new_v4(), &self.customer_id);

        let payload = EventData::json("order-event", &order.uncommitted_events());
        let _ = client
            .append_to_stream(order.id.to_string(), &Default::default(), payload?)
            .await?;

        Result::Ok(order.id)
    }
}
