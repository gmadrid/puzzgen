use std::fmt::{self, Display, Formatter};
use rand::Rng;

#[macro_export]
macro_rules! pt {
    ($x: expr, $y: expr) => {
        Point::new($x as f32, $y as f32)
    };
}

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

    pub fn dist(self, other: Point) -> f32 {
        ((self.y - other.y).powf(2.0) + (self.x - other.x).powf(2.0)).sqrt()
    }

    pub fn jitter<R>(self, max: f32, rng: &mut R) -> Point where R: Rng {
        let delta_x = rng.gen_range(-max, max);
        let delta_y = rng.gen_range(-max, max);
        pt!(self.x + delta_x, self.y + delta_y)
    }

    pub fn mirror_x(self) -> Point {
        pt!(self.x, -self.y)
    }

    pub fn scale(self, x_factor: f32, y_factor: f32) -> Point {
        pt!(self.x * x_factor, self.y * y_factor)
    }

    pub fn translate_to(self, pt: Point) -> Point {
        Point::new(self.x + pt.x(), self.y + pt.y())
    }

    pub fn rotate_by(self, radians: f32) -> Point {
        Point::new(
            self.x * radians.cos() - self.y * radians.sin(),
            self.x * radians.sin() + self.y * radians.cos(),
        )
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
