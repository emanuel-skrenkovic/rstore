mod core;
mod shopping;

use crate::shopping::routes as order_routes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            order_routes::create_order,
            order_routes::submit_order_payment
        ],
    )
}
