
enum Action {
    Rect {
        x: i32,
        y: i32
    },
    Roate {
        line: i32,
        step: i32,
        line_type: LineType
    }
}

enum LineType {
    Row,Column
}

fn part1() {
    let input = include_str!("./input.txt");
    let mut matrix = [[50;0];6];
    let action_list:Vec<Action> = input.lines().map(|line| {
        if line.starts_with("rect") {
            parse_rect(line)
        } else {
            parse_roate(line)
        }
    }).collect();
}

fn parse_rect(line: &str) ->Action {
    let ls = line.split(" ").nth(2).unwrap();
    // todo get x y
    Action::Rect {
        x: 1,
        y: 1
    }
}

fn parse_roate(line: &str) -> Action {
    Action::Roate {
        line: 1,
        step: 1,
        line_type: LineType::Column,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        part1();
    }
}