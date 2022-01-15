use uuid::Uuid;

use crate::core::aggregate::AggregateEntity;
use crate::shopping::order::{Order, OrderEvent};

#[get("/shopping/actions/create-order")]
pub async fn create_order() -> String {
    let order = Order::new(&Uuid::new_v4());

    let test = order.uncommitted_events();
    format!("{}", test.first().unwrap().order_id)
}