//! Tabbed widgets

use crate::*;
use super::*;

#[derive(Debug, Copy, Clone)]
pub enum TabSide {
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Debug)]
pub struct Tabbed<T> {
    pub side:  Option<TabSide>,
    pub focus: Option<usize>,
    pub pages: Vec<(String, T)>,
    pub range: (usize, usize)
}

impl<T> Proxy<Option<T>> for Tabbed<T> {
    fn get (&self) -> &Option<T> {
        &self.focus.and_then(|focus|self.pages.get(focus)).map(|page|page.1)
    }
    fn get_mut (&mut self) -> &mut Option<T> {
        &mut self.focus.and_then(|focus|self.pages.get_mut(focus)).map(|page|page.1)
    }
}

impl<T> Tabbed<T> {

    fn add_tabs <'a, U, V, W: Output<U, V> + Collection<'a, U, V>> (
        &'a self, mut container: W
    ) -> W where String: Output<U, V> {
        let selected = self.focus;
        let (skip, size) = self.range;
        for (index, (label, _)) in self.pages.iter().enumerate().skip(skip) {
            let label = label.clone();
            let focused = Some(index) == self.focus;
            container = container.add(label); /*.style(&|s: String|if focused {
                s.with(Color::Yellow).bold()
            } else {
                s.with(Color::White)
            }));*/
            if index >= size {
                break
            }
        }
        container
    }

    pub fn tabs <'a, U: 'a, V: 'a> (&'a self) -> Option<Box<dyn Output<U, V> + 'a>> where
        String:            Output<U, V>,
        Columns<'a, U, V>: Output<U, V> + Collection<'a, U, V>,
        Rows<'a, U, V>:    Output<U, V> + Collection<'a, U, V>
    {
        match self.side {
            None => None,
            Some(side) => Some(match side {
                TabSide::Left | TabSide::Right  =>
                    Box::new(self.add_tabs(Columns::new())),
                TabSide::Top  | TabSide::Bottom =>
                    Box::new(self.add_tabs(Rows::new()))
            })
        }
    }

    pub fn layout <'a, U, V> (&'a self) -> Collected<'a, U, V> where
        T: Output<U, V>,
        String: Output<U, V>,
        Columns<'a, U, V>: Output<U, V>,
        Rows<'a, U, V>:    Output<U, V>,
        u16:                       Output<U, V>,
    {
        let page = self.get();
        let space = if page.is_some() { 1u16 } else { 0u16 };
        match self.side {
            None => Collected::Ref(page),
            Some(side) => Collected::Box(match side {
                TabSide::Left   => Box::new(Columns::new()
                    .add(self.tabs()).add(space).add(page)) as Box<dyn Output<U, V>>,
                TabSide::Top    => Box::new(Rows::new()
                    .add(self.tabs()).add(space).add(page)) as Box<dyn Output<U, V>>,
                TabSide::Right  => Box::new(Columns::new()
                    .add(page).add(space).add(self.tabs())) as Box<dyn Output<U, V>>,
                TabSide::Bottom => Box::new(Rows::new()
                    .add(page).add(space).add(self.tabs())) as Box<dyn Output<U, V>>
            })
        }
    }

}
