use crate::*;

pub trait Proxy<T> {
    fn get (&self) -> &T;
    fn get_mut (&mut self) -> &mut T;
}

impl<T, U, V: Input<T, U>> Input<T, U> for dyn Proxy<V> {
    fn handle (&mut self, context: T) -> Result<Option<U>> {
        self.get_mut().handle(context)
    }
}
