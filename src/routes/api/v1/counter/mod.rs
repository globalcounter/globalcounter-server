use hyper::Body;
use routerify::Router;

mod controllers;
mod handlers;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .post("/increase", handlers::counter_increase_post)
        .post("/decrease", handlers::counter_decrease_post)
        .get("/value", handlers::counter_value_get)
        .build()
        .unwrap()
}
