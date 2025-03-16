use crate::injector::Injector;

use super::Injectable;

use std::fmt::Debug;
use std::ops::Deref;

use std::sync::Arc;

pub struct Injected<T>
where
    T: ?Sized,
{
    pub(crate) inner: Arc<T>,
}

impl<T> Deref for Injected<T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Injectable for Injected<T>
where
    T: Injectable + Send + Sync + 'static,
{
    fn construct(injector: &mut Injector) -> Self {
        injector.get::<T>()
    }
}

impl<T> Debug for Injected<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Injected")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T> Clone for Injected<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Into<Arc<T>> for Injected<T> {
    fn into(self) -> Arc<T> {
        self.inner
    }
}
