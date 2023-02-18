use crate::*;

pub trait Input<T, U> {
    fn handle (&mut self, engine: &mut T) -> Result<Option<U>>;
}

pub trait Proxy<T> {
    fn get (&self) -> &T;
    fn get_mut (&mut self) -> &mut T;
}

impl<T, U, V: Input<T, U>> Input<T, U> for dyn Proxy<V> {
    fn handle (&mut self, engine: &mut T) -> Result<Option<U>> {
        self.get_mut().handle(engine)
    }
}
