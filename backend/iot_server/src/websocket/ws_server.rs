use crate::websocket::messages::BCMessage;
use crate::websocket::messages::*;
use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashMap;

/// This server is used to notify all the connected
/// clients of any changes
pub struct BroadcastServer {
    sessions: HashMap<usize, Recipient<BCMessage>>,
    rng: ThreadRng,
}

impl Default for BroadcastServer {
    fn default() -> BroadcastServer {
        BroadcastServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl BroadcastServer {
    /// Send message to all users
    fn send_message(&self, message: &str) {
        self.sessions.iter().for_each(|(_, addr)| {
            let _ = addr.do_send(BCMessage(message.to_owned()));
        });
    }
}

impl Actor for BroadcastServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for BroadcastServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        // return id to session
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for BroadcastServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        // remove address
        self.sessions.remove(&msg.id);
    }
}

/// Handle messages which are received from the server (i.e. updates)
impl Handler<BCMessage> for BroadcastServer {
    type Result = ();

    fn handle(&mut self, msg: BCMessage, ctx: &mut Self::Context) {
        self.send_message(&msg.0);
    }
}
