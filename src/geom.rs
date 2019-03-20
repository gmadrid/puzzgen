use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    pub fn scale(self, x_factor: f32, y_factor: f32) -> Point {
        Point::new(self.x * x_factor, self.y * y_factor)
    }

    pub fn translate_to(self, pt: Point) -> Point {
        Point::new(self.x + pt.x(), self.y + pt.y())
    }

    pub fn rotate_by(self, radians: f32) -> Point {
        Point::new(self.x * radians.cos() - self.y * radians.sin(),
        self.x * radians.sin() + self.y * radians.cos())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
