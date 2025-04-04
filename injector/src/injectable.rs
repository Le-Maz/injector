use crate::injector::Injector;

pub trait Injectable {
    fn construct(injector: &mut Injector) -> Self;
}

impl<T: Default> Injectable for T {
    fn construct(_: &mut Injector) -> Self {
        Self::default()
    }
}
