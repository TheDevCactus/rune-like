use reqwest::header::AUTHORIZATION;
use serde;
use serde::{Deserialize, Serialize};

use crate::env_manager;

pub struct DiscordClient {
    pub posts: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: String,
    content: String,
    author: Author,
    timestamp: String,
    edited_timestamp: Option<String>,
    reactions: Option<Vec<Reaction>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    id: String,
    username: String,
    discriminator: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    emoji: Emoji,
    me: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    id: Option<String>,
    name: Option<String>,
}

impl DiscordClient {
    pub fn new() -> Self {
        Self { posts: Vec::new() }
    }

    pub async fn scrape_posts(&mut self) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();

        let res = client
            .get(
                std::env::var(env_manager::CHANNELS_ENDPOINT)
                    .unwrap()
                    .as_str(),
            )
            .header(
                AUTHORIZATION,
                std::env::var(env_manager::AUTH_TOKEN).unwrap().as_str(),
            )
            .send()
            .await?;

        let raw_messages = res.text().await?;
        let mut parsed_messages: Vec<Message> = serde_json::from_str(&raw_messages).unwrap();

        parsed_messages.iter_mut().for_each(|post| {
            self.clean_post(post);
        });
        self.posts = parsed_messages;
        return Ok(());
    }

    pub fn clean_post(&self, post: &mut Message) {
        let pattern = r"<.*?:(\w+):(\d+)>";
        let re = regex::Regex::new(pattern).unwrap();
        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_file("emoji", "./templates/emote.handlebars")
            .expect("failed to register template");

        re.captures_iter(&post.content.clone())
            .for_each(|emoji_capture| {
                let full_cap = &emoji_capture[0];
                let emoji_name = &emoji_capture[1];
                let emoji_id = &emoji_capture[2];

                post.content = post.content.replace(
                    full_cap,
                    handlebars
                        .render(
                            "emoji",
                            &serde_json::json!({
                                "name": emoji_name,
                                "id": emoji_id
                            }),
                        )
                        .expect("failed to parse image")
                        .as_str(),
                );
            });

        post.timestamp = post.timestamp.split(".").collect::<Vec<&str>>()[0].to_string();
        let hour = post.timestamp.split(":").collect::<Vec<&str>>()[0]
            .to_string()
            .split("T")
            .collect::<Vec<&str>>()[1]
            .to_string();
        let mut pm = false;
        if hour > "12".to_string() {
            pm = true;
            let hour = hour.trim().parse::<i32>().unwrap() - 12;
            post.timestamp = format!(
                "{}:{} {} - {}",
                hour,
                post.timestamp.split(":").collect::<Vec<&str>>()[1],
                if pm { "PM" } else { "AM" },
                post.timestamp.split("T").collect::<Vec<&str>>()[0]
            );
        } else {
            post.timestamp = format!(
                "{}:{} {} - {}",
                hour,
                post.timestamp.split(":").collect::<Vec<&str>>()[1],
                if pm { "PM" } else { "AM" },
                post.timestamp.split("T").collect::<Vec<&str>>()[0]
            );
        }
    }

    pub async fn get_authors_posts(&mut self) -> Result<Vec<&Message>, reqwest::Error> {
        if self.posts.len() == 0 {
            self.scrape_posts().await?;
        }

        let authors_posts: Vec<&Message> = self
            .posts
            .iter()
            .filter(|post| {
                post.reactions
                    .as_ref()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .any(|reaction| {
                        if let Some(emoji_name) = reaction.emoji.name.clone() {
                            if emoji_name == std::env::var(env_manager::POST_EMOJI).unwrap() {
                                return true;
                            }
                        }
                        return false;
                    })
            })
            .collect();

        return Ok(authors_posts);
    }
}
