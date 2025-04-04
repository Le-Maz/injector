use injector::{Injectable, Injected, Injector, WeakInjected};

#[derive(Injectable)]
struct PongService {
    ping_service: WeakInjected<PingService>,
}

impl PongService {
    pub fn pong(&self, value: u8) -> u8 {
        self.ping_service.upgrade().expect("Cannot upgrade a reference to a dropped Arc").add_one(value)
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
fn test_strong_dependency() {
    let mut injector = Injector::default();
    let ping_service = injector.get::<PingService>();
    assert_eq!(6, ping_service.ping(5), "Circular dependency call failed.");
}

#[test]
fn test_weak_dependency() {
    let mut injector = Injector::default();
    let pong_service = injector.get::<PongService>();
    assert_eq!(8, pong_service.pong(7), "Circular dependency call failed.");
}

#[test]
#[should_panic = "Cannot upgrade a reference to a dropped Arc"]
fn test_memory_release() {
    let mut injector = Injector::default();
    let pong_service = injector.get::<PongService>();
    drop(injector);
    pong_service.pong(0);
}
