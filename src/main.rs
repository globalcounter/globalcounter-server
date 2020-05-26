extern crate globalcounter;
use colored::*;
use hyper::Server;
use globalcounter::{constants, routes, utils};
use routerify::RouterService;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    startup();

    let addr = SocketAddr::new(
        utils::env(constants::env::HOST)
            .and_then(|host| host.parse::<IpAddr>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_IP),
        utils::env(constants::env::PORT)
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(constants::SERVER_DEFAULT_PORT),
    );

    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .http1_half_close(true)
        .http1_only(false)
        .http2_only(false)
        .http1_writev(true)
        .tcp_sleep_on_accept_errors(true)
        .serve(RouterService::new(routes::router()).unwrap());

    globalcounter::info!("App is serving on: {}", server.local_addr());
    if let Err(e) = server.await {
        globalcounter::error!("Server Error: {}", e);
    }
}

fn startup() {
    dotenv::dotenv().ok();
    globalcounter::logger::init_logger();
    log_app_env();
}

fn log_app_env() {
    println!("Environment Variables:");
    get_required_env_names()
        .map(|var| (var, utils::env(var).unwrap_or("<NOT_FOUND>".to_owned())))
        .for_each(|(var, val)| {
            println!("  {}: {}", var.color(Color::BrightBlack), val.color(Color::Green));
        });
    println!();
}

fn get_required_env_names() -> impl Iterator<Item = &'static str> {
    include_str!("../.env.example")
        .lines()
        .filter(|line| !line.starts_with("#") && !line.is_empty())
        .map(|line| line.split("=").take(1))
        .flatten()
}
