use tonic::{Request, Response, Status, Streaming};

use super::relx::{
    relx_manager_server::RelxManager,
    Empty, Tuple,
};

pub struct Manager {}

impl Manager {
    pub(super) fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl RelxManager for Manager {
    async fn create_tuples(
        &self,
        request: Request<Streaming<Tuple>>,
    ) -> Result<Response<Empty>, Status> {
        todo!()
    }

    async fn delete_tuples(
        &self,
        request: Request<Streaming<Tuple>>,
    ) -> std::result::Result<tonic::Response<Empty>, tonic::Status> {
        todo!()
    }
}
