//! Tabbed widgets

use super::*;

#[derive(Debug, Default)]
pub enum TabSide {
    /// Don't display the tabs
    #[default] None,
    Top,
    Right,
    Bottom,
    Left
}

#[derive(Debug)]
pub struct Tabbed<T> {
    pub side:  TabSide,
    pub pages: Focused<(String, T)>,
}
