use puzzgen::Puzzle;

fn main() {
    let puzzle = Puzzle::builder()
        .size(300.0, 200.0)
        .pieces(30, 20)
        .build();
}
