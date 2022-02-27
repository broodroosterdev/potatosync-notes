use tonic::Status;
use sqlx::{Postgres, Pool, Row};
use sqlx::types::Uuid;

pub struct DBData {
    pub id: String,
    pub account_id: String,
    pub blob_type: String,
    pub content: Vec<u8>,
    pub last_changed: u64,
}

const SYNC_QUERY: &str = "INSERT INTO blob ( id, account_id, blob_type, content, last_changed)
VALUES ( uuid($1),uuid($2),$3,$4,$5 )
ON CONFLICT ON CONSTRAINT blob_pkey DO
UPDATE SET content = $4, last_changed = $5";

pub async fn sync(data: DBData, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(SYNC_QUERY)
        .bind(data.id)
        .bind(data.account_id)
        .bind(data.blob_type)
        .bind(data.content)
        .bind(data.last_changed as i64)
        .execute(pool).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            match e {
                sqlx::error::Error::Database(database_error) => {
                    if let Some(code) = database_error.code() {
                        //This is the error code for a duplicate key in Postgres
                        if code.to_string() == "23505" {
                            return Err(Status::already_exists("AlreadyExists"))
                        } else {
                            println!("{}", database_error.to_string());
                        }
                    } else {
                        println!("{}", database_error.to_string());
                    }
                },
                _ => {
                    println!("{}", e.to_string());
                }
            }
            Err(Status::internal("DatabaseError"))
        }
    };
}

pub async fn delete(blob_type: &str, id: String, account_id: String, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(
        &*"DELETE FROM blob WHERE account_id = uuid($1) AND id = uuid($2) AND blob_type = $3")
        .bind(account_id)
        .bind(id)
        .bind(blob_type)
        .execute(pool).await {
        Ok(result) => {
            return if result.rows_affected() > 0 {
                Ok(())
            } else {
                Err(Status::not_found("NotFound"))
            };
        }
        Err(e) => {
            println!("{}", e.to_string());
            Err(Status::internal("DatabaseError"))
        }
    };
}

pub async fn delete_all(table: &str, account_id: String, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(
        &*format!("DELETE FROM {} WHERE account_id = uuid($1)", table))
        .bind(account_id)
        .execute(pool).await {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            println!("{}", e.to_string());
            Err(Status::internal("DatabaseError"))
        }
    };
}

pub async fn get_ids(blob_type: &str, account_id: String, pool: &Pool<Postgres>) -> Result<Vec<String>, Status> {
    return match sqlx::query(
        &*"SELECT id FROM blob WHERE account_id = uuid($1) AND blob_type = $2")
        .bind(account_id)
        .bind(blob_type)
        .fetch_all(pool).await {
        Ok(rows) => {
            let mut list: Vec<String> = Vec::new();
            for row in rows {
                let raw_uuid: Uuid = row.get(0);
                list.push(raw_uuid.to_hyphenated().to_string())
            }
            Ok(list)
        }
        Err(e) => {
            println!("{}", e.to_string());
            Err(Status::internal("DatabaseError"))
        }
    };
}

pub async fn get_updated(blob_type: &str, account_id: String, last_updated: u64, pool: &Pool<Postgres>) -> Result<Vec<DBData>, Status> {
    return match sqlx::query(
        &*"SELECT id, content, last_changed FROM blob WHERE account_id = uuid($1) AND blob_type = $2 AND last_changed > $3")
        .bind(account_id.clone())
        .bind(blob_type)
        .bind(last_updated as i64)
        .fetch_all(pool).await {
        Ok(rows) => {
            let mut list: Vec<DBData> = Vec::new();
            for row in rows {
                let raw_id: Uuid = row.get(0);
                let content: Vec<u8> = row.get(1);
                let last_changed: i64 = row.get(2);
                list.push(DBData {
                    id: raw_id.to_hyphenated().to_string(),
                    account_id: account_id.clone(),
                    blob_type: blob_type.parse().unwrap(),
                    content,
                    last_changed: last_changed as u64
                })
            }
            Ok(list)
        },
        Err(e) => {
            println!("{}", e.to_string());
            Err(Status::internal("DatabaseError"))
        }
    }
}