use crate::prelude::*;
use crate::routes::api::v1::{counter::controllers, types::CounterOutput};
use hyper::{Body, Request, Response};

pub async fn counter_increase_post(_: Request<Body>) -> crate::Result<Response<Body>> {
    let latest_counter = controllers::increase_counter()
        .await
        .context("Failed to increase the counter")?;

    let output = CounterOutput {
        counter: latest_counter,
    };

    resp_200!(output)
}

pub async fn counter_decrease_post(_: Request<Body>) -> crate::Result<Response<Body>> {
    let latest_counter = controllers::decrease_counter()
        .await
        .context("Failed to decrease the counter")?;

    let output = CounterOutput {
        counter: latest_counter,
    };

    resp_200!(output)
}

pub async fn counter_value_get(_: Request<Body>) -> crate::Result<Response<Body>> {
    let latest_counter = controllers::latest_counter_value()
        .await
        .context("Failed to get the latest counter value")?;

    let output = CounterOutput {
        counter: latest_counter,
    };

    resp_200!(output)
}
