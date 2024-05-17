use std::fs::File;
use std::io::{BufRead, BufReader};

// 坐标
type Location = (usize, usize);

fn part1() {
    let matrix: [[i32; 3]; 3] = [ [7,8,9],
        [4,5,6],
        [1,2,3]];
    let mut result: Vec<i32> = vec![];
    let mut location = (1, 1);
    let file = File::open("src/y2016/day2/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        let str = line.unwrap();
        println!("str: {str} loc: {location:?}");
        moveLoc(&mut location, str);
        println!("after move loc: {location:?}");
        let num = matrix[location.1][location.0];
        println!("num: {num}");
        result.push(num);
    });
    println!("result: {result:?}");
}

fn moveLoc(location: &mut Location, str: String) {
    str.chars().for_each(|c| {
        match c {
            'D' => {
                let y = location.1;
                if y != 0 {
                    location.1 -= 1;
                }
            }
            'U' => {
                let y = location.1;
                if y != 2 {
                    location.1 += 1;
                }
            }
            'L' => {
                let x = location.0;
                if x != 0 {
                    location.0 -= 1;
                }
            }
            'R' => {
                let x = location.0;
                if x != 2 {
                    location.0 += 1;
                }
            }
            _ => (),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}