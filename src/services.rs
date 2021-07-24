use crate::notes::{Data, Empty, DeleteRequest, GetDeletedRequest, GetDeletedResponse, GetUpdatedRequest, GetUpdatedResponse};
use crate::notes::notes_server::{Notes};
use crate::notes::tags_server::{Tags};
use crate::notes;

use sqlx::{Pool, Postgres};
use crate::db;
use tonic::{Status, Response};
use crate::db::DBData;

use crate::auth::{extract_claims};

pub struct MyNotes {
    pub database: Pool<Postgres>
}

#[tonic::async_trait]
impl Notes for MyNotes {
    async fn add(
        &self,
        request: tonic::Request<Data>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let claims;
        match extract_claims(request.metadata()){
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let note = request.into_inner();
        return match db::add("notes", DBData {
            id: note.id,
            account_id: claims.sub,
            content: note.content,
            last_changed: note.last_changed
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn update(
        &self,
        request: tonic::Request<notes::Data>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let note = request.into_inner();
        return match db::update("notes", DBData {
            id: note.id,
            account_id: claims.sub,
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
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let data = request.into_inner();
        return match db::delete(
            "notes",
                 data.id,
            claims.sub,
            &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn delete_all(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        return match db::delete_all("notes", claims.sub, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn get_deleted(
        &self,
        request: tonic::Request<GetDeletedRequest>,
    ) -> Result<tonic::Response<GetDeletedResponse>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        return match db::get_ids("notes", claims.sub, &self.database).await {
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
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let last_updated = request.into_inner().last_updated.unwrap();
        return match db::get_updated("notes", claims.sub, last_updated, &self.database).await {
            Ok(updated_notes) => {
                let notes = updated_notes.into_iter().map(|data| {
                    notes::Data{
                        id: data.id,
                        content: data.content,
                        last_changed: data.last_changed
                    }
                }).collect::<Vec<Data>>();
                Ok(Response::new(GetUpdatedResponse{
                    items: notes
                }))
            },
            Err(status) => Err(status)
        };
    }
}


pub struct MyTags {
    pub database: Pool<Postgres>
}

#[tonic::async_trait]
impl Tags for MyTags {
    async fn add(
        &self,
        request: tonic::Request<Data>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let claims;
        match extract_claims(request.metadata()){
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let tag = request.into_inner();
        return match db::add("tags", DBData {
            id: tag.id,
            account_id: claims.sub,
            content: tag.content,
            last_changed: tag.last_changed
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn update(
        &self,
        request: tonic::Request<Data>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()){
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let tag = request.into_inner();
        return match db::update("tags", DBData {
            id: tag.id,
            account_id: claims.sub,
            content: tag.content,
            last_changed: tag.last_changed
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        }
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let data = request.into_inner();
        return match db::delete("tags", data.id, claims.sub, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn delete_all(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        return match db::delete_all("tags", claims.sub, &self.database).await {
            Ok(_) => Ok(Response::new(Empty{})),
            Err(status) => Err(status)
        };
    }

    async fn get_deleted(
        &self,
        request: tonic::Request<GetDeletedRequest>,
    ) -> Result<tonic::Response<GetDeletedResponse>, tonic::Status>{
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        return match db::get_ids("tags", claims.sub, &self.database).await {
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
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let last_updated = request.into_inner().last_updated.unwrap();
        return match db::get_updated("tags", claims.sub, last_updated, &self.database).await {
            Ok(updated_tags) => {
                let tags = updated_tags.into_iter().map(|data| {
                    Data {
                        id: data.id,
                        content: data.content,
                        last_changed: data.last_changed
                    }
                }).collect::<Vec<Data>>();
                Ok(Response::new(GetUpdatedResponse{
                    items: tags
                }))
            },
            Err(status) => Err(status)
        };
    }
}