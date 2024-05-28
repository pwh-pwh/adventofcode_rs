
#[derive(Debug)]
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
#[derive(Debug)]
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
    println!("{action_list:?}");
}

fn parse_rect(line: &str) ->Action {
    let ls = line.split(" ").nth(1).unwrap();
    // todo get x y
    let x_y:Vec<&str> = ls.split("x").collect();
    Action::Rect {
        x: x_y[0].parse().unwrap(),
        y: x_y[1].parse().unwrap(),
    }
}

fn parse_roate(line: &str) -> Action {
    let ls:Vec<&str> = line.split(" ").collect();
    let lt = if ls[1].eq("column") { 
        LineType::Column
    } else {
        LineType::Row
    };
    let line = ls[2].split("=").collect::<Vec<&str>>()[1].parse().unwrap();
    Action::Roate {
        line,
        step: ls[4].parse().unwrap(),
        line_type: lt,
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