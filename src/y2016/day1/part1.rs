use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct Go {
    step: usize,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type Location = (i32, i32);

//方位azimuth
#[derive(Debug)]
enum Azimuth {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
struct State {
    location: Location,
    azimuth: Azimuth,
}

fn sayHi() {
    let mut ct = String::new();
    let _ = File::open("src/y2016/day1/input.txt").unwrap().read_to_string(&mut ct);
    let goList = ct.split(", ").map(|item| {
        Go::from_str(item).unwrap()
    }
    ).collect::<Vec<Go>>();
    println!("{goList:?}");
    let mut state = State{
        location: (0, 0),
        azimuth: Azimuth::North,
    };

}

impl State {
    // todo 方向
    fn turn(&mut self,dir:Direction) {

    }

    fn run(&mut self,step:usize) {

    }

    fn go(&mut self,go: Go) {

    }
}

impl FromStr for Go {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c1 = s.as_bytes()[0] as char;
        let numStr = &s[1..].trim();
        let dir = if c1 == 'R' {
            Direction::Right
        } else {
            Direction::Left
        };
        let step = numStr.parse().unwrap();
        Ok(Self {
            step,
            direction: dir,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        sayHi();
    }
}