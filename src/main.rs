use puzzgen::Puzzle;

fn main() {
    let puzzle = Puzzle::builder()
        .size(225.0, 150.0)
        .pieces(12, 8)
        .build();

    let svg = puzzle.to_svg();

    println!("{}", svg);
}
