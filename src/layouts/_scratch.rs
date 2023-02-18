
/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
pub struct Stacked<'a, T, U>(
    /// The axis along which the components are stacked
    pub Axis,
    /// The stacked components
    pub Vec<Collected<'a, T, U>>,
);

impl<'a, T, U> std::fmt::Debug for Stacked<'a, T, U> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stacked({:?}, {:?})", self.0, &self.1)
    }
}

impl<'a, T, U> Stacked<'a, T, U> {

    /// Stacked left to right
    pub fn x (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::X, Collector::collect_items(items).0)
    }

    /// Stacked top to bottom
    pub fn y (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Y, Collector::collect_items(items).0)
    }

    /// Stacked back to front
    pub fn z (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Axis::Z, Collector::collect_items(items).0)
    }

}

    struct StackedWidget1;

    impl<T, U> Output<T, U> for StackedWidget1 {
        fn render (&self, engine: &mut T) -> Result<Option<U>> {
            Stacked::x(|add|{
                add("String");
                add(String::from("String"));
                add(Stacked::y(|add|{
                    add("String");
                    add(String::from("String"));
                    add(Stacked::z(|add|{
                        add("String");
                        add(String::from("String"));
                    }));
                }));
            }).render(engine)
        }
    }

    #[test]
    fn should_stack_callback () -> Result<()> {
        StackedWidget1.render(&mut ())?;
        Ok(())
    }

/// Like `Stacked`, but keeps track of focus
#[derive(Debug)]
pub struct FocusStack<'a, T, U>(
    pub Stacked<'a, T, U>,
    pub FocusState<usize>
);

impl<'a, T, U> FocusStack<'a, T, U> {
    pub fn new (stack: Stacked<'a, T, U>) -> Self {
        Self(stack, FocusState::default())
    }
    pub fn x (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Stacked::x(items), FocusState::default())
    }
    pub fn y (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Stacked::y(items), FocusState::default())
    }
    pub fn z (items: impl Fn(&mut Collector<'a, T, U>)) -> Self {
        Self(Stacked::z(items), FocusState::default())
    }
}

impl<'a, T, U> Focus<Collected<'a, T, U>> for FocusStack<'a, T, U> {
    fn items (&self) -> &Vec<Collected<'a, T, U>> {
        &self.0.1
    }
    fn items_mut (&mut self) -> &mut Vec<Collected<'a, T, U>> {
        &mut self.0.1
    }
    fn state (&self) -> &FocusState<usize> {
        &self.1
    }
    fn state_mut (&mut self) -> &mut FocusState<usize> {
        &mut self.1
    }
}

//impl<'a> Widget for FocusStack<'a> {
    //impl_render!(self, out, area => {
        //if let Some(item) = self.get() {
            //item.render(out, area)
        //} else {
            //Ok((0, 0))
        //}
    //});
//}
