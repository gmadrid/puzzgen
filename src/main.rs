use puzzgen::Puzzle;

fn main() {
    let puzzle = Puzzle::builder().size(300.0, 200.0).pieces(15, 10).build();

    // unwrap: making gross assumptions.
    let svg = puzzle.to_svg().unwrap();

    println!("{}", svg);
}
