use crate::*;

/// Wrapper that allows owned and borrowed items to be treated similarly.
pub enum Collected<'a, T, U> {
    Box(Box<dyn Output<T, U> + 'a>),
    Ref(&'a dyn Output<T, U>),
    None
}

/// Callable struct that collects Collecteds into itself.
pub struct Collector<'a, T, U>(pub Vec<Collected<'a, T, U>>);

impl<'a, T, U> std::fmt::Debug for Collected<'a, T, U> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Collected({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
        })
    }
}

impl<'a, T, U> Output<T, U> for Collected<'a, T, U> {
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        Ok(match self {
            Self::Box(item) => (*item).render(context)?,
            Self::Ref(item) => (*item).render(context)?,
            Self::None => None
        })
    }
}

impl<'a, T, U, V: Output<T, U> + 'a> FnOnce<(V, )> for Collector<'a, T, U> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (V,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T, U, V: Output<T, U> + 'a> FnMut<(V, )> for Collector<'a, T, U> {
    extern "rust-call" fn call_mut (&mut self, args: (V,)) -> Self::Output {
        args.0.collect(self)
    }
}

impl<'a, T, U> Collector<'a, T, U> {
    pub fn collect (collect: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}


