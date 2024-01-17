use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    pub timeofday: String,
    pub feed_time_s: f32,
}

#[derive(Serialize, Deserialize)]
pub struct FeederConfig {
    pub backward_time_s: f32,
    pub pause_time_s: f32,
    pub feed_times: Vec<FeedItem>,
    pub gpiochip_name: String,
    pub forward_line: i32,
    pub backward_line: i32,
}