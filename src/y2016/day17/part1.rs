use std::usize;

static doors: [&'static str; 4] = ["U", "D", "L", "R"];
static dx: [i32; 4] = [0, 0, -1, 1];
static dy: [i32; 4] = [-1, 1, 0, 0];

static INPUT: &'static str = "udskfozm";

static mut longLen: usize = usize::MIN;

static mut shortestLen: usize = usize::MAX;

static mut shortestPath: String = String::new();

struct State {
    route: String,
    x: i32,
    y: i32,
}

impl State {
    fn isValid(&self) -> bool {
        self.x >= 0 && self.x <= 3 && self.y >= 0 && self.y <= 3
    }

    fn isEnd(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

fn walk(state: State) {
    unsafe {
        if !state.isValid() {
            return;
        }
        if state.isEnd() {
            if state.route.len() > longLen {
                longLen = state.route.len();
            }
            if state.route.len() < shortestLen {
                shortestLen = state.route.len();
                shortestPath = state.route.clone();
            }
            return;
        }
        let hex = format!(
            "{:x}",
            md5::compute(format!("{}{}", INPUT, state.route).as_bytes())
        );
        for i in 0..4 {
            if is_door_open(&hex, i) {
                let mut new_route = state.route.clone();
                new_route.push_str(doors[i]);
                walk(State {
                    x: state.x + dx[i],
                    y: state.y + dy[i],
                    route: new_route,
                });
            }
        }
    }
}

fn is_door_open(hex: &str, i: usize) -> bool {
    hex.as_bytes()[i] > b'a'
}

fn part1() {
    unsafe {
        walk(State {
            x: 0,
            y: 0,
            route: String::new(),
        });
        println!("Longest route: {}", longLen);
        println!("Shortest route: {}", shortestPath);
    }
}

#[cfg(test)]
mod tests {
    use crate::y2016::day17::part1::part1;

    #[test]
    fn test() {
        part1()
    }
}
