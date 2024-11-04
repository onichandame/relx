use db_entity::tuple::{self};
use futures::StreamExt;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use tonic::{Request, Response, Status, Streaming};

use super::relx::{relx_manager_server::RelxManager, tuple::User, Empty, Tuple};

pub struct Manager {
    db: DatabaseConnection,
}

impl Manager {
    pub(super) fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl RelxManager for Manager {
    async fn create_tuples(
        &self,
        mut request: Request<Streaming<Tuple>>,
    ) -> Result<Response<Empty>, Status> {
        while let Some(Ok(tuple_req)) = request.get_mut().next().await {
            tuple_req
                .try_into_tuple_record()?
                .insert(&self.db)
                .await
                .map_err(|db_err| Status::internal(db_err.to_string()))?;
        }
        Ok(Response::new(Empty {}))
    }

    async fn delete_tuples(
        &self,
        mut request: Request<Streaming<Tuple>>,
    ) -> Result<Response<Empty>, Status> {
        while let Some(Ok(tuple_req)) = request.get_mut().next().await {
            let mut cursor = tuple::Entity::find();
            if let Some(object) = tuple_req.object {
                if let Some(namespace) = object.namespace {}
            } else {
                return Err(Status::invalid_argument(
                    "Object must be provided in DeleteTuples",
                ));
            }
        }
        Ok(Response::new(Empty {}))
    }
}

trait TryIntoTupleRecord {
    fn try_into_tuple_record(self) -> Result<tuple::ActiveModel, Status>;
}

impl TryIntoTupleRecord for Tuple {
    fn try_into_tuple_record(self) -> Result<tuple::ActiveModel, Status> {
        let object = self
            .object
            .ok_or(Status::invalid_argument("object must be provided"))?;
        let user = self
            .user
            .ok_or(Status::invalid_argument("user must be provided"))?;
        let relation = self
            .relation
            .ok_or(Status::invalid_argument("relation must be provided"))?;
        let (user_set_namespace, user_set_name, user_set_relation) = match user {
            User::Single(user) => (user.namespace, user.name, None),
            User::UserSet(user_set) => {
                let object = user_set.object.ok_or(Status::invalid_argument(
                    "object in UserSet must be provided",
                ))?;
                (object.namespace, object.name, Some(user_set.relation))
            }
        };
        Ok(tuple::ActiveModel {
            object_namespace: self
                .object
                .and_then(|v| v.namespace)
                .map(|v| Set(v))
                .unwrap_or(NotSet),
            object_name: Set(object.name),
            relation: Set(relation),
            user_set_namespace: Set(user_set_namespace),
            user_set_name: Set(user_set_name),
            user_set_relation: Set(user_set_relation),
            ..Default::default()
        })
    }
}
