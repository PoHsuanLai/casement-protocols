//! Plain geometry in the compositor's logical coordinates.

/// A position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub struct Point {
    /// Horizontal, rightwards.
    pub x: i32,
    /// Vertical, downwards.
    pub y: i32,
}

impl Point {
    /// A point at `x`, `y`.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// An extent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub struct Size {
    /// Width.
    pub w: u32,
    /// Height.
    pub h: u32,
}

impl Size {
    /// A size of `w` by `h`.
    pub const fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

/// A rectangle: where and how big.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub struct Rect {
    /// Left edge.
    pub x: i32,
    /// Top edge.
    pub y: i32,
    /// Width.
    pub w: u32,
    /// Height.
    pub h: u32,
}

impl Rect {
    /// A rectangle at `x`, `y` of `w` by `h`.
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    /// Its top-left corner.
    pub const fn origin(self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Its extent.
    pub const fn size(self) -> Size {
        Size::new(self.w, self.h)
    }
}
