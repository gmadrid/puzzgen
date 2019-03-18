use puzzgen::Puzzle;

fn main() {
    let puzzle = Puzzle::builder()
        .size(225.0, 150.0)
        .pieces(15, 10)
        .build();

    let svg = puzzle.to_svg();

    println!("{}", svg);
}
