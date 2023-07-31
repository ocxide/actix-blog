use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::sqlx::{insert::InsertErr, select::SelectErr},
    models::select_slice::SelectSlice,
    shared::db::{Pool, QueryResult},
};

#[derive(Serialize)]
pub struct Reply {
    pub id: Uuid,
    pub user_id: Uuid,
    pub comment_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
}

pub async fn get_many_by_parent(
    pool: &Pool,
    comment_id: Uuid,
    parent_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<Reply>, SelectErr> {
    query_as!(
        Reply,
        "SELECT id, user_id, comment_id, parent_id, content FROM replies \
                WHERE comment_id = $1 AND parent_id = $2 LIMIT $3 OFFSET $4",
        comment_id,
        parent_id,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn get_many(
    pool: &Pool,
    comment_id: Uuid,
    SelectSlice { limit, offset }: SelectSlice,
) -> Result<Vec<Reply>, SelectErr> {
    query_as!(
        Reply,
        "SELECT id, user_id, comment_id, parent_id, content FROM replies \
                WHERE comment_id = $1 LIMIT $2 OFFSET $3",
        comment_id,
        limit,
        offset,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.into())
}

pub async fn create(
    pool: &Pool,
    content: &str,
    user_id: Uuid,
    comment_id: Uuid,
    parent_id: Option<Uuid>,
) -> Result<QueryResult, InsertErr> {
    query!(
        "INSERT INTO replies (content, user_id, comment_id, parent_id) \
            VALUES ($1, $2, $3, $4)",
        content,
        user_id,
        comment_id,
        parent_id,
    )
    .execute(pool)
    .await
    .map_err(|e| e.into())
}
