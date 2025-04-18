use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

mod injectable;
mod injected;
mod injector;
mod weak_injected;

pub use crate::injectable::Injectable;
pub use crate::injected::Injected;
pub use crate::injector::Injector;
pub use crate::weak_injected::WeakInjected;
pub use injector_macro::Injectable;

static GLOBAL_INJECTOR: LazyLock<Arc<Mutex<Injector>>> = LazyLock::new(Default::default);

pub fn global_injector() -> MutexGuard<'static, Injector> {
    GLOBAL_INJECTOR.lock().unwrap()
}
