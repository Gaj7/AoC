use std::io;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    // Reads until EOF
    io::stdin().read_to_string(&mut input);

    let mut doubles = 0;
    let mut triples = 0;
    for line in input.lines() {
        let mut map = HashMap::new();
        for c in line.chars() {
            // Get a mutable reference to the entry (default to 0 if it didn't exist) and add 1
            *map.entry(c.to_string()).or_insert(0) += 1;
        }
        // Count doubles and triples
        if map.iter().any(|(_, &v)| v == 2) {doubles += 1;}
        if map.iter().any(|(_, &v)| v == 3) {triples += 1;}
    }

    println!("checksum: {}", doubles*triples);
}
