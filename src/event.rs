use chrono::{ Utc, DateTime };
use crate::keys::Key;

#[derive(Debug)]
pub struct Event {
    key: Key,
    time: DateTime<Utc>,
    window_name: String
}

impl Event {
    pub fn new(key: Key, window_name: String) -> Self {
        Event {
            key,
            time: Utc::now(),
            window_name
        }
    }

}
