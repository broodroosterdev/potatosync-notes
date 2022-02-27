use crate::blob::{Data, Empty, DeleteRequest, GetDeletedRequest, GetDeletedResponse, GetUpdatedRequest, GetUpdatedResponse};
use crate::blob::blob_server::{Blob};

use sqlx::{Pool, Postgres};
use crate::db;
use tonic::{Status, Response};
use crate::db::DBData;

use crate::auth::{extract_claims};

pub struct BlobService {
    pub database: Pool<Postgres>,
}

#[tonic::async_trait]
impl Blob for BlobService {
    async fn sync(
        &self,
        request: tonic::Request<Data>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let blob = request.into_inner();
        return match db::sync(DBData {
            id: blob.id,
            account_id: claims.sub,
            blob_type: blob.blob_type,
            content: blob.content,
            last_changed: blob.last_changed,
        }, &self.database).await {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(status) => Err(status)
        };
    }

    async fn delete(
        &self,
        request: tonic::Request<DeleteRequest>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
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
            Ok(_) => Ok(Response::new(Empty {})),
            Err(status) => Err(status)
        };
    }

    async fn delete_all(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        return match db::delete_all("notes", claims.sub, &self.database).await {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(status) => Err(status)
        };
    }

    async fn get_deleted(
        &self,
        request: tonic::Request<GetDeletedRequest>,
    ) -> Result<tonic::Response<GetDeletedResponse>, tonic::Status> {
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
                Ok(Response::new(GetDeletedResponse {
                    list: deleted_id_list
                }))
            }
            Err(status) => Err(status)
        };
    }

    async fn get_updated(
        &self,
        request: tonic::Request<GetUpdatedRequest>,
    ) -> Result<tonic::Response<GetUpdatedResponse>, tonic::Status> {
        let claims;
        match extract_claims(request.metadata()) {
            Ok(result) => claims = result,
            Err(error) => return Err(Status::unauthenticated(error))
        }

        let request: GetUpdatedRequest = request.into_inner();
        return match db::get_updated(&request.blob_type, claims.sub, request.last_updated, &self.database).await {
            Ok(updated_blobs) => {
                let blobs = updated_blobs.into_iter().map(|data| {
                    Data {
                        id: data.id,
                        blob_type: data.blob_type,
                        content: data.content,
                        last_changed: data.last_changed,
                    }
                }).collect::<Vec<Data>>();
                Ok(Response::new(GetUpdatedResponse {
                    items: blobs
                }))
            }
            Err(status) => Err(status)
        };
    }
}
