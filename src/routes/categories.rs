mod sub_categories;
mod tags;

mod create_one;

use actix_web::{
    delete, get,
    web::{scope, Data, Path, ServiceConfig},
    Responder,
};
use uuid::Uuid;

use crate::{
    modules::{admin::IsAdminFactory, category, db::Pool},
    sqlx::{deleted_response, select_response},
};

#[get("/")]
async fn get_all(pool: Data<Pool>) -> impl Responder {
    let result = category::get_all_categories(pool.as_ref()).await;
    select_response(result)
}

#[delete("/{id}/")]
async fn delete_one(id: Path<Uuid>, pool: Data<Pool>) -> impl Responder {
    let result = category::delete_category(pool.as_ref(), id.into_inner()).await;
    deleted_response(result)
}

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/categories")
            .service(get_all)
            .configure(sub_categories::router)
            .configure(tags::router)
            .wrap(IsAdminFactory)
            .service(delete_one)
            .service(create_one::endpoint),
    );
}
