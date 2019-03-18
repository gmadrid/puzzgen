use crate::geom::Point;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use rand::Rng;

pub struct Puzzle {
    x_mm: f32,
    y_mm: f32,
    x_pieces: usize,
    y_pieces: usize,

    vertices: Vec<Point>,

    // Map from vertex indices to edges.
    edges: HashMap<(VertexIndex, VertexIndex), Edge>,
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
            edges: HashMap::default(),
        };

        let mut rng = rand::thread_rng();

        puzzle.gen_vertices(&rng);
        puzzle.gen_edges(&mut rng);

        assert_eq!(
            (puzzle.x_pieces + 1) * (puzzle.y_pieces + 1),
            puzzle.vertices.len()
        );
        assert_eq!(
            (puzzle.x_pieces + 1) * puzzle.y_pieces + puzzle.x_pieces * (puzzle.y_pieces + 1),
            puzzle.edges.len()
        );

        puzzle
    }

    fn index_of_vertex(&self, vi: VertexIndex) -> usize {
        vi.row * (self.x_pieces + 1) + vi.col
    }

    fn gen_vertices<R>(&mut self, rng: &R) where R:rand::Rng {
        assert!(self.vertices.is_empty());

        let piece_width = self.x_mm / self.x_pieces as f32;
        let piece_height = self.y_mm / self.y_pieces as f32;

        for y in 0..self.y_pieces + 1 {
            for x in 0..self.x_pieces + 1 {
                let vertex = Point::new(x as f32 * piece_width, y as f32 * piece_height);
                debug_assert_eq!(
                    self.vertices.len(),
                    self.index_of_vertex(VertexIndex::new(y, x))
                );
                self.vertices.push(vertex);
            }
        }
    }

    fn gen_edges<R>(&mut self, rng: &mut R) where R:rand::Rng {
        assert!(self.edges.is_empty());
        assert!(!self.vertices.is_empty());

        let mut done: HashSet<VertexIndex> = HashSet::default();

        // Seed the 'todo' list with the upper-left vertex.
        let mut todo: HashSet<VertexIndex> = HashSet::default();
        todo.insert(VertexIndex::new(0, 0));

        while !todo.is_empty() {
            // Unwrap: tested for empty.
            let current = *todo.iter().next().unwrap();
            todo.remove(&current);

            for neighbor in self
                .neighbors(&current)
                .into_iter()
                .filter(|vi| !done.contains(vi))
            {
                todo.insert(neighbor);
                self.add_edge(rng, current, neighbor);
            }

            let inserted = done.insert(current);
            assert!(inserted);
        }
    }

    fn add_edge<R>(&mut self, rng: &mut R, vi1: VertexIndex, vi2: VertexIndex) where R: rand::Rng {
        let v1 = min(vi1, vi2);
        let v2 = max(vi1, vi2);

        let is_along_edge = self.is_edge_vertex(vi1) && self.is_edge_vertex(vi2);
        let edge = match is_along_edge {
            true => Edge::Bumpless,
            false => Edge::Bumpy(rng.gen()),
        };
        self.edges.insert((v1, v2), edge);
    }

    fn is_edge_vertex(&self, vi: VertexIndex) -> bool {
        vi.row == 0 || vi.row == self.y_pieces || vi.col == 0 || vi.col == self.x_pieces
    }

    fn neighbors(&self, vi: &VertexIndex) -> Vec<VertexIndex> {
        let mut neighbors = Vec::default();
        if vi.row > 0 {
            neighbors.push(VertexIndex::new(vi.row - 1, vi.col));
        }
        if vi.row < self.y_pieces {
            neighbors.push(VertexIndex::new(vi.row + 1, vi.col));
        }
        if vi.col > 0 {
            neighbors.push(VertexIndex::new(vi.row, vi.col - 1));
        }
        if vi.col < self.x_pieces {
            neighbors.push(VertexIndex::new(vi.row, vi.col + 1));
        }
        neighbors
    }

    pub fn to_svg(&self) -> String {
        let mut svg = "".to_string();

        // TODO: consider using a templating engine instead of this mess.
        write!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.0" "#
        );
        write!(svg, r#"viewBox="0 0 {} {}" "#, self.x_mm, self.y_mm);
        write!(svg, r#"style="margin: 1em;" "#);
        write!(svg, r#"width="{}mm" height="{}mm" "#, self.x_mm, self.y_mm);
        write!(svg, ">\n");

        write!(
            svg,
            r#"<path fill="none" stroke="black" stroke-width="0.1" d=""#
        );

        for ((vi1, vi2), e) in &self.edges {
            let v1 = &self.vertices[self.index_of_vertex(*vi1)];
            let v2 = &self.vertices[self.index_of_vertex(*vi2)];
            write!(svg, "{}", self.edge_svg(v1, v2, &e));
            write!(svg, "\n");
        }

        write!(svg, r#""></path>"#);
        write!(svg, "\n");
        write!(svg, r#"</svg>"#);

        svg
    }

    fn edge_svg(&self, vi1: &Point, vi2: &Point, e: &Edge) -> String {
        match e {
            Edge::Bumpless => format!(r#"M {} {} L {} {}"#, vi1.x(), vi1.y(), vi2.x(), vi2.y()),
            Edge::Bumpy(_) => {
                let one_third_x = (vi2.x() - vi1.x()) / 3.0;
                let one_third_y = (vi2.y() - vi1.y()) / 3.0;
                let one_fifth_x = (vi2.y() - vi1.y()) / 5.0;
                let one_fifth_y = (vi2.x() - vi1.x()) / 5.0;
                if one_third_x == 0.0 {
                    // vertical
                    format!(
                        r#"M {} {} L {} {} L {} {} L {} {} L {} {} L {} {}"#,
                        vi1.x(),
                        vi1.y(),
                        vi1.x(),
                        vi1.y() + one_third_y,
                        vi1.x() + one_fifth_x * e.polarity_factor(),
                        vi1.y() + one_third_y,
                        vi1.x() + one_fifth_x * e.polarity_factor(),
                        vi1.y() + 2.0 * one_third_y,
                        vi1.x(),
                        vi1.y() + 2.0 * one_third_y,
                        vi2.x(),
                        vi2.y()
                    )
                } else {
                    format!(
                        r#"M {} {} L {} {} L {} {} L {} {} L {} {} L {} {}"#,
                        vi1.x(),
                        vi1.y(),
                        vi1.x() + one_third_x,
                        vi1.y(),
                        vi1.x() + one_third_x,
                        vi1.y() + one_fifth_y * e.polarity_factor(),
                        vi1.x() + 2.0 * one_third_x,
                        vi1.y() + one_fifth_y * e.polarity_factor(),
                        vi1.x() + 2.0 * one_third_x,
                        vi1.y(),
                        vi2.x(),
                        vi2.y(),
                    )
                }
                //                format!(r#"M {} {} L {} {}"#, vi1.x(), vi1.y(), vi2.x(), vi2.y())
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

#[derive(Eq, PartialOrd, PartialEq, Ord, Copy, Clone, Debug, Hash)]
struct VertexIndex {
    row: usize,
    col: usize,
}

impl VertexIndex {
    fn new(row: usize, col: usize) -> VertexIndex {
        VertexIndex { row, col }
    }
}

#[derive(Debug)]
enum Edge {
    Bumpless,

    // (Polarity)
    Bumpy(bool),
}

impl Edge {
    fn polarity(&self) -> bool {
        use Edge::*;

        match self {
            Bumpless => true,
            Bumpy(polarity) => *polarity,
        }
    }

    fn polarity_factor(&self) -> f32 {
        if self.polarity() {
            1.0
        } else {
            -1.0
        }
    }
}
