use crate::routes::ws::v1::publish_counter_value;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<i64> = Mutex::new(0);
}

pub async fn increase_counter() -> crate::Result<i64> {
    let mut lock = COUNTER.lock().await;
    *lock += 1;
    let value = *lock;
    publish_latest_counter_value(value).await;
    Ok(value)
}

pub async fn decrease_counter() -> crate::Result<i64> {
    let mut lock = COUNTER.lock().await;
    *lock -= 1;
    let value = *lock;
    publish_latest_counter_value(value).await;
    Ok(value)
}

pub async fn publish_latest_counter_value(value: i64) {
    tokio::spawn(async move {
        if let Err(err) = publish_counter_value(value).await {
            warn!(
                "Couldn't publish the new counter value to the connected devices: {}",
                err
            );
        }
    });
}

pub async fn latest_counter_value() -> crate::Result<i64> {
    let lock = COUNTER.lock().await;
    Ok(*lock)
}
