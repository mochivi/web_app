mod create;
mod get;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
        .route("create/{title}", post().to(create::create))
        .route("get", get().to(get::get))
    );
}