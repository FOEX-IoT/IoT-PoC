mod error;
mod error_macro;
mod handlers;
mod models;
mod prelude;
mod request_response_data;
mod settings;
mod tradfri;
mod urls;
mod websocket;

use crate::tradfri::TradfriServer;
use crate::urls::url_config;
use crate::websocket::ws_server::BroadcastServer;
use crate::websocket::ws_session::WSSession;
use actix::prelude::*;
use actix_web::*;
use actix_web_actors::ws;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let coap_addr = web::Data::new(SyncArbiter::start(12, || TradfriServer::new()));
    let bc_addr = web::Data::new(BroadcastServer::default().start());

    HttpServer::new(move || {
        // start servers
        App::new()
            .app_data(coap_addr.clone())
            .app_data(bc_addr.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .finish(),
            )
            // websocket route
            .service(web::scope("/api").configure(url_config))
            .service(web::resource("/ws/").route(web::get().to(index)))
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

//fn main() {
//  let input = r#"{"9001":"TRADFRI remote control","9002":1583327787,"9020":1597443597,"9003":65536,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI remote control","2":"","3":"2.3.014","6":3,"9":34},"15009":[{"9003":0}]}"#;
//  let lamp: crate::models::Lamp = serde_json::from_str(input).unwrap();
//}
