mod error;
mod error_macro;
mod models;
mod settings;
mod tradfri;
mod websocket;

use crate::tradfri::TradfriServer;
use crate::websocket::ws_server::BroadcastServer;
use crate::websocket::ws_session::WSSession;
use actix::prelude::*;
use actix_web::*;
use actix_web_actors::ws;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//   std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
//   env_logger::init();

//   HttpServer::new(|| {
//     // start servers
//     let coap_addr = SyncArbiter::start(12, || TradfriServer::new());
//     let bc_addr = BroadcastServer::default().start();

//     App::new()
//       .data(coap_addr)
//       .data(bc_addr)
//       // enable logger
//       .wrap(middleware::Logger::default())
//       // websocket route
//       .service(web::resource("/ws/").route(web::get().to(index)))
//     // static files
//   })
//   // start http server on 127.0.0.1:8080
//   .bind("127.0.0.1:8080")?
//   .run()
//   .await
// }

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

fn main() {
  let input = r#"{"9001":"TRADFRI remote control","9002":1583327787,"9020":1597443597,"9003":65536,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI remote control","2":"","3":"2.3.014","6":3,"9":34},"15009":[{"9003":0}]}"#;
  let lamp: crate::models::Lamp = serde_json::from_str(input).unwrap();
}
