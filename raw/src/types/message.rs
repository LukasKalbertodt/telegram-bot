use serde::de::{Deserialize, Deserializer, Error};

use crate::types::*;
use crate::url::*;

/// This object represents a chat message or a channel post.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageOrChannelPost {
    Message(Message),
    ChannelPost(ChannelPost),
}

/// This object represents a chat message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub id: MessageId,
    /// Sender, can be empty for messages sent to channels.
    pub from: User,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: MessageChat,
    /// Information about the original message.
    pub forward: Option<Forward>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Kind of the message.
    pub kind: MessageKind,
}

/// This object represents a channel message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ChannelPost {
    /// Unique message identifier inside this chat.
    pub id: MessageId,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: Channel,
    /// Information about the original message.
    pub forward: Option<Forward>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Kind of the message.
    pub kind: MessageKind,
}

/// Information about the original message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Forward {
    /// Date the original message was sent in Unix time
    pub date: Integer,
    /// Sender of the original message.
    pub from: ForwardFrom,
}

/// Information about the source of the original message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ForwardFrom {
    /// Sender of the original message.
    User {
        /// Sender of the original message.
        user: User,
    },
    /// For messages forwarded from a channel, information about the original channel.
    Channel {
        /// Original channel.
        channel: Channel,
        /// Identifier of the original message in the channel
        message_id: Integer,
    },
    ChannelHiddenUser {
        sender_name: String,
    },
}

/// Kind of the message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageKind {
    /// Text message.
    Text {
        /// Actual UTF-8 text of the message, 0-4096 characters.
        data: String,
        /// Special entities like usernames, URLs, bot commands, etc. that appear in the text
        entities: Vec<MessageEntity>,
    },
    /// Message is an animation.
    Animation {
        /// Information about the animation.
        data: Animation,
        /// Caption for the animation, 0-200 characters.
        caption: Option<String>,
        /// The unique identifier of a media message group this message belongs to.
        media_group_id: Option<String>,
    },
    /// Message is an audio file.
    Audio {
        /// Information about the file.
        data: Audio,
    },
    /// Message is a general file.
    Document {
        /// Information about the file.
        data: Document,
        /// Caption for the document, 0-200 characters.
        caption: Option<String>,
    },
    /// Message is a photo.
    Photo {
        /// Available sizes of the photo.
        data: Vec<PhotoSize>,
        /// Caption for the photo, 0-200 characters.
        caption: Option<String>,
        /// The unique identifier of a media message group this message belongs to.
        media_group_id: Option<String>,
    },
    /// Message is a sticker.
    Sticker {
        /// Information about the sticker.
        data: Sticker,
    },
    /// Message is a video.
    Video {
        /// Information about the video.
        data: Video,
        /// Caption for the video, 0-200 characters.
        caption: Option<String>,
        /// The unique identifier of a media message group this message belongs to.
        media_group_id: Option<String>,
    },
    /// Message is a voice message.
    Voice {
        /// Information about the file.
        data: Voice,
    },
    /// Message is a video note.
    VideoNote {
        /// Information about the file.
        data: VideoNote,
    },
    /// Message is a shared contact.
    Contact {
        /// Information about the contact.
        data: Contact,
    },
    /// Message is a shared location.
    Location {
        /// Information about the location.
        data: Location,
    },
    /// Message is a poll.
    Poll {
        /// Information about the poll.
        data: Poll,
    },
    /// Message is a venue.
    Venue {
        /// Information about the venue.
        data: Venue,
    },
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these members)
    NewChatMembers {
        /// Information about user (this member may be the bot itself).
        data: Vec<User>,
    },
    /// A member was removed from the group.
    LeftChatMember {
        /// Information about user (this member may be the bot itself).
        data: User,
    },
    /// New chat title.
    NewChatTitle {
        /// A chat title was changed to this value.
        data: String,
    },
    /// New chat photo.
    NewChatPhoto {
        /// A chat photo was change to this value.
        data: Vec<PhotoSize>,
    },
    /// Service message: the chat photo was deleted.
    DeleteChatPhoto,
    /// Service message: the group has been created.
    GroupChatCreated,
    /// Service message: the supergroup has been created. This field can‘t be received in a
    /// message coming through updates, because bot can’t be a member of a supergroup when
    /// it is created. It can only be found in reply_to_message if someone replies to a very
    /// first message in a directly created supergroup.
    SupergroupChatCreated,
    /// Service message: the channel has been created. This field can‘t be received in a message
    /// coming through updates, because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies
    /// to a very first message in a channel.
    ChannelChatCreated,
    /// The group has been migrated to a supergroup.
    MigrateToChatId {
        /// Supergroup chat identifier.
        data: Integer,
    },
    /// The supergroup has been migrated from a group.
    MigrateFromChatId {
        /// Group chat identifier.
        data: Integer,
    },
    /// Specified message was pinned.
    PinnedMessage {
        // Specified message was pinned. Note that the Message object in this field will not
        // contain further reply_to_message fields even if it is itself a reply.
        data: Box<MessageOrChannelPost>,
    },
    #[doc(hidden)]
    Unknown { raw: Box<RawMessage> },
}

impl Message {
    fn from_raw_message(raw: RawMessage) -> Result<Self, String> {
        let id = raw.message_id;
        let from = match raw.from.clone() {
            Some(from) => from,
            None => return Err("Missing `from` field for Message".into()),
        };
        let date = raw.date;
        let chat = match raw.chat.clone() {
            Chat::Private(x) => MessageChat::Private(x),
            Chat::Group(x) => MessageChat::Group(x),
            Chat::Supergroup(x) => MessageChat::Supergroup(x),
            Chat::Unknown(x) => MessageChat::Unknown(x),
            Chat::Channel(_) => return Err("Channel chat in Message".into()),
        };

        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;

        let forward = match (
            raw.forward_date,
            &raw.forward_from,
            &raw.forward_from_chat,
            raw.forward_from_message_id,
            &raw.forward_sender_name,
        ) {
            (None, &None, &None, None, &None) => None,
            (Some(date), &Some(ref from), &None, None, &None) => Some(Forward {
                date,
                from: ForwardFrom::User { user: from.clone() },
            }),
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id), &None) => {
                Some(Forward {
                    date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id,
                    },
                })
            }
            (Some(date), &None, &None, None, &Some(ref sender_name)) => Some(Forward {
                date,
                from: ForwardFrom::ChannelHiddenUser {
                    sender_name: sender_name.clone(),
                },
            }),
            _ => return Err("invalid forward fields combination".into()),
        };

        let make_message = |kind| {
            Ok(Message {
                id: id.into(),
                from,
                date,
                chat,
                forward,
                reply_to_message,
                edit_date,
                kind,
            })
        };

        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant { data: val });
                }
            }};
        }

        macro_rules! maybe_field_with_caption {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                    });
                }
            }};
        }

        macro_rules! maybe_field_with_caption_and_group {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                        media_group_id: raw.media_group_id,
                    });
                }
            }};
        }

        macro_rules! maybe_true_field {
            ($name:ident, $variant:ident) => {{
                if let Some(True) = raw.$name {
                    return make_message(MessageKind::$variant);
                }
            }};
        }

        if let Some(text) = raw.text {
            let entities = raw.entities.unwrap_or_else(Vec::new);
            return make_message(MessageKind::Text {
                data: text,
                entities,
            });
        }

        maybe_field_with_caption_and_group!(animation, Animation);
        maybe_field!(audio, Audio);
        maybe_field_with_caption!(document, Document);
        maybe_field_with_caption_and_group!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field_with_caption_and_group!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(video_note, VideoNote);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(poll, Poll);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_members, NewChatMembers);
        maybe_field!(left_chat_member, LeftChatMember);
        maybe_field!(new_chat_title, NewChatTitle);
        maybe_field!(new_chat_photo, NewChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(group_chat_created, GroupChatCreated);
        maybe_true_field!(supergroup_chat_created, SupergroupChatCreated);
        maybe_true_field!(channel_chat_created, ChannelChatCreated);
        maybe_field!(migrate_to_chat_id, MigrateToChatId);
        maybe_field!(migrate_from_chat_id, MigrateFromChatId);
        maybe_field!(pinned_message, PinnedMessage);

        make_message(MessageKind::Unknown { raw: Box::new(raw) })
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Self::from_raw_message(raw).map_err(D::Error::custom)
    }
}

impl ChannelPost {
    fn from_raw_message(raw: RawMessage) -> Result<Self, String> {
        let id = raw.message_id;
        let date = raw.date;
        let chat = match raw.chat.clone() {
            Chat::Channel(channel) => channel,
            _ => return Err("Expected channel chat type for ChannelMessage".into()),
        };
        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;

        let forward = match (
            raw.forward_date,
            &raw.forward_from,
            &raw.forward_from_chat,
            raw.forward_from_message_id,
            &raw.forward_sender_name,
        ) {
            (None, &None, &None, None, &None) => None,
            (Some(date), &Some(ref from), &None, None, &None) => Some(Forward {
                date,
                from: ForwardFrom::User { user: from.clone() },
            }),
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id), &None) => {
                Some(Forward {
                    date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id,
                    },
                })
            }
            (Some(date), &None, &None, None, &Some(ref sender_name)) => Some(Forward {
                date,
                from: ForwardFrom::ChannelHiddenUser {
                    sender_name: sender_name.clone(),
                },
            }),
            _ => return Err("invalid forward fields combination".into()),
        };

        let make_message = |kind| {
            Ok(ChannelPost {
                id: id.into(),
                date,
                chat,
                forward,
                reply_to_message,
                edit_date,
                kind,
            })
        };

        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant { data: val });
                }
            }};
        }

        macro_rules! maybe_field_with_caption {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                    });
                }
            }};
        }

        macro_rules! maybe_field_with_caption_and_group {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                        media_group_id: raw.media_group_id,
                    });
                }
            }};
        }

        macro_rules! maybe_true_field {
            ($name:ident, $variant:ident) => {{
                if let Some(True) = raw.$name {
                    return make_message(MessageKind::$variant);
                }
            }};
        }

        if let Some(text) = raw.text {
            let entities = raw.entities.unwrap_or_else(Vec::new);
            return make_message(MessageKind::Text {
                data: text,
                entities,
            });
        }

        maybe_field_with_caption_and_group!(animation, Animation);
        maybe_field!(audio, Audio);
        maybe_field_with_caption!(document, Document);
        maybe_field_with_caption_and_group!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field_with_caption_and_group!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(video_note, VideoNote);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(poll, Poll);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_members, NewChatMembers);
        maybe_field!(left_chat_member, LeftChatMember);
        maybe_field!(new_chat_title, NewChatTitle);
        maybe_field!(new_chat_photo, NewChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(group_chat_created, GroupChatCreated);
        maybe_true_field!(supergroup_chat_created, SupergroupChatCreated);
        maybe_true_field!(channel_chat_created, ChannelChatCreated);
        maybe_field!(migrate_to_chat_id, MigrateToChatId);
        maybe_field!(migrate_from_chat_id, MigrateFromChatId);
        maybe_field!(pinned_message, PinnedMessage);

        make_message(MessageKind::Unknown { raw: Box::new(raw) })
    }
}

impl<'de> Deserialize<'de> for ChannelPost {
    // TODO(knsd): Remove .clone()
    fn deserialize<D>(deserializer: D) -> Result<ChannelPost, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Self::from_raw_message(raw).map_err(D::Error::custom)
    }
}

impl<'de> Deserialize<'de> for MessageOrChannelPost {
    // TODO(knsd): Remove .clone()
    fn deserialize<D>(deserializer: D) -> Result<MessageOrChannelPost, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;
        let is_channel = matches!(raw.chat, Chat::Channel(_));

        let res = if is_channel {
            ChannelPost::from_raw_message(raw).map(MessageOrChannelPost::ChannelPost)
        } else {
            Message::from_raw_message(raw).map(MessageOrChannelPost::Message)
        };

        res.map_err(D::Error::custom)
    }
}

/// This object represents a message. Directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessage {
    /// Unique message identifier inside this chat.
    pub message_id: Integer,
    /// Sender, can be empty for messages sent to channels.
    pub from: Option<User>,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: Chat,
    /// For forwarded messages, sender of the original message.
    pub forward_from: Option<User>,
    /// For messages forwarded from a channel, information about the original channel.
    pub forward_from_chat: Option<Chat>,
    /// For forwarded channel posts, identifier of the original message in the channel.
    pub forward_from_message_id: Option<Integer>,
    /// For forwarded messages, date the original message was sent in Unix time.
    pub forward_date: Option<Integer>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to.
    pub media_group_id: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc.
    /// that appear in the text.
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation file, information about the file.
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file.
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file.
    pub document: Option<Document>,
    // pub game: Option<Game>,
    /// Message is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker.
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video.
    pub video: Option<Video>,
    /// Message is a voice message, information about the file.
    pub voice: Option<Voice>,
    /// Message is a video note message, information about the file.
    pub video_note: Option<VideoNote>,
    /// Caption for the document, photo or video, 0-200 characters.
    pub caption: Option<String>,
    /// Message is a shared contact, information about the contact.
    pub contact: Option<Contact>,
    /// Message is a shared location, information about the location.
    pub location: Option<Location>,
    /// Message is a native poll, information about the poll.
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue.
    pub venue: Option<Venue>,
    /// New members that were added to the group or supergroup and information
    /// about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about
    /// them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value.
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value.
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: Option<True>,
    /// Service message: the group has been created.
    pub group_chat_created: Option<True>,
    /// Service message: the supergroup has been created. This field can‘t be received in a
    /// message coming through updates, because bot can’t be a member of a supergroup when
    /// it is created. It can only be found in reply_to_message if someone replies to a very
    /// first message in a directly created supergroup.
    pub supergroup_chat_created: Option<True>,
    /// Service message: the channel has been created. This field can‘t be received in a message
    /// coming through updates, because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies
    /// to a very first message in a channel.
    pub channel_chat_created: Option<True>,
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned. Note that the Message object in this field will not contain
    /// further reply_to_message fields even if it is itself a reply.
    pub pinned_message: Option<Box<MessageOrChannelPost>>,
    /// Forward from channel by a hidden user.
    pub forward_sender_name: Option<String>,
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageEntity {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// Kind of the entity.
    pub kind: MessageEntityKind,
}

/// Kind of the entity.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    BotCommand,
    Url,
    Email,
    Bold,
    Italic,
    Code,
    Pre,
    TextLink(String), // TODO(knsd) URL?
    TextMention(User),
    #[doc(hidden)]
    Unknown(RawMessageEntity),
}

impl<'de> Deserialize<'de> for MessageEntity {
    fn deserialize<D>(deserializer: D) -> Result<MessageEntity, D::Error>
    where
        D: Deserializer<'de>,
    {
        use self::MessageEntityKind::*;

        let raw: RawMessageEntity = Deserialize::deserialize(deserializer)?;

        let offset = raw.offset;
        let length = raw.length;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }

        let kind = match raw.type_.as_str() {
            "mention" => Mention,
            "hashtag" => Hashtag,
            "bot_command" => BotCommand,
            "url" => Url,
            "email" => Email,
            "bold" => Bold,
            "italic" => Italic,
            "code" => Code,
            "pre" => Pre,
            "text_link" => TextLink(required_field!(url)),
            "text_mention" => TextMention(required_field!(user)),
            _ => Unknown(raw),
        };

        Ok(MessageEntity {
            offset,
            length,
            kind,
        })
    }
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc. Directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, bot_command, url, email,
    /// bold (bold text), italic (italic text), code (monowidth string), pre (monowidth block),
    /// text_link (for clickable text URLs), text_mention (for users without usernames).
    #[serde(rename = "type")]
    pub type_: String,
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units.
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text.
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user.
    pub user: Option<User>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PhotoSize {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Photo width.
    pub width: Integer,
    /// Photo height.
    pub height: Integer,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a animation file.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Animation {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Video width as defined by sender.
    pub width: Integer,
    /// Video height as defined by sender.
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender.
    pub duration: Integer,
    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Audio {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender.
    pub duration: Integer,
    /// Performer of the audio as defined by sender or by audio tags.
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Document {
    /// Unique file identifier.
    pub file_id: String,
    /// Document thumbnail as defined by sender.
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a sticker.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Sticker width.
    pub width: Integer,
    /// Sticker height.
    pub height: Integer,
    /// Sticker thumbnail in .webp or .jpg format.
    pub thumb: Option<PhotoSize>,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// The name of the sticker set this sticker belongs to.
    pub set_name: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a video file.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Video {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Video width as defined by sender.
    pub width: Integer,
    /// Video height as defined by sender.
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender.
    pub duration: Integer,
    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a voice note.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Voice {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Duration of the audio in seconds as defined by sender.
    pub duration: Integer,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct VideoNote {
    /// Unique identifier for this file.
    pub file_id: String,
    pub length: Integer,
    /// Duration of the video in seconds as defined by sender.
    pub duration: Integer,
    /// Video thumbnail.
    pub thumb: Option<PhotoSize>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a phone contact.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Contact {
    /// Contact's phone number.
    pub phone_number: String,
    /// Contact's first name.
    pub first_name: String,
    /// Contact's last name.
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram.
    pub user_id: Option<Integer>,
}

/// This object represents a point on the map.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender.
    pub longitude: Float,
    /// Latitude as defined by sender.
    pub latitude: Float,
}

/// This object represents a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Venue {
    /// Venue location.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Address of the venue.
    pub address: String,
    /// Foursquare identifier of the venue.
    pub foursquare_id: Option<String>,
}

/// This object contains information about a poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,
    /// Poll question.
    pub question: String,
    /// List of poll options.
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll.
    pub total_voter_count: Integer,
    /// True, if the poll is closed.
    pub is_closed: bool,
    /// True, if the poll is anonymous.
    pub is_anonymous: bool,
    /// Poll type.
    #[serde(rename = "type")]
    pub type_: PollType,
    /// True, if the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option. Available only for polls in the quiz mode,
    /// which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<Integer>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll.
    pub explanation: Option<String>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed.
    pub close_date: Option<Integer>,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,
    /// The user, who changed the answer to the poll.
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
}

/// This object contains information about one answer option in a poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PollOption {
    /// Option text.
    pub text: String,
    /// Number of users that voted for this option.
    pub voter_count: Integer,
}

/// Type of a poll.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum PollType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "quiz")]
    Quiz,
}

/// This object represent a user's profile pictures.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct File {
    /// Unique identifier for this file.
    pub file_id: String,
    /// File size, if known.
    pub file_size: Option<Integer>,
    /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    pub file_path: Option<String>,
}

impl File {
    pub fn get_url(&self, token: &str) -> Option<String> {
        self.file_path
            .as_ref()
            .map(|path| format!("{}file/bot{}/{}", telegram_api_url(), token, path))
    }
}

/// Strongly typed ParseMode.
/// See [documentation](https://core.telegram.org/bots/api#formatting-options) for details.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub enum ParseMode {
    /// Use legacy markdown formatting.
    Markdown,
    /// Use MarkdownV2 formatting.
    MarkdownV2,
    /// Use HTML formatting.
    #[serde(rename = "HTML")]
    Html,
}

impl ::std::fmt::Display for ParseMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ParseMode::Markdown => write!(f, "Markdown"),
            ParseMode::MarkdownV2 => write!(f, "MarkdownV2"),
            ParseMode::Html => write!(f, "HTML"),
        }
    }
}
