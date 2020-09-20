use actix::prelude::*;

/// The message which is being sent to a session.
#[derive(Message)]
#[rtype(result = "()")]
pub struct BCMessage(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<BCMessage>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
