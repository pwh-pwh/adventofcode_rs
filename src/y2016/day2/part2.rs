use std::fs::File;
use std::io::{BufRead, BufReader};

// 坐标
type Location = (usize, usize);

/*
    1
  2 3 4
5 6 7 8 9
  A B C
    D
*/

const MATRIX: [[i32; 5]; 5] = [
    [0, 0, 13, 0, 0],
    [0, 10, 11, 12, 0],
    [5, 6, 7, 8, 9],
    [0, 2, 3, 4, 0],
    [0, 0, 1, 0, 0],
];

fn part2() {
    let mut result: Vec<i32> = vec![];
    let mut location = (0, 2);
    let file = File::open("src/y2016/day2/input.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        let str = line.unwrap();
        println!("str: {str} loc: {location:?}");
        moveLoc(&mut location, str);
        println!("after move loc: {location:?}");
        let num = MATRIX[location.1][location.0];
        println!("num: {num}");
        result.push(num);
    });
    println!("result: {result:?}");
}

fn moveLoc(location: &mut Location, str: String) {
    str.chars().for_each(|c| match c {
        'D' => {
            let y = location.1;
            if y != 0 && MATRIX[location.0][y - 1] != 0 {
                location.1 -= 1;
            }
        }
        'U' => {
            let y = location.1;
            if y != 4 && MATRIX[location.0][y + 1] != 0 {
                location.1 += 1;
            }
        }
        'L' => {
            let x = location.0;
            if x != 0 && MATRIX[x - 1][location.1] != 0 {
                location.0 -= 1;
            }
        }
        'R' => {
            let x = location.0;
            if x != 4 && MATRIX[x + 1][location.1] != 0 {
                location.0 += 1;
            }
        }
        _ => (),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part2();
    }
}
