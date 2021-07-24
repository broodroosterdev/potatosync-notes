use tonic::Status;
use sqlx::{Postgres, Pool, Row};
use sqlx::types::Uuid;

pub struct DBData {
    pub id: String,
    pub account_id: String,
    pub content: Vec<u8>,
    pub last_changed: u64,
}

pub async fn add(table: &str, data: DBData, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(&*format!("INSERT INTO {} ( id, account_id, content, last_changed) VALUES ( uuid($1),uuid($2),$3,$4 )", table))
        .bind(data.id)
        .bind(data.account_id)
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

pub async fn update(table: &str, data: DBData, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(
        &*format!("UPDATE {} SET content = $1, last_changed = $2 WHERE account_id = uuid($3) AND id = uuid($4)", table))
        .bind(data.content)
        .bind(data.last_changed as i64)
        .bind(data.account_id)
        .bind(data.id)
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

pub async fn delete(table: &str, id: String, account_id: String, pool: &Pool<Postgres>) -> Result<(), Status> {
    return match sqlx::query(
        &*format!("DELETE FROM {} WHERE account_id = uuid($1) AND id = uuid($2)", table))
        .bind(account_id)
        .bind(id)
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

pub async fn get_ids(table: &str, account_id: String, pool: &Pool<Postgres>) -> Result<Vec<String>, Status> {
    return match sqlx::query(
        &*format!("SELECT id FROM {} WHERE account_id = uuid($1)", table))
        .bind(account_id)
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

pub async fn get_updated(table: &str, account_id: String, last_updated: u64, pool: &Pool<Postgres>) -> Result<Vec<DBData>, Status> {
    return match sqlx::query(
        &*format!("SELECT id, content, last_changed FROM {} WHERE account_id = uuid($1) AND last_changed > $2", table))
        .bind(account_id.clone())
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