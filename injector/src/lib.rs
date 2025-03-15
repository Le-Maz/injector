use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

mod injectable;
mod injector;
mod injected;

pub use crate::injectable::Injectable;
pub use crate::injector::Injector;
pub use crate::injected::Injected;

static GLOBAL_INJECTOR: LazyLock<Arc<Mutex<Injector>>> = LazyLock::new(Default::default);

pub fn global_injector() -> MutexGuard<'static, Injector> {
    GLOBAL_INJECTOR.lock().unwrap()
}
