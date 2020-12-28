use std::borrow::Cow;
use std::ops::Not;

use crate::requests::*;
use crate::types::*;

/// Use this method to send information about a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendVenue<'t, 'a, 'f> {
    chat_id: ChatRef,
    latitude: Float,
    longitude: Float,
    title: Cow<'t, str>,
    address: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<Cow<'f, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'t, 'a, 'f> Request for SendVenue<'t, 'a, 'f> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendVenue"), self)
    }
}

impl<'t, 'a, 'f> SendVenue<'t, 'a, 'f> {
    pub fn new<C, T, A>(chat: C, latitude: Float, longitude: Float, title: T, address: A) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'t, str>>,
        A: Into<Cow<'a, str>>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            disable_notification: false,
            foursquare_id: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn foursquare_id<F>(&mut self, id: F) -> &mut Self
    where
        F: Into<Cow<'f, str>>,
    {
        self.foursquare_id = Some(id.into());
        self
    }

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self
    where
        R: ToMessageId,
    {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Send information about a venue.
pub trait CanSendVenue<'t, 'a, 'f> {
    fn venue<T, A>(
        &self,
        latitude: Float,
        longitude: Float,
        title: T,
        address: A,
    ) -> SendVenue<'t, 'a, 'f>
    where
        T: Into<Cow<'t, str>>,
        A: Into<Cow<'a, str>>;
}

impl<'t, 'a, 'f, C> CanSendVenue<'t, 'a, 'f> for C
where
    C: ToChatRef,
{
    fn venue<T, A>(
        &self,
        latitude: Float,
        longitude: Float,
        title: T,
        address: A,
    ) -> SendVenue<'t, 'a, 'f>
    where
        T: Into<Cow<'t, str>>,
        A: Into<Cow<'a, str>>,
    {
        SendVenue::new(self, latitude, longitude, title, address)
    }
}

/// Reply with information about a venue.
pub trait CanReplySendVenue {
    fn venue_reply<'t, 'a, 'f, T, A>(
        &self,
        latitude: Float,
        longitude: Float,
        title: T,
        address: A,
    ) -> SendVenue<'t, 'a, 'f>
    where
        T: Into<Cow<'t, str>>,
        A: Into<Cow<'a, str>>;
}

impl<M> CanReplySendVenue for M
where
    M: ToMessageId + ToSourceChat,
{
    fn venue_reply<'t, 'a, 'f, T, A>(
        &self,
        latitude: Float,
        longitude: Float,
        title: T,
        address: A,
    ) -> SendVenue<'t, 'a, 'f>
    where
        T: Into<Cow<'t, str>>,
        A: Into<Cow<'a, str>>,
    {
        let mut rq = self
            .to_source_chat()
            .venue(latitude, longitude, title, address);
        rq.reply_to(self.to_message_id());
        rq
    }
}

impl<'b> ToRequest<'b> for Venue {
    type Request = SendVenue<'b, 'b, 'b>;

    fn to_request<C>(&'b self, chat: C) -> Self::Request
    where
        C: ToChatRef,
    {
        let mut rq = chat.venue(
            self.location.latitude,
            self.location.longitude,
            self.title.as_str(),
            self.address.as_str(),
        );
        if let Some(ref foursquare_id) = self.foursquare_id {
            rq.foursquare_id(foursquare_id.as_str());
        }
        rq
    }
}

impl<'b> ToReplyRequest<'b> for Venue {
    type Request = SendVenue<'b, 'b, 'b>;

    fn to_reply_request<M>(&'b self, message: M) -> Self::Request
    where
        M: ToMessageId + ToSourceChat,
    {
        let mut rq = message.venue_reply(
            self.location.latitude,
            self.location.longitude,
            self.title.as_str(),
            self.address.as_str(),
        );
        if let Some(ref foursquare_id) = self.foursquare_id {
            rq.foursquare_id(foursquare_id.as_str());
        }
        rq
    }
}
