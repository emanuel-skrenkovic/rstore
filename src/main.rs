mod core;
mod error;
mod shopping;

use crate::shopping::routes as order_routes;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, order_routes::create_order])
}
