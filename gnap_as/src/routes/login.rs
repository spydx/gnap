use actix_web::web;
use actix_web_lab::web::spa;
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login").service(
            spa()
                .index_file("./static/login.html")
                .static_resources_location("./static/")
                .finish(),
        ),
    );
}
