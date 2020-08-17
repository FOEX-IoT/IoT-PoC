mod websocket;
mod tradfri;
mod settings;

use actix_web::{middleware, App, HttpServer};
use crate::tradfri::TradfriServer;
use actix::prelude::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let addr = SyncArbiter::start(12, || TradfriServer::new());
        App::new()
            .data(addr)
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            //.service(web::resource("/ws/").route(web::get().to(ws_index)))
            // static files
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
