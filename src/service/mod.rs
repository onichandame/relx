use manager::Manager;
use queryer::Queryer;
use tonic::transport::Server;

use relx::{relx_manager_server::RelxManagerServer, relx_queryer_server::RelxQueryerServer};

mod manager;
mod queryer;
mod relx;

pub(crate) async fn start_service() {
    Server::builder()
        .add_service(RelxManagerServer::new(Manager::new()))
        .add_service(RelxQueryerServer::new(Queryer::new()))
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .unwrap();
}
