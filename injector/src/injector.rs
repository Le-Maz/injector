use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex, Weak},
};

use crate::{injectable::Injectable, injected::Injected};

#[derive(Default, Debug)]
pub struct InjectorInner {
    table: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

#[derive(Default, Clone, Debug)]
pub struct Injector {
    injector: Arc<Mutex<InjectorInner>>,
}

impl Injector {
    pub fn set<T>(&mut self, data: T)
    where
        T: Any + Send + Sync + 'static,
    {
        self.injector
            .lock()
            .unwrap()
            .table
            .insert(TypeId::of::<T>(), Arc::new(data));
    }

    pub fn get<T>(&mut self) -> Injected<T>
    where
        T: Any + Injectable + Send + Sync + 'static,
    {
        if let Some(item) = self
            .injector
            .lock()
            .unwrap()
            .table
            .get(&TypeId::of::<T>())
            .and_then(|item| item.clone().downcast().ok())
        {
            Injected { inner: item }
        } else {
            let item = Arc::new(T::construct(self));
            let item_generalized: Arc<dyn Any + Send + Sync> = item.clone();
            self.injector
                .lock()
                .unwrap()
                .table
                .insert(TypeId::of::<T>(), item_generalized);
            Injected { inner: item }
        }
    }

    pub(crate) fn as_weak(&self) -> WeakInjector {
        WeakInjector { injector: Arc::downgrade(&self.injector) }
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct WeakInjector {
    injector: Weak<Mutex<InjectorInner>>,
}

impl WeakInjector {
    pub(crate) fn upgrade(&self) -> Option<Injector> {
        Some(Injector {
            injector: self.injector.upgrade()?
        })
    }
}
