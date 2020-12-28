use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send an animation
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendAnimation<'c> {
    chat_id: ChatRef,
    animation: InputFile, // TODO: add "or String" option
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    duration: Option<Integer>,
    width: Option<Integer>,
    height: Option<Integer>,
    thumb: Option<InputFile>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendAnimation<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (animation (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (duration (text), optional);
            (width (text), optional);
            (height (text), optional);
            (thumb (raw), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendAnimation<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendAnimation"), self)
    }
}

impl<'c> SendAnimation<'c> {
    pub fn new<C, A>(chat: C, animation: A) -> Self
    where
        C: ToChatRef,
        A: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            animation: animation.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn thumb<A>(&mut self, thumb: A) -> &mut Self
    where
        A: Into<InputFileUpload>,
    {
        self.thumb = Some(thumb.into().into());
        self
    }

    pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
        T: Into<Cow<'c, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn width(&mut self, width: Integer) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn height(&mut self, height: Integer) -> &mut Self {
        self.height = Some(height);
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

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}

/// Can reply with an animation
pub trait CanReplySendAnimation {
    fn animation_reply<'c, T>(&self, animation: T) -> SendAnimation<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanReplySendAnimation for M
where
    M: ToMessageId + ToSourceChat,
{
    fn animation_reply<'c, T>(&self, animation: T) -> SendAnimation<'c>
    where
        T: Into<InputFile>,
    {
        let mut req = SendAnimation::new(self.to_source_chat(), animation);
        req.reply_to(self);
        req
    }
}

/// Send an animation
pub trait CanSendAnimation {
    fn animation<'c, T>(&self, animation: T) -> SendAnimation<'c>
    where
        T: Into<InputFile>;
}

impl<M> CanSendAnimation for M
where
    M: ToChatRef,
{
    fn animation<'c, T>(&self, animation: T) -> SendAnimation<'c>
    where
        T: Into<InputFile>,
    {
        SendAnimation::new(self.to_chat_ref(), animation)
    }
}
