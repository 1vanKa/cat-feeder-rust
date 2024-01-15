use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FeedItem {
    timeofday: String,
    feed_time_s: f32,
}

#[derive(Serialize, Deserealize)]
pub struct FeederModel {
    backward_time_s: f32,
    pause_time_s: f32,
    feed_times: Vec<FeedItem>,
    gpiochip_name: String,
    forward_line: i32,
    backward_line: i32,
}