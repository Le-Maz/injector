use crate::injector::Injector;

pub trait Injectable {
    fn construct(injector: &mut Injector) -> Self;
}
