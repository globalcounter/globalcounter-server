use hyper::{Body, Request, Response};

pub async fn counter_increase_post(_: Request<Body>) -> crate::Result<Response<Body>> {
    todo!()
}

pub async fn counter_decrease_post(_: Request<Body>) -> crate::Result<Response<Body>> {
    todo!()
}

pub async fn counter_value_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    todo!()
}
