use hyper::Body;
use routerify::Router;

mod counter;
mod helpers;
mod ping;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .get("/ping", ping::handlers::ping_get)
        .scope("/counter", counter::router())
        .build()
        .unwrap()
}
