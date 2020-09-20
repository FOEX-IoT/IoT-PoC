use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Copy)]
pub struct ChangeStatusRequest {
    #[serde(rename = "instanceId")]
    pub instance_id: i32,
    pub status: bool,
}

impl Message for ChangeStatusRequest {
    type Result = Result<(), APIError>;
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Copy)]
pub struct ChangeBrightnessRequest {
    #[serde(rename = "instanceId")]
    pub instance_id: i32,
    pub brightness: u8,
}

impl Message for ChangeBrightnessRequest {
    type Result = Result<(), APIError>;
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ChangeSceneRequest {
    pub scene: Scene,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Scene {
    EVERYDAY,
    FOCUS,
    RELAX,
    CUSTOM,
}

impl Message for ChangeSceneRequest {
    type Result = Result<(), APIError>;
}
