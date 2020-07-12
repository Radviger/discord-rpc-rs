mod shared;
pub mod message;
pub mod payload;
pub mod commands;
pub mod events;
pub mod rich_presence;


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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Handshake {
    pub v: u32,
    pub config: Config,
    pub user: User
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Config {
    cdn_host: String,
    api_endpoint: String,
    environment: String
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: bool,
    flags: u32,
    premium_type: u32
}

pub use self::message::{Message, OpCode};
pub use self::commands::*;
pub use self::events::*;

#[cfg(feature = "rich_presence")]
pub use self::rich_presence::*;

pub mod prelude {
    pub use super::Command;
    pub use super::Event;
    #[cfg(feature = "rich_presence")]
    pub use super::rich_presence::{
        SetActivityArgs,
        SendActivityJoinInviteArgs,
        CloseActivityRequestArgs,
        ActivityJoinEvent,
        ActivitySpectateEvent,
        ActivityJoinRequestEvent
    };
    pub use super::commands::{
        SubscriptionArgs, Subscription
    };
    pub use super::events::{
        ReadyEvent,
        ErrorEvent,
    };
}
