use actix_web::{ web, middleware::from_fn };

use crate::{
    endpoints::user::{ delete_endpoint, get_endpoint, login, update_endpoint, post_endpoint },
    middleware::admin_middleware,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);

    cfg.service(
        web
            ::scope("")
            .wrap(from_fn(admin_middleware))
            .service(get_endpoint)
            .service(post_endpoint)
            .service(update_endpoint)
            .service(delete_endpoint)
    );
}
