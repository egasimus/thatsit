//! Tabbed widgets

use crate::*;
use super::*;

#[derive(Debug)]
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
        &'a self,
        mut container: W
    ) -> W {
        let selected = self.focus;
        let (skip, size) = self.range;
        for (index, (label, _)) in self.pages.iter().enumerate().skip(skip) {
            let label = label.clone();
            if Some(index) == self.focus {
                container = container.add(label.style(&|s: String|s.with(Color::Yellow).bold()));
            } else {
                container = container.add(label.style(&|s: String|s.with(Color::White), label));
            }
            if index >= size {
                break
            }
        }
        container
    }

    pub fn tabs <'a, U: 'a, V: 'a> (&self) -> Option<Box<dyn Output<U, V>>> where
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

    pub fn layout <U, V> (&self) -> &dyn Output<U, V> where
        T: Output<U, V>,
        for<'a> Columns<'a, U, V>: Output<U, V>,
        for<'a> Rows<'a, U, V>:    Output<U, V>,
        u16:                       Output<U, V>,
    {
        let show_tabs = self.side.is_some();
        let page: Option<T> = *self.get();
        match self.side {
            None => &page,
            Some(side) => match side {
                TabSide::Left   => &Columns::new()
                    .add(self.tabs()).add(page.map(|_|1u16)).add(page) as &dyn Output<U, V>,
                TabSide::Top    => &Rows::new()
                    .add(self.tabs()).add(page.map(|_|1u16)).add(page) as &dyn Output<U, V>,
                TabSide::Right  => &Columns::new()
                    .add(page).add(page.map(|_|1u16)).add(self.tabs()) as &dyn Output<U, V>,
                TabSide::Bottom => &Rows::new()
                    .add(page).add(page.map(|_|1u16)).add(self.tabs()) as &dyn Output<U, V>
            }
        }
    }

}
