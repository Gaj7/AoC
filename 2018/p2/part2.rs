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
        let mut zipped = head.chars().zip(line.chars());
        // Fold takes a function of type b -> a -> b rather than a -> b -> b  :(
        // (or in the case of try_fold, b -> a -> Try<b>)
        // try_fold stops at first error (None for Option)
        let diffs = zipped.try_fold((0, 0), |(diffs, pos), (c1, c2)| {
            if c1 != c2 {
                if diffs == 1 { None } else { Some((diffs+1, pos)) }
            } else {
                Some((diffs, pos+1))
            }
        });

        if let Some((1, i)) = diffs {
            return Some(String::from(&head[0..i]) + &head[i+1..]);
        }
    }

    // Apparently Rust doesn't do tail recursion optimization.
    // This should be refactored into a loop
    foo(tail)
}
