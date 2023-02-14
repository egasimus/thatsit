use crate::{*, widgets::stacked::Stacked};
use std::{slice::Iter, slice::IterMut};

///
/// TODO focused_at (index: usize)
/// impl Focus<T> for Vec<T> ?
/// impl Stack<T> for Vec<T> ?
///

/// The Focus API.
pub trait Focus<T> {
    /// Get an immutable reference to the list of items
    fn items (&self) -> &Vec<T>;
    /// Get a mutable reference to the list of items
    fn items_mut (&mut self) -> &mut Vec<T>;
    /// Get an immutable reference to the focus state
    fn state (&self) -> &FocusState<usize>;
    /// Get a mutable reference to the focus state
    fn state_mut (&mut self) -> &mut FocusState<usize>;

    /// Add an item
    fn push (&mut self, item: T) {
        self.items_mut().push(item);
    }
    /// Iterate over immutable references to the contained items
    fn iter (&self) -> Iter<T> {
        self.items().iter()
    }
    /// Iterate over immutable references to the contained items
    fn iter_mut (&mut self) -> IterMut<T> {
        self.items_mut().iter_mut()
    }
    /// Iterate over mutable references to the contained items
    /// Replace the list of items, resetting the item focus
    fn replace (&mut self, items: Vec<T>) {
        *self.items_mut() = items;
        self.state_mut().1 = None;
    }
    /// Count the contained items
    fn len (&self) -> usize {
        self.items().len()
    }
    /// Get an immutable reference the currently focused item
    fn get (&self) -> Option<&T> {
        match self.state().1 {
            Some(i) => self.items().get(i),
            _ => None
        }
    }
    /// Get a mutable reference the currently focused item
    fn get_mut (&mut self) -> Option<&mut T> {
        match self.state().1 {
            Some(i) => self.items_mut().get_mut(i),
            _ => None
        }
    }
    /// Set the focus
    fn focus (&mut self) -> bool {
        self.state_mut().0 = true;
        true
    }
    /// Clear the focus
    fn unfocus (&mut self) -> bool {
        self.state_mut().0 = false;
        true
    }
    /// Get the index of the currently selected item
    fn selected (&self) -> Option<usize> {
        self.state().1
    }
    /// Set the selected item
    fn select (&mut self, index: usize) -> bool {
        if self.items().get(index).is_some() {
            self.unselect();
            self.state_mut().1 = Some(index);
            true
        } else {
            false
        }
    }
    /// Select the next item
    fn select_next (&mut self) -> bool {
        if let Some(index) = self.state().1 {
            self.select(if index >= self.items().len() - 1 { 0 } else { index + 1 })
        } else {
            self.select(0)
        }
    }
    /// Select the previous item
    fn select_prev (&mut self) -> bool {
        if let Some(index) = self.state().1 {
            self.select(if index == 0 { self.items().len() - 1 } else { index - 1 })
        } else {
            self.select(0)
        }
    }
    /// Clear the selected item
    fn unselect (&mut self) -> bool {
        self.state_mut().1 = None;
        true
    }
}

/// The focus state of an item
#[derive(Debug, Default)]
pub struct FocusState<T>(
    /// Whether this item is focused
    pub bool,
    /// Whether an item owned by this item is focused
    pub Option<T>
);

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct FocusList<T> {
    /// The list of items
    items: Vec<T>,
    /// The focus state
    pub state: FocusState<usize>,
}

impl<T> Default for FocusList<T> {
    /// Create an empty focus list
    fn default () -> Self { Self { items: vec![], state: FocusState(false, None) } }
}

impl<T> FocusList<T> {
    /// Create a new focus list, taking ownership of a collection of items
    pub fn new (items: Vec<T>) -> Self { Self { items, ..Self::default() } }
}

impl<T> Focus<T> for FocusList<T> {
    fn items (&self) -> &Vec<T> { &self.items }
    fn items_mut (&mut self) -> &mut Vec<T> { &mut self.items }
    fn state (&self) -> &FocusState<usize> { &self.state }
    fn state_mut (&mut self) -> &mut FocusState<usize> { &mut self.state }
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

#[cfg(test)]
mod test {
    use crate::widgets::focus::*;

    //#[test]
    //fn should_maintain_focus_in_stack () {
        //let mut output = Vec::<u8>::new();
        //let layout = FocusStack::y(|add: Collector<'a, dyn Collectible>|{
            //add(String::from("Item1"));
            //add(FocusStack::x(|add|{
                //add(String::from("Item1"));
                //add(String::from("Item1"));
                //add(String::from("Item1"));
            //}));
            //add(String::from("Item1"));
        //});
    //}
}
