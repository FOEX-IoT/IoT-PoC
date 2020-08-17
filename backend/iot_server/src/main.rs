mod websocket;
mod tradfri;
mod settings;

use actix_web::*;
use crate::websocket::ws_server::BroadcastServer;
use crate::tradfri::TradfriServer;
use crate::websocket::ws_session::WSSession;
use actix::prelude::*;
use actix_web_actors::ws;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        // start servers
        let coap_addr = SyncArbiter::start(12, || TradfriServer::new());
        let bc_addr = BroadcastServer::default().start();

        App::new()
            .data(coap_addr)
            .data(bc_addr)
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(index)))
            // static files
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    bc_srv: web::Data<Addr<BroadcastServer>>,
    coap_srv: web::Data<Addr<TradfriServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WSSession {
            id: 0,
            addr: bc_srv.get_ref().clone(),
            coap_addr: coap_srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
