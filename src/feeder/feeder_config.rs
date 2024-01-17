use super::feeder::Feeder;
use super::feeder_model::FeederConfig;
use std::vec::Vec;

pub struct FeedManager {
    feeder: Feeder,
    feeds: Vec<(i32, f32)>,
}

impl FeedManager {
    pub fn new(config: FeederConfig) -> FeedManager {
        return FeedManager {
            feeder: Feeder::new(config.backward_time_s, config.pause_time_s),
            feeds: vec![(1i32, 1f32), (2i32, 2f32), (3i32, 3.)],
        };
    }

    pub fn manage(&self) {
        self.feeder.feed(2f32);
    }
}
