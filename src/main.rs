use futures::{pin_mut, FutureExt};
use service::start_service;

mod event;
mod service;

#[tokio::main]
async fn main() {
    let service = start_service().fuse();
    pin_mut!(service);
    tokio::select! {
        _ = service => {}
    };
}
