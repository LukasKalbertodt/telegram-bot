use crate::requests::*;
use crate::types::*;

/// Use this method to edit live location messages sent by the bot.
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageLiveLocation {
    chat_id: ChatRef,
    message_id: MessageId,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for EditMessageLiveLocation {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("editMessageLiveLocation"), self)
    }
}

impl EditMessageLiveLocation {
    pub fn new<C, M>(chat: C, message_id: M, latitude: Float, longitude: Float) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Edit live location messages sent by the bot.
pub trait CanEditMessageLiveLocation {
    fn edit_live_location(&self, latitude: Float, longitude: Float) -> EditMessageLiveLocation;
}

impl<M> CanEditMessageLiveLocation for M
where
    M: ToMessageId + ToSourceChat,
{
    fn edit_live_location(&self, latitude: Float, longitude: Float) -> EditMessageLiveLocation {
        EditMessageLiveLocation::new(
            self.to_source_chat(),
            self.to_message_id(),
            latitude,
            longitude,
        )
    }
}
