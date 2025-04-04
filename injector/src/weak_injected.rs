use std::sync::{Arc, LazyLock, Weak};

use crate::{Injectable, Injected, Injector};

type InitFn<T> = Box<dyn FnOnce() -> Option<Weak<T>> + Send + Sync>;

pub struct WeakInjected<T>(LazyLock<Option<Weak<T>>, InitFn<T>>);

impl<T> WeakInjected<T> {
    pub fn new(init: InitFn<T>) -> Self {
        Self(LazyLock::new(init))
    }

    pub fn upgrade(&self) -> Option<Injected<T>> {
        Some(Injected {
            inner: self.0.clone()?.upgrade()?,
        })
    }
}

impl<T> Injectable for WeakInjected<T>
where
    T: Injectable + Send + Sync + 'static,
{
    fn construct(injector: &mut Injector) -> Self {
        let weak_injector = injector.as_weak();
        WeakInjected::new(Box::new(move || {
            Some(Arc::downgrade(&weak_injector.upgrade()?.get().inner))
        }))
    }
}
