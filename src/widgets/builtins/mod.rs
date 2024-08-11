pub use centered_prompt::CenteredPrompt;
pub use centered_text::CenteredText;
pub use ruler::Ruler;
pub use sweeper::Sweeper;
pub use text::Text;
pub use uv::Uv;
pub use margin::Margin;
pub use centered::Centered;
pub use stacked::Stacked;
pub use buttons::Buttons;
pub use shrink_wrap::ShrinkWrap;
pub use backdrop::Backdrop;
use crate::widgets::BoundingBox;

/// The code for the [`Sweeper`] widget.
pub mod sweeper;
/// The code for the [`CenteredText`] widget.
pub mod centered_text;
/// The code for the [`CenteredPrompt`] widget.
pub mod centered_prompt;
/// The code for the [`Ruler`] widget.
pub mod ruler;
/// The code for the [`Text`] widget.
pub mod text;
/// The code for the [`Uv`] widget.
pub mod uv;
/// Teh code for the [`Margin`] widget.
pub mod margin;
/// The code the [`dummy::Dummy`] widget.
pub mod dummy;
/// The code for the [`Centered`] widget.
pub mod centered;
/// The code for the [`Stacked`] widget.
pub mod stacked;
/// The code for the [`Buttons`] widget.
pub mod buttons;
/// The code for the [`ShrinkWrap`] widget.
pub mod shrink_wrap;
/// The code for the [`Backdrop`] widget.
pub mod backdrop;

impl<T: BoundingBox> From<T> for Centered<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: BoundingBox> From<T> for Margin<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

/// An auto-implemented trait for widgets that provides some convenience methods for layouts.
pub trait WithLayout: Sized {
    /// Adds a margin by the specified distance -- can be negative to expand the widget.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tuit::prelude::*;
    /// use tuit::terminal::ConstantSize;
    /// use tuit::widgets::builtins::Text;
    ///
    /// let text = Text::new("I should be centered!").with_margin(2);
    ///
    /// let mut terminal: ConstantSize<50, 20> = ConstantSize::new();
    ///
    /// text.drawn(&mut terminal).expect("Should draw successfully :)");
    ///
    /// let mut very_tiny_terminal: ConstantSize<5, 5> = ConstantSize::new();
    ///
    /// text.drawn(&mut very_tiny_terminal).expect_err("Should not have enough space.");
    /// ```
    fn with_margin(self, margin: isize) -> Margin<Self> {
        Margin::new(self).margin(margin)
    }

    /// Adds a padding by the specified distance by applying a [`ShrinkWrap`].
    fn with_shrink(self, shrink: usize) -> ShrinkWrap<Self> { ShrinkWrap::new(self).shrink(shrink) }

    /// Centers the widget.
    fn centered(self) -> Centered<Self> {
        Centered::new(self)
    }

    /// Stacks the widget on top of another widget.
    fn on_top_of<T>(self, other: T) -> Stacked<Self, T> {
        Stacked::new(self, other)
    }
}

impl<T: BoundingBox> WithLayout for T {}
