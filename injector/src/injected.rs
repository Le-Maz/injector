use crate::injector::Injector;

use super::Injectable;

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
