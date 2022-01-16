use std::process::Command;
use uuid::Uuid;

use crate::shopping::order::Order;
use crate::error::result::Result;
use crate::core::command::CommandHandler;

pub struct CreateOrderCommand {
    pub customer_id: Uuid,
    pub session_id: Uuid
}

impl CommandHandler<Result<Uuid>> for CreateOrderCommand {
    fn execute(&self) -> Result<Uuid> {
        let order = Order::new(
            &Uuid::new_v4(),
            &self.customer_id);

        Result::Ok(order.id)
    }
}