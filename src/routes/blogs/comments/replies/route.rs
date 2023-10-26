use actix_web::{
    get, post,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    Responder,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    routes::blogs::comments::create_comment::CreateComment,
    services::auth::claims::Claims,
    shared::{
        db::{models::replies, Pool},
        extractors::partial_query::PartialQuery,
        models::select_slice::SelectSlice,
    },
    traits::{catch_http::CatchHttp, into_response::IntoResponse, json_result::JsonResult},
};

#[derive(Debug, Deserialize)]
pub struct ParentUuid {
    pub parent_id: Option<Uuid>,
}

#[get("/")]
pub async fn get_all(
    pool: Data<Pool>,
    path: Path<(Uuid, Uuid)>,
    parent_id: Query<ParentUuid>,
    slice: PartialQuery<SelectSlice>,
) -> impl Responder {
    let (_blog_id, comment_id) = path.into_inner();
    let res = match parent_id.into_inner().parent_id {
        Some(parent_id) => {
            replies::get_many_by_parent(pool.get_ref(), comment_id, parent_id, slice.into_inner())
                .await
        }
        None => replies::get_many(pool.get_ref(), comment_id, slice.into_inner()).await,
    };

    res.json_result()
}

#[post("/")]
pub async fn create(
    pool: Data<Pool>,
    path: Path<(Uuid, Uuid)>,
    Claims { id, .. }: Claims,
    req: Json<CreateComment>,
    parent_id: Query<ParentUuid>,
) -> actix_web::Result<impl Responder> {
    req.validate().catch_http()?;
    let (_blog_id, comment_id) = path.into_inner();

    replies::create(
        pool.get_ref(),
        &req.content,
        id,
        comment_id,
        parent_id.parent_id,
    )
    .await
    .into_response()
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{comment_id}/replies")
            .service(create)
            .service(get_all),
    );
}
