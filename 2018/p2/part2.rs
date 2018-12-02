use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    match foo(&input) {
        Some(s) => println!("{}", s),
        None    => println!("No matches"),
    }
}

fn foo(s: &str) -> Option<String> {
    let (head, tail) = match s.find('\n') {
        Some(i) => (&s[0..i], &s[i+1..]),
        None    => return None,
    };

    for line in tail.lines() {
        let zipped = head.chars().zip(line.chars()); // Fold is b -> a -> b rather than a -> b -> b :(
        let (diffs, _, i) = zipped.fold((0, 0, 0), |(diffs, pos, i), (c1, c2)| {
            if c1 != c2 {
                (diffs+1, pos+1, pos)
            } else {
                (diffs, pos+1, i)
            }
        });
        if diffs == 1 {
            return Some(String::from(&head[0..i]) + &head[i+1..]);
        }
    }

    foo(tail)
}
