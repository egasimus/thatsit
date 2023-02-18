use crate::{
    *,
    layouts::*,
    engines::tui::{
        *,
        TUIInputEvent as Event
    }
};

use std::{io::{Write, Error, ErrorKind}};

impl<W: Write> Output<TUI<W>, [u16;2]> for String {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        self.as_str().render(engine)
    }
}

impl<W: Write> Output<TUI<W>, [u16;2]> for &str {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        engine.put(engine.area.x(), engine.area.y(), &self)?;
        // FIXME: handle line wrap
        Ok(Some([self.len() as u16, 1]))
    }
}

impl<W: Write, O: Output<TUI<W>, [u16;2]>> Output<TUI<W>, [u16;2]> for Fixed<u16, O> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        self.get().render(engine.area(|area|match self {
            Self::X(width, _)            => [area.x(), area.y(), *width, area.h()],
            Self::Y(height, _)           => [area.x(), area.y(), area.w(), *height],
            Self::XY((width, height), _) => [area.x(), area.y(), *width, *height]
        }))
    }
}

impl<W: Write, T: Output<TUI<W>, [u16;2]>> Output<TUI<W>, [u16;2]> for Max<u16, T> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        self.get().render(engine.area(|area|match self {
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

impl<W: Write, T: Output<TUI<W>, [u16;2]>> Output<TUI<W>, [u16;2]> for Min<u16, T> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        self.get().render(engine.area(|area|match self {
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

impl<W: Write, T: Output<TUI<W>, [u16;2]>> Output<TUI<W>, [u16;2]> for Offset<u16, T> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        self.2.render(engine.area(|area|[
            area.x() + self.0,
            area.y() + self.1,
            area.w().saturating_sub(self.0),
            area.h().saturating_sub(self.1)
        ]))
    }
}

impl<'a, W: Write> Output<TUI<W>, [u16;2]> for Rows<'a, TUI<W>, [u16;2]> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        let mut x = 0;
        let mut y = 0;
        expect_min(&engine.area, [1, self.0.len() as u16])?; // FIXME width
        for item in self.0.iter() {
            let [w, h] = Offset(0, y, item).render(engine)?.unwrap_or([0, 0]);
            x = x.max(w);
            y = y + h;
        }
        Ok(Some([x, y]))
    }
}

impl<'a, W: Write> Output<TUI<W>, [u16;2]> for Columns<'a, TUI<W>, [u16;2]> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        let mut x = 0;
        let mut y = 0;
        expect_min(&engine.area, [self.0.len() as u16, 1])?; // FIXME height
        for item in self.0.iter() {
            let [w, h] = Offset(x, 0, item).render(engine)?.unwrap_or([0, 0]);
            x = x + w;
            y = y.max(h);
        }
        Ok(Some([x, y]))
    }
}

impl<'a, W: Write> Output<TUI<W>, [u16;2]> for Layers<'a, TUI<W>, [u16;2]> {
    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        let mut x = 0;
        let mut y = 0;
        expect_min(&engine.area, [1, 1 as u16])?; // FIXME size
        for item in self.0.iter().rev() {
            let [w, h] = item.render(engine)?.unwrap_or([0, 0]);
            x = x.max(w);
            y = y.max(h);
        }
        Ok(Some([x, y]))
    }
}

/// Return an error if the available area is larger than the minimum needed size
fn expect_min <T: Rect<u16> + std::fmt::Debug> (area: &T, min: [u16; 2]) -> std::io::Result<&T> {
    let [min_w, min_h] = min;
    if area.w() < min_w || area.h() < min_h {
        let msg = format!("no space ({:?} < {}x{})", area, min_w, min_h);
        Err(Error::new(ErrorKind::Other, msg))
    } else {
        Ok(area)
    }
}
