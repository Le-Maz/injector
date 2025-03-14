use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    ops::Deref,
    sync::{Arc, LazyLock, Mutex, MutexGuard},
};

#[derive(Default, Debug)]
pub struct Injector {
    table: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Injector {
    pub fn set<T>(&mut self, data: T)
    where
        T: Any + Send + Sync + 'static,
    {
        self.table.insert(TypeId::of::<T>(), Arc::new(data));
    }

    pub fn get<T>(&mut self) -> Injected<T>
    where
        T: Any + Injectable + Send + Sync + 'static,
    {
        if let Some(item) = self
            .table
            .get(&TypeId::of::<T>())
            .and_then(|item| item.clone().downcast().ok())
        {
            Injected { inner: item }
        } else {
            let item = Arc::new(T::construct(self));
            let item_generalized: Arc<dyn Any + Send + Sync> = item.clone();
            self.table.insert(TypeId::of::<T>(), item_generalized);
            Injected { inner: item }
        }
    }
}

static GLOBAL_INJECTOR: LazyLock<Arc<Mutex<Injector>>> = LazyLock::new(Default::default);

pub fn global_injector() -> MutexGuard<'static, Injector> {
    GLOBAL_INJECTOR.lock().unwrap()
}

pub trait Injectable {
    fn construct(injector: &mut Injector) -> Self;
}

impl<T> Injectable for T
where
    T: Default,
{
    fn construct(_: &mut Injector) -> Self {
        Default::default()
    }
}

pub struct Injected<T>
where
    T: Injectable,
{
    inner: Arc<T>,
}

impl<T> Deref for Injected<T>
where
    T: Injectable,
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
