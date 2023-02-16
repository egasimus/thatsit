use crate::{
    *,
    layouts::*,
    engines::tui::{
        *,
        TUIInputEvent as Event
    }
};

use std::{io::{Error, ErrorKind}};

impl<S: AsRef<str>> Input<Event, bool> for S {
    fn handle (&mut self, context: Event) -> Result<Option<bool>> {
        // FIXME: render the string as a prompt
        Ok(None)
    }
}

impl<'a> Output<TUI<'a>, [u16;2]> for String {
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        // FIXME: render the string as a label
        Ok(Some([10, 10]))
    }
}

impl<'a> Output<TUI<'a>, [u16;2]> for &str {
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        // FIXME: render the string as a label
        Ok(Some([10, 10]))
    }
}

impl<'a, W: Output<TUI<'a>, [u16;2]>> Output<TUI<'a>, [u16;2]> for Fixed<u16, W> {
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        self.get().render(context.area(|area|match self {
            Self::X(width, _)            => [area.x(), area.y(), *width, area.h()],
            Self::Y(height, _)           => [area.x(), area.y(), area.w(), *height],
            Self::XY((width, height), _) => [area.x(), area.y(), *width, *height]
        }))
    }
}

impl<'a, T> Output<TUI<'a>, [u16;2]> for Max<u16, T>
where
    T: Output<TUI<'a>, [u16;2]>
{
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        self.get().render(context.area(|area|match self {
            Self::X(max_width, _) => {
                [area.x(), area.y(), area.w().max(*max_width), area.h()]
            },
            Self::Y(max_height, _) => {
                [area.x(), area.y(), area.w(), area.h().max(*max_height)]
            },
            Self::XY((max_width, max_height), _) => {
                [area.x(), area.y(), area.w().max(*max_width), area.h().max(*max_height)]
            }
        }))
    }
}

impl<'a, T> Output<TUI<'a>, [u16;2]> for Min<u16, T>
where
    T: Output<TUI<'a>, [u16;2]>
{
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        self.get().render(context.area(|area|match self {
            Self::X(min_width, _) => [
                area.x(), area.y(), area.w().min(*min_width), area.h()
            ],
            Self::Y(min_height, _) => [
                area.x(), area.y(), area.w(), area.h().min(*min_height)
            ],
            Self::XY((min_width, min_height), _) => [
                area.x(), area.y(), area.w().min(*min_width), area.h().min(*min_height)
            ]
        }))
    }
}

impl<'a, T> Output<TUI<'a>, [u16;2]> for Offset<u16, T>
where
    T: Output<TUI<'a>, [u16;2]>
{
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        self.2.render(context.area(|area|[
            area.x() + self.0,
            area.y() + self.1,
            area.w().saturating_sub(self.0),
            area.h().saturating_sub(self.1)
        ]))
    }
}
impl<'a> Output<TUI<'a>, [u16;2]> for Stacked<'a, TUI<'a>, [u16;2]> {
    fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
        let mut x = 0;
        let mut y = 0;
        match self.0 {
            Axis::X => {
                self.expect_min(&context.area, [self.1.len() as u16, 1])?; // FIXME height
                for item in self.1.iter() {
                    let [w, h] = Offset(x, 0, item).render(context)?.unwrap_or([0, 0]);
                    x = x + w;
                    y = y.max(h);
                }
            },
            Axis::Y => {
                self.expect_min(&context.area, [1, self.1.len() as u16])?; // FIXME width
                for item in self.1.iter() {
                    let [w, h] = Offset(0, y, item).render(context)?.unwrap_or([0, 0]);
                    x = x.max(w);
                    y = y + h;
                }
            },
            Axis::Z => {
                self.expect_min(&context.area, [1, 1 as u16])?; // FIXME size
                for item in self.1.iter().rev() {
                    let [w, h] = item.render(context)?.unwrap_or([0, 0]);
                    x = x.max(w);
                    y = y.max(h);
                }
            }
        };
        Ok(Some([x, y]))
    }
}

impl<'a> Stacked<'a, TUI<'a>, [u16;2]> {
    /// Return an error if this area is larger than the minimum needed size
    pub fn expect_min (&self, area: &impl Rect<u16>, min: [u16; 2]) -> std::io::Result<&Self> {
        let [min_w, min_h] = min;
        if area.w() < min_w || area.h() < min_h {
            let msg = format!("no space ({:?} < {}x{})", self, min_w, min_h);
            Err(Error::new(ErrorKind::Other, msg))
        } else {
            Ok(self)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{*, engines::tui::TUI, widgets::stacked::Stacked};

    struct StackedWidget;

    impl<'a> Output<TUI<'a>, [u16;2]> for StackedWidget {

        fn render (&self, context: &mut TUI<'a>) -> Result<Option<[u16;2]>> {
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
            }).render(context)
        }

    }

    #[test]
    fn should_stack () -> Result<()> {
        let widget = StackedWidget;
        Ok(())
    }
}
