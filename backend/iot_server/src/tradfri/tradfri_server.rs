use super::tradfri_messages::GetAllLampsMessage;
use crate::prelude::*;
use crate::request_response_data::request_data::ChangeStatusRequest;
use coap::dtls_client::DTLSCoAPClient;

type ChangeStatusMessage = ChangeStatusRequest;
type ChangeBrightnessMessage = ChangeBrightnessRequest;
type ChangeSceneMessage = ChangeSceneRequest;

pub struct TradfriServer {
    //client: DTLSCoAPClient,
}

impl Actor for TradfriServer {
    type Context = SyncContext<Self>;
}

impl TradfriServer {
    pub fn new() -> Self {
        Self {
            //client: DTLSCoAPClient::new(crate::settings::SERVER_ADDR)
            //    .expect("Error connecting to coap server on tradfri."),
        }
    }
}

impl Handler<GetAllLampsMessage> for TradfriServer {
    type Result = Result<Vec<Lamp>, APIError>;

    fn handle(&mut self, _: GetAllLampsMessage, _: &mut Self::Context) -> Self::Result {
        // TODO retrieve & parse lamps from coap request
        Ok(vec![])
    }
}

impl Handler<ChangeStatusMessage> for TradfriServer {
    type Result = Result<(), APIError>;

    fn handle(&mut self, msg: ChangeStatusMessage, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}

impl Handler<ChangeBrightnessMessage> for TradfriServer {
    type Result = Result<(), APIError>;

    fn handle(&mut self, msg: ChangeBrightnessMessage, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}

impl Handler<ChangeSceneMessage> for TradfriServer {
    type Result = Result<(), APIError>;

    fn handle(&mut self, msg: ChangeSceneMessage, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}
