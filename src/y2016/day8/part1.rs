use std::fmt::{Debug, Display};

#[derive(Debug)]
enum Action {
    Rect {
        x: i32,
        y: i32,
    },
    Roate {
        line: i32,
        step: i32,
        line_type: LineType,
    },
}
#[derive(Debug)]
enum LineType {
    Row,
    Column,
}

const W: usize = 50;
const H: usize = 6;

fn part1() {
    let input = include_str!("./input.txt");
    let mut matrix = [[0; W]; H];
    let action_list: Vec<Action> = input
        .lines()
        .map(|line| {
            if line.starts_with("rect") {
                parse_rect(line)
            } else {
                parse_roate(line)
            }
        })
        .collect();
    action_list.iter().for_each(|action| {
        doChange(action, &mut matrix);
    });
    log_matrix(&matrix);
    let count: i32 = matrix.iter().flat_map(|v| v.iter()).sum();
    println!("{count}");
}

fn log_matrix<const W: usize, const H: usize, T: Debug>(matrix: &[[T; W]; H]) {
    for x in matrix {
        for x in x {
            print!("{x:?} ");
        }
        println!();
    }
}

fn doChange(action: &Action, matrix: &mut [[i32; W]; H]) {
    match action {
        Action::Rect { x, y } => {
            for i in 0..*y {
                for j in 0..*x {
                    matrix[i as usize][j as usize] = 1;
                }
            }
        }
        Action::Roate {
            line,
            step,
            line_type,
        } => match line_type {
            LineType::Row => {
                let mut temp = [0; W];
                for i in 0..W {
                    let index = (i + (*step as usize)) % W;
                    temp[index] = matrix[*line as usize][i];
                }
                for i in 0..W {
                    matrix[*line as usize][i] = temp[i];
                }
            }
            LineType::Column => {
                let mut temp = [0; H];
                for i in 0..H {
                    let index = (i + (*step as usize)) % H;
                    temp[index] = matrix[i][*line as usize];
                }
                for i in 0..H {
                    matrix[i][*line as usize] = temp[i];
                }
            }
        },
    }
}

fn parse_rect(line: &str) -> Action {
    let ls = line.split(" ").nth(1).unwrap();
    // get x y
    let x_y: Vec<&str> = ls.split("x").collect();
    Action::Rect {
        x: x_y[0].parse().unwrap(),
        y: x_y[1].parse().unwrap(),
    }
}

fn parse_roate(line: &str) -> Action {
    let ls: Vec<&str> = line.split(" ").collect();
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
