use crate::types::*;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Kind of the incoming update.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(rename = "message")]
    Message(Message),
    /// New version of a message that is known to the bot and was edited
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(rename = "channel_post")]
    ChannelPost(ChannelPost),
    /// New version of a channel post that is known to the bot and was edited
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost(ChannelPost),
    #[serde(rename = "inline_query")]
    InlineQuery(InlineQuery),
    //    ChosenInlineResult(ChosenInlineResult),
    #[serde(rename = "callback_query")]
    CallbackQuery(Box<CallbackQuery>),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(rename = "poll")]
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself
    #[serde(rename = "poll_answer")]
    PollAnswer(PollAnswer),
    #[doc(hidden)]
    Error(String),
    #[doc(hidden)]
    Unknown,
}
