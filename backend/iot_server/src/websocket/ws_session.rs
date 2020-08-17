use crate::websocket::messages::Message as ServerMessage;
use actix::*;
use actix_web_actors::ws;

use crate::websocket::messages::*;
use crate::websocket::ws_server::BroadcastServer;

/// This is the individual session which is created
/// for each client that connects to the url.
pub struct WSSession {
    pub id: usize,
    pub addr: Addr<BroadcastServer>,
}

impl Actor for WSSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages which are received from the server (i.e. updates)
impl Handler<ServerMessage> for WSSession {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(_) => {}
            ws::Message::Pong(_) => {}
            ws::Message::Text(text) => {
                let m = text.trim();
                self.handle_text_message(m);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WSSession {
    /// This function is invoked once a client sends a message (i.e. a command)
    fn handle_text_message(&mut self, message: &str) {
        // TODO maybe send a message to the broadcast server / to the coap server
    }
}
