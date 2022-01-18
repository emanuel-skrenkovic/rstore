use uuid::Uuid;

use crate::core::command::CommandHandler;
use crate::core::eventstore;
use crate::core::eventstore::AggregateStore;
use crate::core::result::Result;
use crate::shopping::domain::order::{Order, OrderEvent};

pub struct CreateOrderCommand {
    pub customer_id: Uuid,
    pub session_id: Uuid,
}

#[async_trait]
impl CommandHandler<Result<Uuid>> for CreateOrderCommand {
    async fn execute(&self) -> Result<Uuid> {
        let order_id = Uuid::new_v4();
        let order = Order::new(&order_id, &self.customer_id);

        let store: AggregateStore = eventstore::AggregateStore::create().await;

        store.save(&order_id.to_string(), order).await?;

        Result::Ok(order_id)
    }
}

pub struct OrderSubmitPaymentCommand {
    pub order_id: Uuid,
    pub payment_id: Uuid,
}

#[async_trait]
impl CommandHandler<Result<()>> for OrderSubmitPaymentCommand {
    async fn execute(&self) -> Result<()> {
        let store: AggregateStore = eventstore::AggregateStore::create().await;

        let mut order = store
            .load::<Order, OrderEvent>(self.order_id.to_string())
            .await?;

        order.submit_payment(&self.payment_id)?;

        store.save(&self.order_id.to_string(), order).await
    }
}
