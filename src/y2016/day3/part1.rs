use std::fs::File;
use std::io::Read;

fn part1() {
    let mut ct = String::default();
    let _ = File::open("src/y2016/day3/input.txt").unwrap().read_to_string(&mut ct);
    let count = ct.lines().filter(|line| {
        is_triangle(line)
    }).count();
    println!("count: {count}");
}

/*fn is_triangle(str: &str) -> bool {
    let mut num_list = Vec::new();
    let mut flag = false;
    let mut sb = String::default();
    for i in str.chars() {
        if i == ' ' {
            if flag {
                flag = false;
                num_list.push(sb.parse::<usize>().unwrap());
                sb.clear();
            }
            continue;
        }
        if flag {
            sb.push(i);
        } else {
            sb.clear();
            sb.push(i);
        }
        flag = true;
    }
    num_list.push(sb.parse::<usize>().unwrap());
    num_list[0] + num_list[1] > num_list[2] &&
        num_list[2] + num_list[1] > num_list[0] &&
        num_list[0] + num_list[2] > num_list[1]
}*/

fn is_triangle(s: &str) -> bool {
    let sides: Vec<usize> = s
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect();

    if sides.len() != 3 {
        return false;
    }

    sides[0] + sides[1] > sides[2] &&
        sides[1] + sides[2] > sides[0] &&
        sides[0] + sides[2] > sides[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}