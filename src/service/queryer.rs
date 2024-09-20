use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status};

use super::relx::{
    relx_queryer_server::RelxQueryer, Entity, ExistResponse, ListObjectRequest, ListUserRequest,
    Tuple,
};

pub struct Queryer {}

impl Queryer {
    pub(super) fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl RelxQueryer for Queryer {
    async fn exist(&self, request: Request<Tuple>) -> Result<Response<ExistResponse>, Status> {
        todo!()
    }

    type StreamUsersStream = Pin<Box<dyn Stream<Item = Result<Entity, Status>> + Send>>;
    async fn stream_users(
        &self,
        request: Request<ListUserRequest>,
    ) -> Result<tonic::Response<Self::StreamUsersStream>, tonic::Status> {
        todo!()
    }

    type StreamObjectsStream = Pin<Box<dyn Stream<Item = Result<Entity, Status>> + Send>>;
    async fn stream_objects(
        &self,
        request: Request<ListObjectRequest>,
    ) -> Result<tonic::Response<Self::StreamObjectsStream>, tonic::Status> {
        todo!()
    }
}
