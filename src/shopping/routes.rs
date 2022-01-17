use uuid::Uuid;

use crate::core::command::CommandHandler;
use crate::shopping::order_commands::CreateOrderCommand;

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
