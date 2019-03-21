use crate::geom::Point;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use rand::Rng;

// BUG: horiz/vert nubbins are different sizes. :-(

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

    fn gen_vertices<R>(&mut self, _rng: &R)
    where
        R: rand::Rng,
    {
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

    fn gen_edges<R>(&mut self, rng: &mut R)
    where
        R: rand::Rng,
    {
        assert!(self.edges.is_empty());
        assert!(!self.vertices.is_empty());

        let mut done: HashSet<VertexIndex> = HashSet::default();

        // Seed the list with the upper-left vertex.
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

    fn add_edge<R>(&mut self, rng: &mut R, vi1: VertexIndex, vi2: VertexIndex)
    where
        R: rand::Rng,
    {
        let v1 = min(vi1, vi2);
        let v2 = max(vi1, vi2);

        let is_along_edge = self.is_edge_vertex(vi1) && self.is_edge_vertex(vi2);
        let mut edge = match is_along_edge {
            true => Edge::plain(),
            false => Edge::nubbin(),
        };

        edge.jitter(0.06, rng);

        if rng.gen() {
            edge.mirror_x();
        }

        let start = &self.vertices[self.index_of_vertex(v1)];
        let end = &self.vertices[self.index_of_vertex(v2)];
        edge.transform(*start, *end);

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

    pub fn to_svg(&self) -> std::result::Result<String, std::fmt::Error> {
        let mut svg = "".to_string();

        // TODO: consider using a templating engine instead of this mess.
        write!(
            svg,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.0" "#
        )?;
        write!(svg, r#"viewBox="0 0 {} {}" "#, self.x_mm, self.y_mm)?;
        write!(svg, r#"style="margin: 1em;" "#)?;
        write!(svg, r#"width="{}mm" height="{}mm" "#, self.x_mm, self.y_mm)?;
        write!(svg, ">\n")?;

        write!(
            svg,
            r#"<path fill="none" stroke="black" stroke-width="0.1" d=""#
        )?;

        for ((vi1, vi2), e) in &self.edges {
            let v1 = &self.vertices[self.index_of_vertex(*vi1)];
            let v2 = &self.vertices[self.index_of_vertex(*vi2)];
            write!(svg, "{}", e.svg(*v1, *v2))?;
            write!(svg, "\n")?;
        }

        write!(svg, r#""></path>"#)?;
        write!(svg, "\n")?;
        write!(svg, r#"</svg>"#)?;

        Ok(svg)
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

#[derive(Debug, PartialEq)]
enum Edge {
    Bumpless,

    // (Polarity)
    Bumpy(EdgeDesc),
}

impl Edge {
    fn plain() -> Edge {
        Edge::Bumpless
    }

    fn nubbin() -> Edge {
        Edge::Bumpy(EdgeDesc::unit_edge())
    }

    fn mirror_x(&mut self) {
        match self {
            Edge::Bumpless => {},
            Edge::Bumpy(desc) => {
                desc.nubbin_start = desc.nubbin_start.mirror_x();
                desc.nubbin_end = desc.nubbin_end.mirror_x();
                desc.start_control = desc.start_control.mirror_x();
                desc.end_control = desc.end_control.mirror_x();
                desc.left_nubbin_control = desc.left_nubbin_control.mirror_x();
                desc.right_nubbin_control = desc.right_nubbin_control.mirror_x();
            }
        }
    }

    fn jitter<R>(&mut self, max: f32, rng: &mut R) where R: Rng {
        match self {
            Edge::Bumpless => {},
            Edge::Bumpy(desc) => {
                desc.nubbin_start = desc.nubbin_start.jitter(max / 2.0, rng);
                desc.nubbin_end = desc.nubbin_end.jitter(max / 2.0, rng);
                desc.start_control = desc.start_control.jitter(max, rng);
                desc.end_control = desc.end_control.jitter(max, rng);
                desc.left_nubbin_control = desc.left_nubbin_control.jitter(max, rng);
                desc.right_nubbin_control = desc.right_nubbin_control.jitter(max, rng);
            }
        }
    }

    fn transform(&mut self, start: Point, end: Point) {
        match self {
            Edge::Bumpless => {}
            Edge::Bumpy(desc) => {
                desc.nubbin_start = Edge::transform_point(desc.nubbin_start, start, end);
                desc.nubbin_end = Edge::transform_point(desc.nubbin_end, start, end);
                desc.start_control =
                    Edge::transform_point(desc.start_control, start, end);
                desc.left_nubbin_control =
                    Edge::transform_point(desc.left_nubbin_control, start, end);
                desc.right_nubbin_control =
                    Edge::transform_point(desc.right_nubbin_control, start, end);
                desc.end_control =
                    Edge::transform_point(desc.end_control, start, end);
            }
        }
    }

    fn transform_point(pt: Point, start: Point, end: Point) -> Point {
        let rise = end.y() - start.y();
        let run = end.x() - start.x();
        let theta = rise.atan2(run);
        let dist = start.dist(end);
        pt.scale(dist, dist).rotate_by(theta).translate_to(start)
    }

    fn svg(&self, start: Point, end: Point) -> String {
        let d = |pt: Point| format!("{} {}", pt.x(), pt.y());
        match self {
            Edge::Bumpless => format!("M {} L {} ", d(start), d(end)),
            Edge::Bumpy(desc) => format!(
                "M {} C {} {} {} S {} {}  {} {} ",
                d(start),
                d(desc.start_control),  // p1
                d(desc.left_nubbin_control),// p2
                d(desc.nubbin_start),        // p3
                d(desc.right_nubbin_control),// p5
                d(desc.nubbin_end),          // p6
                d(desc.end_control), // p8
                d(end),
            ),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum EdgePolarity {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct EdgeDesc {
    polarity: EdgePolarity,

    nubbin_start: Point,
    nubbin_end: Point,

    start_control: Point,
    left_nubbin_control: Point,
    right_nubbin_control: Point,
    end_control: Point,
}

impl EdgeDesc {
    fn unit_edge() -> EdgeDesc {
        EdgeDesc {
            polarity: EdgePolarity::Left,

            // Along with the start/end points of the edge, these are the endpoints of the
            // three beziers that make up the nubbin.
            nubbin_start: pt!(0.4, 0.1),
            nubbin_end: pt!(0.6, 0.1),

            // Control points for the beziers that make up the nubbin.
            start_control: pt!(0.2, 0),
            left_nubbin_control: pt!(0.5, -0.1),
            right_nubbin_control: pt!(0.7, 0.3),
            end_control: pt!(0.8, 0),
        }
    }
}
