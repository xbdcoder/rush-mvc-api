use std::time::{Instant, Duration};

pub struct RateLimiter {
    last_request_time: Instant,
    request_count: u32,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            last_request_time: Instant::now(),
            request_count: 0,
        }
    }

    pub fn allow_request(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_request_time) > Duration::new(1, 0) {
            // Reset the count every second
            self.last_request_time = now;
            self.request_count = 0;
        }

        // Allow request if there are less than 3 requests in the last second
        if self.request_count < 3 {
            self.request_count += 1;
            true
        } else {
            false
        }
    }
}
