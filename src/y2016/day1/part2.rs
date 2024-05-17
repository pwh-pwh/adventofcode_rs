use std::collections::HashSet;
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
    North,
    East,
    South,
    West,
}

impl From<i32> for Azimuth {
    fn from(value: i32) -> Self {
        match (value + 4) % 4 {
            0 => Azimuth::North,
            1 => Azimuth::East,
            2 => Azimuth::South,
            _ => Azimuth::West,
        }
    }
}

impl Azimuth {
    fn to_number(&self) -> i32 {
        match self {
            Azimuth::North => 0,
            Azimuth::East => 1,
            Azimuth::South => 2,
            Azimuth::West => 3,
        }
    }
}

#[derive(Debug)]
struct State {
    location: Location,
    azimuth: Azimuth,
    firstLocation: Option<Location>,
    storage: HashSet<Location>,
}

fn part2() {
    let mut ct = String::new();
    let _ = File::open("src/y2016/day1/input.txt")
        .unwrap()
        .read_to_string(&mut ct);
    let goList = ct
        .split(", ")
        .map(|item| Go::from_str(item).unwrap())
        .collect::<Vec<Go>>();
    println!("{goList:?}");
    let mut state = State {
        location: (0, 0),
        azimuth: Azimuth::North,
        firstLocation: None,
        storage: HashSet::new(),
    };
    goList.iter().for_each(|item| {
        state.go(item);
    });
    let location = state.location;
    println!("location: {location:?}");
    let first_location = state.firstLocation;
    println!("first location: {first_location:?}");
}

impl State {
    // todo 方向
    fn turn(&mut self, dir: &Direction) {
        match dir {
            Direction::Left => self.azimuth = (self.azimuth.to_number() - 1).into(),
            Direction::Right => self.azimuth = (self.azimuth.to_number() + 1).into(),
        }
    }

    fn doLog(&mut self) {
        if self.firstLocation == None && self.storage.contains(&self.location) {
            self.firstLocation = Some(self.location);
        }
        self.storage.insert(self.location);
    }

    fn run(&mut self, step: usize) {
        let step = step as i32;
        match self.azimuth {
            Azimuth::North => {
                for i in 1..=step {
                    self.location.1 += 1;
                    self.doLog();
                }
            }
            Azimuth::East => {
                for i in 1..=step {
                    self.location.0 += 1;
                    self.doLog();
                }
            }
            Azimuth::South => {
                for i in 1..=step {
                    self.location.1 -= 1;
                    self.doLog();
                }
            }
            Azimuth::West => {
                for i in 1..=step {
                    self.location.0 -= 1;
                    self.doLog();
                }
            }
        }
    }

    fn go(&mut self, go: &Go) {
        self.turn(&go.direction);
        self.run(go.step);
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
        part2();
    }
}
