use super::feeder::Feeder;
use super::feeder_model::FeederModel;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

#[derive()]
pub struct FeedManager {
    feeder: Feeder,
    feeds: Vec<(i32, f32)>,
}

impl FeedManager {
    pub fn new() -> FeedManager {
        let f = File::open("config.json").unwrap();
        let reader = BufReader::new(f);
        let feeder_model: FeederModel = serde_json::from_reader(reader).unwrap();
        return FeedManager {
            feeder: Feeder::new(0.0f32, 0.0f32),
            feeds: vec![(1i32, 1f32), (2i32, 2f32), (3i32, 3.)],
        };
    }

    pub fn manage(&self) {
        self.feeder.feed(2f32);
    }
}
