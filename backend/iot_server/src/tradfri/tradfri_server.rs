use super::tradfri_messages::GetAllLampsMessage;
use crate::prelude::*;
use crate::request_response_data::request_data::ChangeStatusRequest;
use crate::settings::*;
use coap::dtls_client::DTLSCoAPClient;
use coap::message::IsMessage;
use coap::CoAPRequest;
use coap::Method;
use serde_json::json;
use serde_json::Value;
use std::convert::TryFrom;

type ChangeStatusMessage = ChangeStatusRequest;
type ChangeBrightnessMessage = ChangeBrightnessRequest;
type ChangeSceneMessage = ChangeSceneRequest;

pub struct TradfriServer {
  client: DTLSCoAPClient,
  lamp_ids: Vec<i64>,
}

impl Actor for TradfriServer {
  type Context = SyncContext<Self>;
}

impl TradfriServer {
  pub fn new() -> Self {
    Self {
      client: DTLSCoAPClient::new(SERVER_ADDR)
        .expect("Error connecting to coap server on tradfri."),
      lamp_ids: vec![],
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
    let content: Vec<i64> = content
      .iter()
      .map(|item| item.as_i64().ok_or(APIError::InternalServerError))
      .collect::<Result<Vec<i64>, APIError>>()?;
    // set own lamps
    self.lamp_ids.clear();

    content.iter().for_each(|lamp: &i64| {
      self.lamp_ids.push(*lamp);
    });

    // for each lamp, fetch the details
    content
      .into_iter()
      .filter_map(|lamp: i64| {
        let path = format!("/15001/{}", lamp);
        let mut request = CoAPRequest::new();
        request.set_method(Method::Get);
        request.set_path(&path);
        let content = execute_and_get_response(&mut self.client, &request).ok()?;
        let content: Value = serde_json::from_str(&content).ok()?;
        match Lamp::try_from(content) {
          Ok(lamp) => Some(Ok(lamp)),
          Err(_) => None,
        }
      })
      .collect::<Result<Vec<Lamp>, APIError>>()
  }
}

impl Handler<ChangeStatusMessage> for TradfriServer {
  type Result = Result<(), APIError>;

  fn handle(&mut self, msg: ChangeStatusMessage, _: &mut Self::Context) -> Self::Result {
    let path = format!("/15001/{}", msg.instance_id);

    let payload = json!({
        "3311" : [
            {
                "5850": if msg.status {1} else { 0 }
            }
        ]
    });

    let payload = serde_json::to_string(&payload)?;

    let mut request = CoAPRequest::new();
    request.set_path(&path);
    request.set_payload(payload.as_bytes().to_vec());
    request.set_method(Method::Put);

    self.client.send(&request)?;

    Ok(())
  }
}

impl Handler<ChangeBrightnessMessage> for TradfriServer {
  type Result = Result<(), APIError>;

  fn handle(&mut self, msg: ChangeBrightnessMessage, _: &mut Self::Context) -> Self::Result {
    let path = format!("/15001/{}", msg.instance_id);

    let payload = json!({
        "3311" : [
            {
                "5851": msg.brightness
            }
        ]
    });

    let payload = serde_json::to_string(&payload)?;

    let mut request = CoAPRequest::new();
    request.set_path(&path);
    request.set_payload(payload.as_bytes().to_vec());
    request.set_method(Method::Put);

    self.client.send(&request)?;

    Ok(())
  }
}

impl Handler<ChangeSceneMessage> for TradfriServer {
  type Result = Result<(), APIError>;

  fn handle(&mut self, msg: ChangeSceneMessage, _: &mut Self::Context) -> Self::Result {
    // In case it's a custom scene, there are no changes on the backend
    if msg.scene == Scene::CUSTOM {
      return Ok(());
    };

    let ChangeSceneMessage { scene } = msg;

    let brightness_val: i32 = scene.into();

    for lamp in self.lamp_ids.iter() {
      let path = format!("/15001/{}", lamp);

      // switch all lamps on
      let payload = json!({
          "3311" : [
              {
                  "5850": 1
              }
          ]
      });

      let payload = serde_json::to_string(&payload)?;

      let mut request = CoAPRequest::new();

      request.set_path(&path);
      request.set_payload(payload.as_bytes().to_vec());
      request.set_method(Method::Put);

      self.client.send(&request)?;

      // update the brightness
      let payload = json!({
          "3311" : [
              {
                  "5851": brightness_val
              }
          ]
      });

      let payload = serde_json::to_string(&payload)?;

      request.set_path(&path);
      request.set_payload(payload.as_bytes().to_vec());
      request.set_method(Method::Put);

      self.client.send(&request)?;
    }

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
