use serde::{de::DeserializeOwned, Serialize};
#[allow(unused)]
use serde_json::Value;

use connection::Manager as ConnectionManager;
use error::{Error, Result};
use models::{
    Command,
    commands::{Subscription, SubscriptionArgs},
    Event,
    message::Message, OpCode, payload::Payload,
};
#[cfg(feature = "rich_presence")]
use models::rich_presence::{
    Activity, CloseActivityRequestArgs, SendActivityJoinInviteArgs, SetActivityArgs,
};

#[derive(Clone)]
pub struct Client {
    connection_manager: ConnectionManager,
}

impl Client {
    pub fn new(client_id: u64) -> Self {
        let connection_manager = ConnectionManager::new(client_id);
        Self { connection_manager }
    }

    pub fn start(&mut self, retries: u32) {
        self.connection_manager.start(retries);
    }

    fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
        where
            A: Serialize + Send + Sync,
            E: Serialize + DeserializeOwned + Send + Sync,
    {
        let message = Message::new(
            OpCode::Frame,
            Payload::with_nonce(cmd, Some(args), None, evt),
        );
        self.connection_manager.send(message)?;
        let Message { payload, .. } = self.connection_manager.recv()?;
        let response: Payload<E> = serde_json::from_str(&payload)?;

        match response.evt {
            Some(Event::Error) => Err(Error::SubscriptionFailed),
            _ => Ok(response),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connection_manager.is_connected()
    }

    #[cfg(feature = "rich_presence")]
    pub fn set_activity(&mut self, activity: Activity) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::new(activity), None)
    }

    #[cfg(feature = "rich_presence")]
    pub fn clear_activity(&mut self) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::default(), None)
    }

    // NOTE: Not sure what the actual response values of
    //       SEND_ACTIVITY_JOIN_INVITE and CLOSE_ACTIVITY_REQUEST are,
    //       they are not documented.
    #[cfg(feature = "rich_presence")]
    pub fn send_activity_join_invite(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::SendActivityJoinInvite,
            SendActivityJoinInviteArgs::new(user_id),
            None,
        )
    }

    #[cfg(feature = "rich_presence")]
    pub fn close_activity_request(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::CloseActivityRequest,
            CloseActivityRequestArgs::new(user_id),
            None,
        )
    }

    pub fn subscribe(
        &mut self,
        evt: Event,
        args: SubscriptionArgs,
    ) -> Result<Payload<Subscription>> {
        self.execute(Command::Subscribe, args, Some(evt))
    }

    pub fn unsubscribe(
        &mut self,
        evt: Event,
        args: SubscriptionArgs,
    ) -> Result<Payload<Subscription>> {
        self.execute(Command::Unsubscribe, args, Some(evt))
    }
}
