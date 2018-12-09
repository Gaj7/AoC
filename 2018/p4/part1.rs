use std::io;
use std::io::Read;

enum GuardEvent { WakeUp, FallAsleep, BeginShift(usize) }

struct TimedEvent {
    year:   usize,
    month:  usize,
    day:    usize,
    hour:   usize,
    second: usize,
    event:  GuardEvent,
}

// Assumes string in the following format:
// [year-month-day hour:second] event
// In contrast to my previous solutions, this parse function is actually safe. It won't panic, but
// rather returns the None option when the string is not formatted as expected. This leads to a lot
// of boilerplate code. The `?` operator fixes a lot of this, but unfortunately I need to return
// either a Result or an Option. The former would allow me to use `?` on `parse` calls, and the
// latter on `iter.next()`s.
fn parse_line(s : &str) -> Option<TimedEvent> {
    let mut iter  = s.split_whitespace();
    let date_str  = iter.next()?;
    let time_str  = iter.next()?;
    let event_str = iter.next()?;

    let year  = match  (&date_str[1..5]).parse::<usize>() {Ok(n) => n, Err(_) => return None};
    let month = match  (&date_str[6..8]).parse::<usize>() {Ok(n) => n, Err(_) => return None};
    let day   = match (&date_str[9..11]).parse::<usize>() {Ok(n) => n, Err(_) => return None};

    let hour   = match (&time_str[0..2]).parse::<usize>() {Ok(n) => n, Err(_) => return None};
    let second = match (&time_str[3..5]).parse::<usize>() {Ok(n) => n, Err(_) => return None};

    let event =
        if event_str == "wakes" {GuardEvent::WakeUp} else
        if event_str == "falls" {GuardEvent::FallAsleep} else
        if event_str == "Guard" {
            GuardEvent::BeginShift(
                match (&(iter.next()?)[1..]).parse::<usize>() {Ok(n) => n, Err(_) => return None}
            )
        } else {return None};

    Some(TimedEvent {
        year:   year,
        month:  month,
        day:    day,
        hour:   hour,
        second: second,
        event:  event,
    })
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input);

    // let parsed_input : Vec<_> = input.lines().map(parse_line).collect();
    let parsed_input = input.lines().map(parse_line);

    for event in parsed_input.map(|opt| opt.unwrap()) {
        
    }

    // print!("{}", input);
}
