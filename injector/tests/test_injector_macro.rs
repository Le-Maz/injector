use std::sync::Mutex;

use injector::{Injected, Injector, Injectable};

#[derive(Injectable)]
struct ConfigService {
    counter_increase: Mutex<usize>,
}

impl ConfigService {
    fn counter_increase(&self) -> usize {
        *self.counter_increase.lock().unwrap()
    }
    fn set_counter_increase(&self, value: usize) {
        *self.counter_increase.lock().unwrap() = value;
    }
}

#[derive(Injectable)]
struct CounterService {
    other_service: Injected<ConfigService>,
    counter: Mutex<usize>,
}

impl CounterService {
    fn inc_counter(&self) {
        let mut lock = self.counter.lock().unwrap();
        *lock += self.other_service.counter_increase();
    }

    fn get_counter(&self) -> usize {
        let lock = self.counter.lock().unwrap();
        *lock
    }
}

#[test]
fn test_service_counter_increments() {
    let mut injector = Injector::default();
    let config_service = injector.get::<ConfigService>();
    config_service.set_counter_increase(2);

    let counter_service = injector.get::<CounterService>();

    counter_service.inc_counter();
    assert_eq!(
        counter_service.get_counter(),
        2,
        "Counter should be 2 after first call."
    );

    counter_service.inc_counter();
    assert_eq!(
        counter_service.get_counter(),
        4,
        "Counter should be 4 after second call."
    );

    config_service.set_counter_increase(3);
    counter_service.inc_counter();
    assert_eq!(
        counter_service.get_counter(),
        7,
        "Counter should be 7 after third call."
    );
}
