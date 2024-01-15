mod feeder;

use feeder::FeedManager;

fn main() {
    let feed_manager = FeedManager::new();
    feed_manager.manage();
}
