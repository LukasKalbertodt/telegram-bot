use std::fmt;
use std::ops::Deref;

use serde::ser::{Serialize, Serializer};

use crate::types::*;

macro_rules! integer_id_impls {
    ($name: ident) => {
        impl $name {
            pub fn new(inner: Integer) -> Self {
                $name(inner)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Integer> for $name {
            fn from(inner: Integer) -> Self {
                $name::new(inner)
            }
        }

        impl From<$name> for Integer {
            fn from(from: $name) -> Self {
                from.0
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let inner = ::serde::de::Deserialize::deserialize(deserializer)?;
                Ok($name::new(inner))
            }
        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_i64(self.0)
            }
        }
    };
}

/// Get source `ChatId` from the type reference.
pub trait ToSourceChat {
    fn to_source_chat(&self) -> ChatId;
}

impl<S> ToSourceChat for S
where
    S: Deref,
    S::Target: ToSourceChat,
{
    fn to_source_chat(&self) -> ChatId {
        self.deref().to_source_chat()
    }
}

impl ToSourceChat for Message {
    fn to_source_chat(&self) -> ChatId {
        self.chat.id()
    }
}

impl ToSourceChat for ChannelPost {
    fn to_source_chat(&self) -> ChatId {
        self.chat.id.into()
    }
}

impl ToSourceChat for MessageOrChannelPost {
    fn to_source_chat(&self) -> ChatId {
        match self {
            MessageOrChannelPost::Message(ref message) => message.to_source_chat(),
            MessageOrChannelPost::ChannelPost(ref channel_post) => channel_post.to_source_chat(),
        }
    }
}

/// Unique identifier for the target chat or username of the
/// target channel (in the format @channelusername)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChatRef {
    Id(ChatId),
    #[doc(hidden)]
    ChannelUsername(String),
}

impl ChatRef {
    pub fn from_chat_id(chat_id: ChatId) -> ChatRef {
        ChatRef::Id(chat_id)
    }
}

/// Get `ChatRef` from the type reference.
pub trait ToChatRef {
    fn to_chat_ref(&self) -> ChatRef;
}

impl<S> ToChatRef for S
where
    S: Deref,
    S::Target: ToChatRef,
{
    fn to_chat_ref(&self) -> ChatRef {
        self.deref().to_chat_ref()
    }
}

impl ToChatRef for ChatRef {
    fn to_chat_ref(&self) -> ChatRef {
        self.clone()
    }
}

impl ToChatRef for Chat {
    fn to_chat_ref(&self) -> ChatRef {
        self.id().to_chat_ref()
    }
}

impl ToChatRef for MessageChat {
    fn to_chat_ref(&self) -> ChatRef {
        self.id().to_chat_ref()
    }
}

impl ToChatRef for ChatMember {
    fn to_chat_ref(&self) -> ChatRef {
        self.user.to_chat_ref()
    }
}

impl ToChatRef for ForwardFrom {
    fn to_chat_ref(&self) -> ChatRef {
        match *self {
            ForwardFrom::User { ref user, .. } => user.to_chat_ref(),
            ForwardFrom::Channel { ref channel, .. } => channel.to_chat_ref(),
            ForwardFrom::ChannelHiddenUser { ref sender_name } => {
                ChatRef::ChannelUsername(sender_name.clone())
            }
        }
    }
}

impl ToChatRef for Forward {
    fn to_chat_ref(&self) -> ChatRef {
        self.from.to_chat_ref()
    }
}

impl Serialize for ChatRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ChatRef::Id(id) => serializer.serialize_i64(id.into()),
            ChatRef::ChannelUsername(ref username) => serializer.serialize_str(&username),
        }
    }
}

impl fmt::Display for ChatRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChatRef::Id(id) => write!(f, "{}", id),
            ChatRef::ChannelUsername(ref username) => write!(f, "{}", username),
        }
    }
}

macro_rules! chat_id_impls {
    ($id: ident) => {
        integer_id_impls!($id);

        impl ToChatRef for $id {
            fn to_chat_ref(&self) -> ChatRef {
                ChatRef::from_chat_id((*self).into())
            }
        }
    };
}

macro_rules! specific_chat_id_impls {
    ($id: ident, $typ: ident) => {
        chat_id_impls!($id);

        impl From<$id> for ChatId {
            fn from(c: $id) -> Self {
                ChatId::new(c.into())
            }
        }

        impl ToChatRef for $typ {
            fn to_chat_ref(&self) -> ChatRef {
                self.id.to_chat_ref()
            }
        }
    };
}

/// Get `UserId` from the type reference.
pub trait ToUserId {
    fn to_user_id(&self) -> UserId;
}

impl<S> ToUserId for S
where
    S: Deref,
    S::Target: ToUserId,
{
    fn to_user_id(&self) -> UserId {
        self.deref().to_user_id()
    }
}

impl ToUserId for UserId {
    fn to_user_id(&self) -> UserId {
        *self
    }
}

impl ToUserId for ChatMember {
    fn to_user_id(&self) -> UserId {
        self.user.id
    }
}

impl ToUserId for User {
    fn to_user_id(&self) -> UserId {
        self.id
    }
}

/// Unique user identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(Integer);
specific_chat_id_impls!(UserId, User);

/// Unique group identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupId(Integer);
specific_chat_id_impls!(GroupId, Group);

/// Unique supergroup identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupergroupId(Integer);
specific_chat_id_impls!(SupergroupId, Supergroup);

/// Unique channel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChannelId(Integer);
specific_chat_id_impls!(ChannelId, Channel);

/// Unique chat identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatId(Integer);
chat_id_impls!(ChatId);

/// Get `MessageId` from the type reference.
pub trait ToMessageId {
    fn to_message_id(&self) -> MessageId;
}

impl<S> ToMessageId for S
where
    S: Deref,
    S::Target: ToMessageId,
{
    fn to_message_id(&self) -> MessageId {
        self.deref().to_message_id()
    }
}

impl ToMessageId for MessageId {
    fn to_message_id(&self) -> MessageId {
        *self
    }
}

impl ToMessageId for Message {
    fn to_message_id(&self) -> MessageId {
        self.id
    }
}

impl ToMessageId for ChannelPost {
    fn to_message_id(&self) -> MessageId {
        self.id
    }
}

impl ToMessageId for MessageOrChannelPost {
    fn to_message_id(&self) -> MessageId {
        match self {
            MessageOrChannelPost::Message(ref message) => message.to_message_id(),
            MessageOrChannelPost::ChannelPost(ref channel_post) => channel_post.to_message_id(),
        }
    }
}

/// Unique message identifier inside a chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(Integer);
integer_id_impls!(MessageId);

/// Get `FileRef` from the type reference.
pub trait ToFileRef {
    fn to_file_ref(&self) -> FileRef;
}

impl<S> ToFileRef for S
where
    S: Deref,
    S::Target: ToFileRef,
{
    fn to_file_ref(&self) -> FileRef {
        self.deref().to_file_ref()
    }
}

macro_rules! file_id_impls {
    ($name: ident) => {
        impl ToFileRef for $name {
            fn to_file_ref(&self) -> FileRef {
                self.file_id.clone().into()
            }
        }
    };
}

file_id_impls!(PhotoSize);
file_id_impls!(Animation);
file_id_impls!(Audio);
file_id_impls!(Document);
file_id_impls!(Sticker);
file_id_impls!(Video);
file_id_impls!(Voice);
file_id_impls!(VideoNote);

/// Unique file identifier reference.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileRef {
    pub(crate) inner: String,
}

impl<'a> From<&'a str> for FileRef {
    fn from(s: &'a str) -> Self {
        FileRef {
            inner: s.to_string(),
        }
    }
}

impl<'a> From<String> for FileRef {
    fn from(s: String) -> Self {
        FileRef { inner: s }
    }
}

impl Serialize for FileRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.inner)
    }
}

/// Get `CallbackQueryId` from the type reference.
pub trait ToCallbackQueryId {
    fn to_callback_query_id(&self) -> CallbackQueryId;
}

impl<S> ToCallbackQueryId for S
where
    S: Deref,
    S::Target: ToCallbackQueryId,
{
    fn to_callback_query_id(&self) -> CallbackQueryId {
        self.deref().to_callback_query_id()
    }
}

impl ToCallbackQueryId for CallbackQuery {
    fn to_callback_query_id(&self) -> CallbackQueryId {
        self.id.clone()
    }
}

/// Unique identifier for CallbackQuery.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CallbackQueryId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct InlineQueryId(String);
