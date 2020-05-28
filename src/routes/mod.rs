use crate::constants;
use crate::types::AppInfo;
use crate::utils;
use hyper::{Body, Request, Response};
use routerify::{Middleware, Router};
use routerify_cors::enable_cors_all;
use tokio::time::{self, Duration};

mod api;
mod ws;

pub fn router() -> Router<Body, crate::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger_middleware))
        .middleware(enable_cors_all())
        .get("/", home_get)
        .scope("/api", api::router())
        .scope("/ws", ws::router())
        .err_handler(error_handler)
        .build()
        .unwrap()
}

async fn logger_middleware(req: Request<Body>) -> crate::Result<Request<Body>> {
    info!(
        "{} {} {}",
        utils::extract_client_ip_from_req(&req),
        req.method(),
        req.uri()
    );
    Ok(req)
}

async fn home_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    resp_200!(AppInfo {
        name: constants::APP_NAME,
        version: constants::APP_VERSION,
        description: constants::APP_DESCRIPTION,
    })
}

async fn error_handler(err: routerify::Error) -> Response<Body> {
    error!("{}", err);
    resp_500!("{}", err).expect("Couldn't create a response while handling the server error")
}

pub async fn startup_routes() -> crate::Result<()> {
    tokio::spawn(async move {
        let period = Duration::from_secs(1 * 60 * 60);
        loop {
            info!("Next ws client shaking after: {:?}", period);
            time::delay_for(period).await;
            ws::v1::shake_client_registry().await;
            info!("WS client shaking is done");
        }
    });

    Ok(())
}
