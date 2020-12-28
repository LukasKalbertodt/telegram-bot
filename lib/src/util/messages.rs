//! Utility traits that extends [`telegram_bot_raw::types::message`]
//!
//! [`telegram_bot_raw::types::message`]: ../../telegram_bot_raw/types/message/index.html

use crate::prelude::CanGetFile;
use crate::types::{
    requests::get_file::GetFile, ChannelPost, Message, MessageKind, MessageOrChannelPost,
};

/// A trait to obtain text from a message.
///
/// For example, this will return the text from text messages, or the caption of a photo.
pub trait MessageText {
    /// Obtain text from a message if available.
    fn text(&self) -> Option<String>;
}

impl MessageText for MessageOrChannelPost {
    fn text(&self) -> Option<String> {
        match self {
            MessageOrChannelPost::Message(msg) => msg.text(),
            MessageOrChannelPost::ChannelPost(post) => post.text(),
        }
    }
}

impl MessageText for Message {
    fn text(&self) -> Option<String> {
        self.kind.text()
    }
}

impl MessageText for MessageKind {
    fn text(&self) -> Option<String> {
        match self {
            MessageKind::Text { data, .. } => Some(data.to_owned()),
            MessageKind::Animation { caption, .. } => caption.to_owned(),
            MessageKind::Audio { data } => data.title.to_owned(),
            MessageKind::Document { data, caption } => {
                caption.clone().or_else(|| data.file_name.clone())
            }
            MessageKind::Photo { caption, .. } => caption.to_owned(),
            MessageKind::Sticker { .. } => None,
            MessageKind::Video { caption, .. } => caption.to_owned(),
            MessageKind::Voice { .. } => None,
            MessageKind::VideoNote { .. } => None,
            MessageKind::Contact { data } => Some(data.first_name.to_owned()),
            MessageKind::Location { .. } => None,
            MessageKind::Poll { data } => Some(data.question.to_owned()),
            MessageKind::Venue { data } => Some(data.title.to_owned()),
            MessageKind::NewChatMembers { .. } => None,
            MessageKind::LeftChatMember { .. } => None,
            MessageKind::NewChatTitle { data } => Some(data.to_owned()),
            MessageKind::NewChatPhoto { .. } => None,
            MessageKind::DeleteChatPhoto => None,
            MessageKind::GroupChatCreated => None,
            MessageKind::SupergroupChatCreated => None,
            MessageKind::ChannelChatCreated => None,
            MessageKind::MigrateToChatId { .. } => None,
            MessageKind::MigrateFromChatId { .. } => None,
            MessageKind::PinnedMessage { data } => data.text(),
            MessageKind::Unknown { .. } => None,
        }
    }
}

impl MessageText for ChannelPost {
    fn text(&self) -> Option<String> {
        self.kind.text()
    }
}

/// A trait to obtain `GetFile` requests from a message.
///
/// Many message kinds such as `Sticker` return a single `GetFile`.
/// Message kinds like `Photo` might return more if an album is posted.
/// A video, video note or document returns any thumbnail as well.
pub trait MessageGetFiles {
    /// Obtain files from a message if available.
    fn get_files(&self) -> Option<Vec<GetFile>>;
}

impl MessageGetFiles for MessageOrChannelPost {
    fn get_files(&self) -> Option<Vec<GetFile>> {
        match self {
            MessageOrChannelPost::Message(msg) => msg.get_files(),
            MessageOrChannelPost::ChannelPost(post) => post.get_files(),
        }
    }
}

impl MessageGetFiles for Message {
    fn get_files(&self) -> Option<Vec<GetFile>> {
        self.kind.get_files()
    }
}

impl MessageGetFiles for MessageKind {
    fn get_files(&self) -> Option<Vec<GetFile>> {
        match self {
            MessageKind::Text { .. } => None,
            MessageKind::Animation { data, .. } => {
                let mut files = vec![data.get_file()];
                if let Some(thumb) = &data.thumb {
                    files.push(thumb.get_file());
                }
                Some(files)
            }
            MessageKind::Audio { data } => Some(vec![data.get_file()]),
            MessageKind::Document { data, .. } => {
                let mut files = vec![data.get_file()];
                if let Some(thumb) = &data.thumb {
                    files.push(thumb.get_file());
                }
                Some(files)
            }
            MessageKind::Photo { data, .. } => Some(data.iter().map(|f| f.get_file()).collect()),
            MessageKind::Sticker { data } => Some(vec![data.get_file()]),
            MessageKind::Video { data, .. } => {
                let mut files = vec![data.get_file()];
                if let Some(thumb) = &data.thumb {
                    files.push(thumb.get_file());
                }
                Some(files)
            }
            MessageKind::Voice { data } => Some(vec![data.get_file()]),
            MessageKind::VideoNote { data, .. } => {
                let mut files = vec![data.get_file()];
                if let Some(thumb) = &data.thumb {
                    files.push(thumb.get_file());
                }
                Some(files)
            }
            MessageKind::Contact { .. } => None,
            MessageKind::Location { .. } => None,
            MessageKind::Poll { .. } => None,
            MessageKind::Venue { .. } => None,
            MessageKind::NewChatMembers { .. } => None,
            MessageKind::LeftChatMember { .. } => None,
            MessageKind::NewChatTitle { .. } => None,
            MessageKind::NewChatPhoto { data } => Some(data.iter().map(|f| f.get_file()).collect()),
            MessageKind::DeleteChatPhoto => None,
            MessageKind::GroupChatCreated => None,
            MessageKind::SupergroupChatCreated => None,
            MessageKind::ChannelChatCreated => None,
            MessageKind::MigrateToChatId { .. } => None,
            MessageKind::MigrateFromChatId { .. } => None,
            MessageKind::PinnedMessage { .. } => None,
            MessageKind::Unknown { .. } => None,
        }
    }
}

impl MessageGetFiles for ChannelPost {
    fn get_files(&self) -> Option<Vec<GetFile>> {
        self.kind.get_files()
    }
}
