pub struct Feeder {
    backward_time: f32,
    pause_time: f32,
}

impl Feeder {
    pub fn new(backward_time: f32, pause_time: f32) -> Feeder {
        return Feeder {
            backward_time: backward_time,
            pause_time: pause_time,
        };
    }

    pub fn feed(&self, feed_time: f32) {
        println!(
            "Feed for {0}s with backward: {1}s, pause: {2}s",
            feed_time, self.backward_time, self.pause_time
        );
    }
}
