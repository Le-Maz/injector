use std::{ops::Deref, sync::LazyLock};

use crate::Injected;

pub struct LazyInjected<T>(LazyLock<Injected<T>, Box<dyn FnOnce() -> Injected<T> + Send + Sync>>);

impl<T> LazyInjected<T> {
    pub fn new(init: Box<dyn FnOnce() -> Injected<T> + Send + Sync>) -> Self {
        Self(LazyLock::new(init))
    }
}

impl<T> Deref for LazyInjected<T> {
    type Target = LazyLock<Injected<T>, Box<dyn FnOnce() -> Injected<T> + Send + Sync>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
