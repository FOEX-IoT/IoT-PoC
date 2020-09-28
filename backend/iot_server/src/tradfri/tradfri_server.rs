use super::tradfri_messages::GetAllLampsMessage;
use crate::prelude::*;
use crate::request_response_data::request_data::ChangeStatusRequest;
use crate::settings::*;
use coap::dtls_client::DTLSCoAPClient;
use coap::CoAPRequest;
use coap::Method;
use serde_json::Value;
use std::convert::TryFrom;

type ChangeStatusMessage = ChangeStatusRequest;
type ChangeBrightnessMessage = ChangeBrightnessRequest;
type ChangeSceneMessage = ChangeSceneRequest;

pub struct TradfriServer {
    client: DTLSCoAPClient,
}

impl Actor for TradfriServer {
    type Context = SyncContext<Self>;
}

impl TradfriServer {
    pub fn new() -> Self {
        Self {
            client: DTLSCoAPClient::new(SERVER_ADDR, KEY.to_owned(), ID.to_owned())
                .expect("Error connecting to coap server on tradfri."),
        }
    }
}

impl Handler<GetAllLampsMessage> for TradfriServer {
    type Result = Result<Vec<Lamp>, APIError>;

    fn handle(&mut self, _: GetAllLampsMessage, _: &mut Self::Context) -> Self::Result {
        //fetch all the lamps
        let path = "/15001";
        let mut request = CoAPRequest::new();
        request.set_method(Method::Get);
        request.set_path(path);

        let content = execute_and_get_response(&mut self.client, &request)?;
        let content: Value = serde_json::from_str(&content)?;
        let content = content.as_array().ok_or(APIError::InternalServerError)?;

        let content: Vec<&str> = content
            .iter()
            .map(|item| item.as_str().ok_or(APIError::InternalServerError))
            .collect::<Result<Vec<&str>, APIError>>()?;

        // for each lamp, fetch the details
        content
            .into_iter()
            .map(|lamp: &str| {
                let path = format!("/15001/{}", lamp);
                let mut request = CoAPRequest::new();
                request.set_method(Method::Get);
                request.set_path(&path);
                let content = execute_and_get_response(&mut self.client, &request)?;
                let content: Value = serde_json::from_str(&content)?;
                Lamp::try_from(content)
            })
            .collect::<Result<Vec<Lamp>, APIError>>()
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

fn execute_and_get_response(
    client: &mut DTLSCoAPClient,
    request: &CoAPRequest,
) -> Result<String, APIError> {
    client.send(&request)?;

    let response = client.receive()?;

    let response_body = String::from_utf8(response.message.payload)?;

    Ok(response_body)
}
