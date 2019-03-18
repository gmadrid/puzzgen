use crate::geom::Point;

pub struct Puzzle {
    x_mm: f32,
    y_mm: f32,
    x_pieces: usize,
    y_pieces: usize,

    vertices: Vec<Point>,
}

impl Puzzle {
    pub fn builder() -> Builder {
        Builder::new()
    }

    fn build(builder: &Builder) -> Puzzle {
        let mut puzzle = Puzzle {
            x_mm: builder.x_mm,
            y_mm: builder.y_mm,
            x_pieces: builder.x_pieces,
            y_pieces: builder.y_pieces,
            vertices: Vec::default(),
        };

        puzzle.gen_vertices();

        puzzle
    }

    fn gen_vertices(&mut self) {
        let piece_width = self.x_mm / self.x_pieces as f32;
        let piece_height = self.y_mm / self.y_pieces as f32;

        for y in 0..self.y_pieces + 1 {
            for x in 0..self.x_pieces + 1 {
                let vertex = Point::new(x as f32 * piece_width, y as f32 * piece_height);
                self.vertices.push(vertex);
            }
        }
    }
}

#[derive(Default)]
pub struct Builder {
    x_mm: f32,
    y_mm: f32,
    x_pieces: usize,
    y_pieces: usize,
}

impl Builder {
    fn new() -> Builder {
        Builder::default()
    }

    pub fn size(mut self, x: f32, y: f32) -> Builder {
        self.x_mm = x;
        self.y_mm = y;
        self
    }

    pub fn pieces(mut self, x: usize, y: usize) -> Builder {
        self.x_pieces = x;
        self.y_pieces = y;
        self
    }

    pub fn build(self) -> Puzzle {
        Puzzle::build(&self)
    }
}
