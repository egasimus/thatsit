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
    /// On which side to render the tabs
    pub side:  Option<TabSide>,
    /// List of named tabs
    pub pages: Vec<(String, T)>,
    /// The currently focused tab
    pub focus: Option<usize>,
    /// For scrolling
    pub range: Option<(usize, usize)>
}

impl<T> Tabbed<T> {

    pub fn new (side: Option<TabSide>, pages: Vec<(String, T)>) -> Self {
        Self {
            side,
            pages,
            focus: None,
            range: None
        }
    }

    pub fn left (pages: Vec<(String, T)>) -> Self {
        Self::new(Some(TabSide::Left), pages)
    }

    pub fn right (pages: Vec<(String, T)>) -> Self {
        Self::new(Some(TabSide::Right), pages)
    }

    pub fn top (pages: Vec<(String, T)>) -> Self {
        Self::new(Some(TabSide::Top), pages)
    }

    pub fn bottom (pages: Vec<(String, T)>) -> Self {
        Self::new(Some(TabSide::Bottom), pages)
    }

    /// Create a container holding the tab container and the active tab
    pub fn layout <'a, U, V> (&'a self) -> Collected<'a, U, V> where
        T:                 Output<U, V>,
        String:            Output<U, V>,
        Columns<'a, U, V>: Output<U, V>,
        Rows<'a, U, V>:    Output<U, V>,
        u16:               Output<U, V>,
    {
        let page = self.focus.and_then(|focus|self.pages.get(focus)).map(|page|&page.1);
        match self.side {
            None => match page {
                Some(page) => return Collected::Ref(page),
                None       => return Collected::None
            }
            Some(side) => {
                let tabs = self.layout_tabs();
                let space = if page.is_some() { 1u16 } else { 0u16 };
                match side {
                    TabSide::Left => {
                        return Columns::new().add(tabs).add(space).add(page).into_collected()
                    }
                    TabSide::Top => {
                        return Rows::new().add(tabs).add(space).add(page).into_collected()
                    }
                    TabSide::Right => {
                        return Columns::new().add(page).add(space).add(tabs).into_collected()
                    }
                    TabSide::Bottom => {
                        return Rows::new().add(page).add(space).add(tabs).into_collected()
                    }
                }
            }
        }
    }

    /// Create a container holding the tabs
    pub fn layout_tabs <'a, U: 'a, V: 'a> (&'a self) -> Option<Box<dyn Output<U, V> + 'a>> where
        String:            Output<U, V>,
        Columns<'a, U, V>: Output<U, V> + Collection<'a, U, V>,
        Rows<'a, U, V>:    Output<U, V> + Collection<'a, U, V>
    {
        match self.side {
            None => None,
            Some(side) => Some(match side {
                TabSide::Left | TabSide::Right  =>
                    Box::new(self.layout_tabs_in(Columns::new())),
                TabSide::Top  | TabSide::Bottom =>
                    Box::new(self.layout_tabs_in(Rows::new()))
            })
        }
    }

    /// Add the tabs to the passed container
    pub fn layout_tabs_in <'a, U, V, W: Output<U, V> + Collection<'a, U, V>> (
        &'a self, mut container: W
    ) -> W where String: Output<U, V> {
        let selected = self.focus;
        let (skip, size) = self.range.unwrap_or((0, usize::MAX));
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

}
