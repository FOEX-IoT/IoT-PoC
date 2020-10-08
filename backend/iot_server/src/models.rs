/*
export type Status = 'on' | 'off';
export interface Lamp {
   id: number;
   name: string;
   brightness: number;
   status: Status;
}
*/

/*
get all devices
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15001"
[65540,65536,65539]
*/

/*
get info about a specific device
export TF_DEVICEID=65540
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15001/$TF_DEVICEID"
{"9001":"","9002":1597443583,"9020":104,"9003":65540,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI bulb E27 WW 806lm","2":"","3":"2.1.022","6":1},"15009":[{"5850":1,"9003":0,"5851":25}]}
{"9001":"","9002":1597443583,"9020":104,"9003":65540,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI bulb E27 WW 806lm","2":"","3":"2.1.022","6":1},"15009":[{"5850":1,"9003":0,"5851":25}]}
*/

/*
get all groups
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15004"
[131073]
*/

/*
get info about a specific group
export TF_GROUPID=131073
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15004/$TF_GROUPID"
{"9001":"TRADFRI group","9039":196608,"5850":0,"9002":1583327798,"5851":0,"9003":131073,"9018":{"15002":{"9003":[65536,65539,65540]}}}
*/

/*
get a list of all scenes
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15005/$TF_GROUPID"
[196608,196609,196610]
*/

/*
get info about a specific scene
coap-client -m get -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" "coaps://$TF_GATEWAYIP:5684/15005/$TF_GROUPID/$TF_SCENEID"
{"9001":"EVERYDAY","9002":1583327798,"9003":196608,"9057":0,"9068":1}
*/

/*
change the brightness of a device
coap-client -m put -u "$TF_USERNAME" -k "$TF_PRESHARED_KEY" -e '{ "3311": [{ "5851": 127 }] }' "coaps://$TF_GATEWAYIP:5684/15001/$TF_DEVICEID"

here basically everything a payload can contain:
{
  "3311": [
    {
      "5850": 1, // on / off
      "5851": 254, // dimmer (1 to 254)
      "5712": 10 // transition time (fade time)
      // THIS IS USELESS FOR US
      "5706": "f1e0b5", // color in HEX (Don't use in combination with: color X and/or color Y)
      "5709": 65535, // color X (Only use in combination with color Y)
      "5710": 65535, // color Y (Only use in combination with color X)
    }
  ]
}

you just put the json into the -e field
*/

/*
3311: is a lamp

9001: Name
9003: instance id

5850: on/off
5851: brightness
*/

use crate::error::APIError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Lamp {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "instanceId")]
  pub instance_id: i64,
  #[serde(rename = "status")]
  pub status: bool,
  #[serde(rename = "brightness")]
  pub brightness: u8,
}

impl TryFrom<Value> for Lamp {
  type Error = APIError;

  fn try_from(value: Value) -> Result<Self, Self::Error> {
    //{"9001":"","9002":1597443583,"9020":104,"9003":65540,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI bulb E27 WW 806lm","2":"","3":"2.1.022","6":1},"15009":[{"5850":1,"9003":0,"5851":25}]}
    println!("parsing {:?}!", value);
    let name: String = value
      .get("3")
      .ok_or(APIError::InternalServerError)?
      .get("1")
      .ok_or(APIError::InternalServerError)?
      .as_str()
      .ok_or(APIError::InternalServerError)?
      .to_owned();

    println!("parsed name!");
    let sub: &Value = value
      .get("15009")
      .ok_or(APIError::InternalServerError)?
      .as_array()
      .ok_or(APIError::InternalServerError)?
      .get(0)
      .ok_or(APIError::InternalServerError)?;

    println!("parsed sub!");
    let instance_id: i64 = sub
      .get("9003")
      .ok_or(APIError::InternalServerError)?
      .as_i64()
      .ok_or(APIError::InternalServerError)?;

    println!("parsed instance id!");
    let status: bool = {
      sub
        .get("5850")
        .ok_or(APIError::InternalServerError)?
        .as_i64()
        .ok_or(APIError::InternalServerError)?
        == 1
    };

    println!("parsed status!");
    let brightness: i64 = sub
      .get("5851")
      .ok_or(APIError::InternalServerError)?
      .as_i64()
      .ok_or(APIError::InternalServerError)?;
    println!("parsed brightness!");
    Ok(Lamp {
      name,
      instance_id,
      status,
      brightness: brightness as u8,
    })
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_lamp() {
    use super::Lamp;
    use serde_json::Value;
    use std::convert::TryFrom;

    let input = r#"{"9001":"","9002":1597443583,"9020":104,"9003":65540,"9054":0,"5750":0,"9019":1,"3":{"0":"IKEA of Sweden","1":"TRADFRI bulb E27 WW 806lm","2":"","3":"2.1.022","6":1},"15009":[{"5850":1,"9003":0,"5851":25}]}"#;

    let value: Value = serde_json::from_str(input).unwrap();

    let lamp: Lamp = Lamp::try_from(value).unwrap();

    assert_eq!(
      Lamp {
        name: "TRADFRI bulb E27 WW 806lm".to_owned(),
        instance_id: 0,
        status: true,
        brightness: 25
      },
      lamp
    );
  }
}
