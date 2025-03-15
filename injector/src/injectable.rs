use crate::injector::Injector;

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
