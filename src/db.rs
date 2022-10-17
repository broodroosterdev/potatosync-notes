use crate::errors::ApiError;
use crate::models::Blob;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn upsert_blob(conn: &mut PgConnection, blob: &Blob) -> Result<usize, ApiError> {
    use crate::schema::blobs::dsl::*;
    diesel::insert_into(blobs)
        .values(blob)
        .on_conflict((id, account_id))
        .do_update()
        .set(blob)
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn delete_all_blobs(
    conn: &mut PgConnection,
    given_account_id: &str,
) -> Result<usize, ApiError> {
    use crate::schema::blobs::dsl::*;
    diesel::delete(blobs)
        .filter(account_id.eq(given_account_id))
        .execute(conn)
        .map_err(|error| ApiError::DBError(error))
}

pub fn get_blobs(conn: &mut PgConnection, given_account_id: &str) -> Result<Vec<Blob>, ApiError> {
    use crate::schema::blobs::dsl::*;
    blobs
        .filter(account_id.eq(given_account_id))
        .load::<Blob>(conn)
        .map_err(|error| ApiError::DBError(error))
}
