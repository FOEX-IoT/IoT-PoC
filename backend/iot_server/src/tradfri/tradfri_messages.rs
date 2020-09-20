use crate::prelude::*;

pub struct GetAllLampsMessage;

impl Message for GetAllLampsMessage {
    type Result = Result<Vec<Lamp>, APIError>;
}
