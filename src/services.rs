use crate::notes::{Note, Empty, DeleteRequest, GetDeletedRequest, GetDeletedResponse, GetUpdatedRequest, GetUpdatedResponse};
use crate::notes::notes_server::{Notes};

use sqlx::{Pool, Postgres};
use crate::db;
use tonic::{Status, Response};
use crate::db::Data;

pub struct MyNotes {
    pub database: Pool<Postgres>
}

#[tonic::async_trait]
impl Notes for MyNotes {
    async fn add(
        &self,
        request: tonic::Request<Note>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        //let claims;
        /*match extract_claims(request.metadata()){
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }*/

        let note = request.into_inner();
        return match db::add("notes", Data {
            id: note.id,
            account_id: "f84ce08a-6ec5-4457-bebb-e7fe6f480d0a".to_string(),
            content: note.content,
            last_changed: note.last_changed
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn update(
        &self,
        request: tonic::Request<Note>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        //let claims;
        /*match extract_claims(request.metadata()){
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }*/

        let note = request.into_inner();
        return match db::update("notes", Data {
            id: note.id,
            account_id: "f84ce08a-6ec5-4457-bebb-e7fe6f480d0a".to_string(),
            content: note.content,
            last_changed: note.last_changed
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let data = request.into_inner();
        return match db::delete("notes", data.id, "f84ce08a-6ec5-4457-bebb-e7fe6f480d0a".to_string(), &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn delete_all(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        return match db::delete_all("notes", "f84ce08a-6ec5-4457-bebb-e7fe6f480d0a".to_string(), &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn get_deleted(
        &self,
        request: tonic::Request<GetDeletedRequest>,
    ) -> Result<tonic::Response<GetDeletedResponse>, tonic::Status>{
        return match db::get_ids("notes", "f84ce08a-6ec5-4457-bebb-e7fe6f480d0a".to_string(), &self.database).await {
            Ok(saved_id_list) => {
                let given_id_list: Vec<String> = request.into_inner().list;
                let deleted_id_list: Vec<String> = given_id_list
                    .into_iter()
                    .filter(|id| !saved_id_list.contains(id))
                    .collect();
                Ok(Response::new(GetDeletedResponse{
                    list: deleted_id_list
                }))
            },
            Err(status) => Err(status)
        };
    }

    async fn get_updated(
        &self,
        request: tonic::Request<GetUpdatedRequest>,
    ) -> Result<tonic::Response<GetUpdatedResponse>, tonic::Status>{
        Ok(Response::new(GetUpdatedResponse{
            notes: Vec::new()
        }))
    }
}