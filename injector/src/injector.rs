use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
    sync::Arc,
};

use crate::{injectable::Injectable, injected::Injected};

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
