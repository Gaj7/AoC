use std::io;
use std::io::Read;
use std::cmp;

fn main() {
    let mut input = String::new();

    // Reads until EOF
    io::stdin().read_to_string(&mut input);

    let mut grid: Vec<Vec<usize>> = Vec::new();
    // insert(&mut grid, 4, 6, 2);

    let mut count = 0;
    for line in input.lines() {
        let (id, x, y, width, len) = parse_line(line);
        for i in x..(x+width) {
            for j in y..(y+len) {
                if insert(&mut grid, i, j) { count += 1; }
            }
        }
    }

    println!("{}", count);
}

// Flexible insert resizes as necessary
// Returns true if we should count an overlap (only true on first overlap on that space)
fn insert(grid: &mut Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let len = grid.len();
    if len <= x {
        grid.resize(cmp::max(len*2, x+1), Vec::new());
    }
    let len = grid[x].len();
    if len <= y {
        grid[x].resize(cmp::max(len*2, y+1), 0);
    }

    let ret = grid[x][y] == 1;
    grid[x][y] = grid[x][y]+1;

    ret
}

// Returns (id, x, y, width, length)
fn parse_line(s: &str) -> (usize, usize, usize, usize, usize) {
    let mut iter = s.split_whitespace();
    let id_str = iter.next().unwrap();
    let _ = iter.next(); // move past @
    let pos_str = iter.next().unwrap();
    let size_str = iter.next().unwrap();

    let id: usize = (&id_str[1..]).parse().unwrap();
    let (x_str, y_str) = pos_str.split_at(pos_str.find(',').unwrap());
    let x: usize = x_str.parse().unwrap();
    let y: usize = (&y_str[1..(y_str.len()-1)]).parse().unwrap();
    let (width_str, len_str) = size_str.split_at(size_str.find('x').unwrap());
    let width: usize = width_str.parse().unwrap();
    let len: usize = (&len_str[1..]).parse().unwrap();

    (id, x, y, width, len)
}
