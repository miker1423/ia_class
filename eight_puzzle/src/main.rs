use eight_puzzle::{Puzzle, bfs};

fn main() {
    let initial = vec![1, 8, 2, 3, 5, 6, 7, 0, 4];
    let puzzle = Puzzle::new(initial);
    let result = bfs(&puzzle, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

    println!("{:?}", result);
}

