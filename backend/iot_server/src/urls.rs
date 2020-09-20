use crate::handlers::*;
use actix_web::web::{self, *};

pub fn url_config(config: &mut ServiceConfig) {
    lamp_config(config);
}

fn lamp_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/lamps")
            .route("/all", web::get().to(get_all_lamps))
            .route("/update-status", web::post().to(change_status_of_lamp))
            .route(
                "/update-brightness",
                web::post().to(change_brightness_of_lamp),
            )
            .route("/update-scene", web::post().to(change_scene)),
    );
}
