use actix::prelude::*;
use coap::dtls_client::DTLSCoAPClient;
use crate::settings;

pub struct TradfriServer {
    client: DTLSCoAPClient,
}

impl Actor for TradfriServer {
    type Context = SyncContext<Self>;
}

impl TradfriServer {
    pub fn new() -> Self {
        Self {
            client: DTLSCoAPClient::new(settings::SERVER_ADDR).expect("Error connecting to coap server on tradfri."),
        }
    }
}
