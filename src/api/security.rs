// File: src/api/security.rs

use warp::Filter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key: String,
    pub permissions: Vec<String>, // List of allowed actions or endpoints
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub requests: Arc<Mutex<HashMap<String, (usize, Instant)>>>, // Map of IP to (count, timestamp)
    pub max_requests: usize,
    pub window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn check(&self, ip: &str) -> Result<(), String> {
        let mut requests = self.requests.lock().map_err(|_| "Rate limiter lock failed")?;
        let now = Instant::now();

        if let Some((count, timestamp)) = requests.get_mut(ip) {
            if now.duration_since(*timestamp) > self.window {
                *count = 1;
                *timestamp = now;
            } else if *count >= self.max_requests {
                return Err("Rate limit exceeded".to_string());
            } else {
                *count += 1;
            }
        } else {
            requests.insert(ip.to_string(), (1, now));
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Unauthorized;
impl Reject for Unauthorized {}

pub fn with_auth(api_keys: Arc<HashMap<String, ApiKey>>) -> impl Filter<Extract = (ApiKey,), Error = warp::Rejection> + Clone {
    warp::header::optional("Authorization").and_then(move |auth: Option<String>| {
        let keys = api_keys.clone();
        async move {
            match auth {
                Some(token) if keys.contains_key(&token) => {
                    Ok(keys.get(&token).unwrap().clone())
                }
                _ => Err(warp::reject::custom(Unauthorized)),
            }
        }
    })
}

pub fn with_rate_limit(rate_limiter: Arc<RateLimiter>) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::addr::remote().and_then(move |addr: Option<std::net::SocketAddr>| {
        let limiter = rate_limiter.clone();
        async move {
            match addr {
                Some(socket) => {
                    let ip = socket.ip().to_string();
                    limiter.check(&ip).map_err(|_| warp::reject::custom(Unauthorized))
                }
                None => Err(warp::reject::custom(Unauthorized)),
            }
        }
    })
}
