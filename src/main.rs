use std::fs::File;
use std::io::BufReader;

mod feeder;

use feeder::{FeedManager, FeederConfig};

fn main() {
    let f = File::open("config.json").unwrap();
    let reader = BufReader::new(f);
    let feeder_config: FeederConfig = serde_json::from_reader(reader).unwrap();
    let feed_manager = FeedManager::new(feeder_config);
    feed_manager.manage();
}
