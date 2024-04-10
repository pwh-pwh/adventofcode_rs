use std::fs::File;
use std::io::Read;

fn sayHi() {
    let mut ct = String::new();
    let _ = File::open("src/y2016/day1/input.txt").unwrap().read_to_string(&mut ct);
    println!("{ct}");
    println!("hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        sayHi();
    }
}