use crate::error::{AppError, AppResult};
use chrono::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DDnsRecord {
    id: u64,
    ttl: usize,
    name: String,
    user_token: String,
    last_ip: Ipv4Addr,
}

impl DDnsRecord {
    pub async fn init(id: u64, user_token: &str) -> AppResult<Self> {
        let get_url = format!("https://api.nctu.me/records/{}/", id);
        if cfg!(debug_assertions) {
            trace!("URL: {}", &get_url);
            trace!("Token: {}", &user_token);
        }
        let respond = Client::new()
            .get(&get_url)
            .query(&[("token", user_token)])
            .send()
            .await?
            .error_for_status()?;
        let msg: RecordRespond = respond.json().await?;
        let msg = msg.msg;
        debug!("Got DNS record: {:?}", msg);

        if msg.content.r#type != "A" {
            error!(
                "DNS type not match, expected: \"A\", got: \"{}\"",
                msg.content.r#type
            );
            return Err(AppError::new("DNS type not match"));
        }

        Ok(DDnsRecord {
            id: id,
            ttl: msg.content.ttl,
            name: msg.content.name,
            user_token: user_token.into(),
            last_ip: msg.content.content,
        })
    }

    pub async fn update(&mut self) -> AppResult<Ipv4Addr> {
        let url = format!("https://api.nctu.me/records/{}/", self.id);
        let ipaddr = Self::get_ip().await.map_err(|e| {
            error!("Failed to get current IP address: {:?}", e);
            e
        })?;
        if self.last_ip == ipaddr {
            info!("IP address not changed since last sync");
            Ok(ipaddr)
        } else {
            info!("IP address changed '{}' -> '{}'", self.last_ip, ipaddr);
            let content = RecordContent {
                content: ipaddr,
                ttl: self.ttl,
                r#type: String::from("A"),
                name: self.name.clone(),
            };
            let content_string = format!("{}", serde_json::to_string(&content)?);
            Client::new()
                .put(&url)
                .query(&[("content", &content_string)])
                .query(&[("token", &self.user_token)])
                .send()
                .await?
                .error_for_status()?;
            self.last_ip = ipaddr;
            Ok(ipaddr)
        }
    }

    async fn get_ip() -> AppResult<Ipv4Addr> {
        let ip_string = Client::new()
            .get("https://api.ipify.org")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(Ipv4Addr::from_str(&ip_string)?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RecordRespond {
    msg: RecordMessage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RecordMessage {
    id: u64,
    content: RecordContent,
    #[serde(with = "simple_datetime")]
    created_at: DateTime<Local>,
    #[serde(with = "simple_datetime")]
    updated_at: DateTime<Local>,
    domain_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RecordContent {
    content: Ipv4Addr,
    ttl: usize,
    r#type: String,
    name: String,
}

// This module is modified from serde's example
// See https://serde.rs/custom-date-format.html
mod simple_datetime {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Local
            .datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

