use hyper::{Body, Request, Response};

pub async fn ping_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    resp_200!("It's working")
}
