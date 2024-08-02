use crate::style::Style;
use crate::terminal::{Cell, Metadata, TerminalConst};
use crate::terminal::extended::view_iterator::ViewIterator;
use crate::terminal::TerminalMut;
use crate::widgets::Rectangle;

/// A mutable view into another [`TerminalMut`].
pub struct View<T>
{
    /// The parent terminal containing the characters inside the view
    parent: T,
    /// The default style of the parent terminal
    default_style: Style,
    /// The area that the view draws into.
    rect: Rectangle,
}

impl<T> Metadata for View<T>
where T: TerminalConst {
    fn dimensions(&self) -> (usize, usize) {
        self.rect.dimensions()
    }

    fn default_style(&self) -> Style {
        self.default_style
    }
}

impl<T> TerminalConst for View<T>
where T: TerminalConst {
    fn cells(&self) -> impl Iterator<Item=&Cell> {
        let parent_dimensions = self.parent.dimensions();
        let view_top = self.rect.top();
        let view_left = self.rect.left();
        let cells = self.parent.cells();

        ViewIterator {
            child: cells
                .skip(view_left)
                .skip(view_top * parent_dimensions.0),
            current_coord: (0,0),
            parent_dimensions,
            view_rect: self.rect
        }
    }
}

impl<T> View<T> {
    /// Creates a new [`ViewMut`] from the given [`TerminalMut`] and the left-top
    /// coordinate.
    pub fn new(terminal: T, view_rect: Rectangle) -> Self
    where T: Metadata {
        Self {
            default_style: terminal.default_style(),
            parent: terminal,
            rect: view_rect
        }
    }
}

