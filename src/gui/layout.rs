use rltk::{Point, Rect};

#[cfg(feature = "default")]
pub const MAIN_CONSOLE_DIMENSION: Point = Point::constant(100, 50);
#[cfg(feature = "default")]
pub const ADDITIONAL_CONSOLE_DIMENSION: Point = Point::constant(200, 50);

#[cfg(feature = "default")]
pub const MAIN_VIEW_POSITION: Rect = Rect {
    x1: 3,
    x2: MAIN_CONSOLE_DIMENSION.x - 4,
    y1: 3,
    y2: 40,
};
pub const STATIC_GUI: Rect = Rect {
    x1: 0,
    x2: ADDITIONAL_CONSOLE_DIMENSION.x,
    y1: MAIN_VIEW_POSITION.y2,
    y2: ADDITIONAL_CONSOLE_DIMENSION.y,
};

#[cfg(feature = "tablet")]
pub const MAIN_CONSOLE_DIMENSION: Point = Point::constant(60, 40);
#[cfg(feature = "tablet")]
pub const ADDITIONAL_CONSOLE_DIMENSION: Point = Point::constant(120, 40);

#[cfg(feature = "tablet")]
pub const MAIN_VIEW_POSITION: Rect = Rect {
    x1: 3,
    x2: MAIN_CONSOLE_DIMENSION.x - 4,
    y1: 3,
    y2: 25,
};
