use std::sync::Mutex;

use injector::Injector;

#[derive(Default)]
struct ExampleService {
    example_field: Mutex<i64>,
}

#[test]
fn test_injection() {
    let mut injector = Injector::default();

    {
        let example_service = injector.get::<ExampleService>();
        let mut lock = example_service.example_field.lock().unwrap();
        *lock = 5;
    }

    {
        let example_service = injector.get::<ExampleService>();
        let lock = example_service.example_field.lock().unwrap();
        assert_eq!(*lock, 5);
    }
}

#[test]
fn test_override() {
    let mut injector = Injector::default();
    injector.set(ExampleService {
        example_field: Mutex::new(7),
    });

    let example_service = injector.get::<ExampleService>();
    let lock = example_service.example_field.lock().unwrap();
    assert_eq!(*lock, 7);
}
