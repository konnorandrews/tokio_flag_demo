use once_cell::sync::{OnceCell, Lazy};
use dashmap::DashSet;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

type DelayQueue<T> = futures_delay_queue::DelayQueue<T, futures_intrusive::buffer::GrowingHeapBuf<T>>;

static TIMEOUT_RESET_QUEUE: OnceCell<DelayQueue<(u64, u64)>> = OnceCell::new();
static TIMEOUTS: Lazy<DashSet<(u64, u64)>> = Lazy::new(DashSet::new);

#[tokio::main]
async fn main() {
    let (delay_queue, expired_items) = futures_delay_queue::delay_queue();
    TIMEOUT_RESET_QUEUE.set(delay_queue).expect("Failed to init flag reset queue.");

    tokio::spawn(async move {
        while let Some(key) = expired_items.receive().await {
            TIMEOUTS.remove(&key).expect("Key wasn't in timeout set.");
            println!("Timeout was removed.");
        }
    });

    tokio::spawn(async move {
        let key = (0, 0);
        if !TIMEOUTS.insert(key) {
            println!("Timeout is active");
        } else {
            let start_time = std::time::Instant::now();
            println!("Do operation on item.");
            let reset_time = start_time + std::time::Duration::from_secs(5);
            TIMEOUT_RESET_QUEUE.get().expect("flag reset queue not init.").insert_at(key, reset_time);
        }
    }).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    tokio::spawn(async move {
        let key = (0, 0);
        if !TIMEOUTS.insert(key) {
            println!("Timeout is active");
        } else {
            let start_time = std::time::Instant::now();
            println!("Do operation on item.");
            let reset_time = start_time + std::time::Duration::from_secs(5);
            TIMEOUT_RESET_QUEUE.get().expect("flag reset queue not init.").insert_at(key, reset_time);
        }
    }).await.unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    tokio::spawn(async move {
        let key = (0, 0);
        if !TIMEOUTS.insert(key) {
            println!("Timeout is active");
        } else {
            let start_time = std::time::Instant::now();
            println!("Do operation on item.");
            let reset_time = start_time + std::time::Duration::from_secs(5);
            TIMEOUT_RESET_QUEUE.get().expect("flag reset queue not init.").insert_at(key, reset_time);
        }
    }).await.unwrap();
}
