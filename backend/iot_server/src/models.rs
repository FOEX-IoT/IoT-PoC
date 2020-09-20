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
    #[serde(rename = "9001")]
    pub name: String,
    #[serde(rename = "9003")]
    pub instance_id: i32,
    #[serde(rename = "5750")]
    pub is_on: i32,
    #[serde(rename = "5851")]
    pub brightness: u8,
}

impl TryFrom<Value> for Lamp {
    type Error = APIError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        //value.get("9001").ok_or(APIError::InternalServerError)
        //let name = value.get("9001").ok_or(APIError::InternalServerError);
        todo!()
    }
}
