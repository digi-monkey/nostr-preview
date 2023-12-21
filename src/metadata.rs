use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Metadata<'a> {
    event: &'a Event,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthorProfile<'a> {
    name: &'a str,
    picture: &'a str,
    about: &'a str,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Meta<'a> {
    title: String,
    content: &'a str,
    created_at: Timestamp,
    author: Option<AuthorProfile<'a>>,
    image: Option<&'a str>,
}

pub const MAX_TITLE_LENGTH: usize = 35;

pub fn truncate(input: &str) -> String {
    if input.len() > MAX_TITLE_LENGTH {
        return format!("{}..", &input[0..MAX_TITLE_LENGTH]);
    }
    return input.to_string();
}

impl<'a> Metadata<'a> {
    pub fn new(e: &'a Event) -> Self {
        Self { event: e }
    }

    pub fn to_meta(&self) -> Meta {
        let data = Meta {
            title: truncate(&self.event.content),
            content: &self.event.content,
            created_at: self.event.created_at,
            author: None,
            image: None,
        };
        return data;
    }
}
