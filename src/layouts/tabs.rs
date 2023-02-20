//! Tabbed widgets

use crate::*;
use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
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
        T:                 Output<U, V>,
        String:            Output<U, V>,
        Columns<'a, U, V>: Output<U, V>,
        Rows<'a, U, V>:    Output<U, V>,
        u16:               Output<U, V>,
    {
        let page  = self.focus.and_then(|focus|self.pages.get(focus)).map(|page|&page.1);
        let space = if page.is_some() { 1u16 } else { 0u16 };
        match self.side {
            Some(TabSide::Left) => {
                return Columns::new().add(self.tabs()).add(space).add(page).into_collected()
            },
            Some(TabSide::Top) => {
                return Rows::new().add(self.tabs()).add(space).add(page).into_collected()
            },
            Some(TabSide::Right) => {
                return Columns::new().add(page).add(space).add(self.tabs()).into_collected()
            },
            Some(TabSide::Bottom) => {
                return Rows::new().add(page).add(space).add(self.tabs()).into_collected()
            },
            None => match page {
                Some(page) => return Collected::Ref(page),
                None       => return Collected::None
            }
        }
    }

}
