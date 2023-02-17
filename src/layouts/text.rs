//! Static text labels

use crate::*;

impl<S: AsRef<str>, T, U> Input<T, U> for S {
    fn handle (&mut self, _engine: &mut T) -> Result<Option<U>> {
        Ok(None)
    }
}
