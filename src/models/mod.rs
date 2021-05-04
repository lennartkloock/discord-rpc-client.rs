pub use self::commands::*;
pub use self::events::*;
pub use self::message::{Message, OpCode};
#[cfg(feature = "rich_presence")]
pub use self::rich_presence::*;

pub mod commands;
pub mod events;
pub mod message;
pub mod payload;
pub mod rich_presence;
mod shared;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Dispatch,
    Authorize,
    Subscribe,
    Unsubscribe,
    #[cfg(feature = "rich_presence")]
    SetActivity,
    #[cfg(feature = "rich_presence")]
    SendActivityJoinInvite,
    #[cfg(feature = "rich_presence")]
    CloseActivityRequest,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    Ready,
    Error,
    #[cfg(feature = "rich_presence")]
    ActivityJoin,
    #[cfg(feature = "rich_presence")]
    ActivitySpectate,
    #[cfg(feature = "rich_presence")]
    ActivityJoinRequest,
}

pub mod prelude {
    pub use super::Command;
    pub use super::commands::{Subscription, SubscriptionArgs};
    pub use super::Event;
    pub use super::events::{ErrorEvent, ReadyEvent};
    #[cfg(feature = "rich_presence")]
    pub use super::rich_presence::{
        ActivityJoinEvent, ActivityJoinRequestEvent, ActivitySpectateEvent,
        CloseActivityRequestArgs, SendActivityJoinInviteArgs, SetActivityArgs,
    };
}
