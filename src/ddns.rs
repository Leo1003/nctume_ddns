use crate::error::AppResult;
use reqwest::Client;
use serde_json::Value as JsonValue;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct DDnsRecord {
    id: u64,
    ttl: usize,
    name: String,
    user_token: String,
    last_ip: Ipv4Addr,
}

