use crate::{
    *,
    layouts::*,
    engines::tui::{
        *,
        crossterm::{
            cursor::MoveTo,
            style::{Print, Color, ResetColor, SetBackgroundColor, SetForegroundColor}
        }
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

/// A border around another widget
#[derive(Copy, Clone, Default)]
pub struct Border<T: Write, U: Output<TUI<T>, [u16;2]>, V, W>(pub V, pub W, pub U);

impl<T, U, V, W> Output<TUI<T>, [u16;2]> for Border<T, U, V, W>
where
    T: Write,
    U: Output<TUI<T>, [u16;2]>
{

    fn render (&self, engine: &mut TUI<T>) -> Result<Option<[u16;2]>> {
        let TUI { output, area } = engine;

        let w = area.w();
        let h = area.h();
        if w == 0 || h == 0 {
            return Ok(None)
        }

        let x = area.x();
        let y = area.y();

        // draw top
        let (top_left, fg, bg) = T::top_left(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x, y))?.queue(Print(&top_left))?;

        let (top, fg, bg) = T::top(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x+1, y))?.queue(Print(&String::from(top).repeat((w-2) as usize)))?;

        let (top_right, fg, bg) = T::top_right(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x+w-1, y))?.queue(Print(&top_right))?;

        // draw sides and background
        let (left, fg, bg) = T::left(&self.1);
        set_colors(output, &fg, &bg)?;
        for y in y+1..y+h-1 {
            output.queue(MoveTo(x, y))?.queue(Print(&left))?;
        }

        set_colors(output, &self.1.hi(), &self.1.bg())?;
        for y in y+1..y+h-1 {
            output.queue(MoveTo(x+1, y))?.queue(Print(&" ".repeat((w-2) as usize)))?;
        }

        let (right, fg, bg) = T::right(&self.1);
        set_colors(output, &fg, &bg)?;
        for y in y+1..y+h-1 {
            output.queue(MoveTo(x+w-1, y))?.queue(Print(&right))?;
        }

        // draw bottom
        let (bottom_left, fg, bg) = T::bottom_left(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x, y+h-1))?.queue(Print(&bottom_left))?;

        let (bottom, fg, bg) = T::bottom(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x+1, y+h-1))?.queue(Print(&String::from(bottom).repeat((w-2) as usize)))?;

        let (bottom_right, fg, bg) = T::bottom_right(&self.1);
        set_colors(output, &fg, &bg)?;
        output.queue(MoveTo(x+w-1, y+h-1))?.queue(Print(&bottom_right))?;

        // Draw contained element
        set_colors(output, &None, &self.1.bg())?;

        engine.area = [x+1,y+1,w-2,h-2];
        self.2.render(output, engine)
    }

}

fn set_colors (out: &mut impl Write, fg: &Option<Color>, bg: &Option<Color>) -> Result<()> {
    out.queue(ResetColor)?;
    if let Some(fg) = fg {
        out.queue(SetForegroundColor(*fg))?;
    }
    if let Some(bg) = bg {
        out.queue(SetBackgroundColor(*bg))?;
    }
    Ok(())
}

/// A set of colors to use for rendering a border.
pub trait BorderTheme {
    /// The color outside the box
    fn out (&self) -> Option<Color> { None }
    /// The background of the box
    fn bg  (&self) -> Option<Color> { None }
    /// One border color.
    fn hi  (&self) -> Option<Color>;
    /// The other border color.
    fn lo  (&self) -> Option<Color>;
}

/// Colors for an inset grey border.
pub struct Inset;

impl BorderTheme for Inset {
    fn bg (&self) -> Option<Color> {
        Some(Color::AnsiValue(235))
    }
    fn hi (&self) -> Option<Color> {
        Some(Color::AnsiValue(240))
    }
    fn lo (&self) -> Option<Color> {
        Some(Color::AnsiValue(16))
    }
}

/// Colors for an outset grey border.
pub struct Outset;

impl BorderTheme for Outset {
    fn bg (&self) -> Option<Color> {
        Some(Color::AnsiValue(235))
    }
    fn hi (&self) -> Option<Color> {
        Some(Color::AnsiValue(16))
    }
    fn lo (&self) -> Option<Color> {
        Some(Color::AnsiValue(240))
    }
}

/// A border character, and its foreground and background colors.
pub type BorderChar = (char, Option<Color>, Option<Color>);

/// A set of characters to use for rendering a border.
pub trait BorderStyle {
    fn top (theme: &impl BorderTheme) -> BorderChar;
    fn top_left (theme: &impl BorderTheme) -> BorderChar;
    fn top_right (theme: &impl BorderTheme) -> BorderChar;
    fn left (theme: &impl BorderTheme) -> BorderChar;
    fn right (theme: &impl BorderTheme) -> BorderChar;
    fn bottom (theme: &impl BorderTheme) -> BorderChar;
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar;
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar;
}

/// A border with more vertical space.
pub struct Tall;

impl BorderStyle for Tall {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.lo())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.hi(), theme.bg())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.lo())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.hi(), theme.bg())
    }
}

/// A border with more horizontal space.
pub struct Wide;

impl BorderStyle for Wide {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('▁', theme.lo(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('▎', theme.lo(), theme.bg())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('▊', theme.bg(), theme.hi())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('▇', theme.bg(), theme.hi())
    }
}

/// A border with the default border characters.
pub struct Flat;

impl BorderStyle for Flat {
    fn top (theme: &impl BorderTheme) -> BorderChar {
        ('─', theme.hi(), theme.bg())
    }
    fn top_left (theme: &impl BorderTheme) -> BorderChar {
        ('┌', theme.hi(), theme.bg())
    }
    fn top_right (theme: &impl BorderTheme) -> BorderChar {
        ('┐', theme.hi(), theme.bg())
    }
    fn left (theme: &impl BorderTheme) -> BorderChar {
        ('│', theme.hi(), theme.bg())
    }
    fn right (theme: &impl BorderTheme) -> BorderChar {
        ('│', theme.hi(), theme.bg())
    }
    fn bottom (theme: &impl BorderTheme) -> BorderChar {
        ('─', theme.hi(), theme.bg())
    }
    fn bottom_left (theme: &impl BorderTheme) -> BorderChar {
        ('└', theme.hi(), theme.bg())
    }
    fn bottom_right (theme: &impl BorderTheme) -> BorderChar {
        ('┘', theme.hi(), theme.bg())
    }
}

#[cfg(test)]
mod test {

    use thatsit::{Area, Widget};

    #[test]
    fn test_borders () {

        use crate::{Border, InsetTall, InsetWide};

        let mut output = Vec::<u8>::new();
        let layout = Border(InsetTall, "foo");
        layout.render(&mut output, Area(0, 0, 5, 5));
        panic!("{}", std::str::from_utf8(&output).unwrap());

        let mut output = Vec::<u8>::new();
        let layout = Border(InsetWide, "foo");
        layout.render(&mut output, Area(0, 0, 5, 5));
        panic!("{}", std::str::from_utf8(&output).unwrap());

    }
}
