use std::sync::Mutex;

use injector::global_injector;

#[derive(Default)]
struct ExampleService {
    example_field: Mutex<i64>,
}

fn set_to_17() {
    let example_service = global_injector().get::<ExampleService>();
    let mut lock = example_service.example_field.lock().unwrap();
    *lock = 17;
}

fn assert_17() {
    let example_service = global_injector().get::<ExampleService>();
    let lock = example_service.example_field.lock().unwrap();
    assert_eq!(*lock, 17);
}

#[test]
fn global_injector_test() {
    set_to_17();
    assert_17();
}
