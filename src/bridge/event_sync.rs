// File: src/bridge/event_sync.rs

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventRecord {
    event_type: String, // "burn" or "mint"
    tx_id: String,
    amount: u64,
    merkle_root: Option<String>,
    block_height: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct EventSyncState {
    pub events: Arc<Mutex<Vec<EventRecord>>>,
}

impl EventSyncState {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_event(&self, event: EventRecord) -> Result<(), String> {
        let mut events = self.events.lock().map_err(|_| "Failed to lock event state")?;
        events.push(event);
        Ok(())
    }

    pub fn get_events(&self) -> Vec<EventRecord> {
        self.events.lock().unwrap_or_else(|_| Vec::new())
    }
}

pub fn event_sync_routes(event_state: Arc<EventSyncState>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let add_event = warp::path("bridge")
        .and(warp::path("sync-event"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |event: EventRecord| {
            let state = event_state.clone();
            async move {
                match state.add_event(event) {
                    Ok(_) => Ok(warp::reply::json(&{"status": "success", "message": "Event added"})),
                    Err(e) => Ok(warp::reply::json(&{"status": "error", "message": e})),
                }
            }
        });

    let get_events = warp::path("bridge")
        .and(warp::path("events"))
        .and(warp::get())
        .map(move || {
            let state = event_state.clone();
            let events = state.get_events();
            warp::reply::json(&events)
        });

    add_event.or(get_events)
}
