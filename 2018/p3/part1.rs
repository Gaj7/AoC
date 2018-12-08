use std::io;
use std::io::Read;
use std::cmp;

fn main() {
    let mut input = String::new();

    // Reads until EOF
    io::stdin().read_to_string(&mut input);

    let mut grid: Vec<Vec<u32>> = Vec::new();
    insert(&mut grid, 4, 6, 2);

    println!("{}", input);
}

// Flexible insert resizes as necessary
// Returns true if there was already a non-zero element at (x, y)
fn insert(grid: &mut Vec<Vec<u32>>, x: usize, y: usize, elem: u32) -> bool {
    let len = grid.len();
    if len <= x {
        grid.resize(cmp::max(len*2, x+1), Vec::new());
    }
    let len = grid[x].len();
    if len <= y {
        grid[x].resize(cmp::max(len*2, y+1), 0);
    }

    let non_zero = grid[x][y] != 0;
    grid[x][y] = elem;

    non_zero
}

// Returns (id, x, y, width, length)
fn parseLine(s: &str) -> (usize, usize, usize, usize, usize) {
    let mut iter = s.split_whitespace();
    let Some(id_str) = iter.next();
    let _ = iter.next(); // move past @
    let Some(pos_str) = iter.next();
    let Some(size_str) = iter.next();

    let id: usize = &id_str[1..].parse().unwrap();

    let mut iter =

    (id, x, y, width, len)
}
