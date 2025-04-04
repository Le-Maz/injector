use std::sync::Mutex;

use injector::{Injectable, Injector};

trait ExampleService: Send + Sync {
    fn set_my_number(&self, number: i64);
    fn my_number(&self) -> i64;
}

impl Injectable for Box<dyn ExampleService> {
    fn construct(injector: &mut Injector) -> Self {
        Box::new(ExampleServiceImpl::construct(injector))
    }
}

#[derive(Injectable)]
struct ExampleServiceImpl {
    example_field: Mutex<i64>,
}

impl ExampleService for ExampleServiceImpl {
    fn set_my_number(&self, number: i64) {
        *self.example_field.lock().unwrap() = number;
    }
    fn my_number(&self) -> i64 {
        *self.example_field.lock().unwrap()
    }
}

#[test]
fn test_polymorphic_injection() {
    let mut injector = Injector::default();

    let example_service = injector.get::<Box<dyn ExampleService>>();
    example_service.set_my_number(23);

    let example_service = injector.get::<Box<dyn ExampleService>>();
    assert_eq!(example_service.my_number(), 23);
}
