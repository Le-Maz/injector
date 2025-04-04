use injector::{Injectable, Injected, Injector, LazyInjected};

#[derive(Injectable)]
struct PongService {
    ping_service: LazyInjected<PingService>,
}

impl PongService {
    pub fn pong(&self, value: u8) -> u8 {
        self.ping_service.add_one(value)
    }
}

#[derive(Injectable)]
struct PingService {
    pong_service: Injected<PongService>,
}

impl PingService {
    pub fn ping(&self, value: u8) -> u8 {
        self.pong_service.pong(value)
    }
    pub fn add_one(&self, value: u8) -> u8 {
        value + 1
    }
}

#[test]
fn test_service_counter_increments() {
    let mut injector = Injector::default();
    let ping_service = injector.get::<PingService>();
    assert_eq!(6, ping_service.ping(5), "Circular dependency call failed.");
}
