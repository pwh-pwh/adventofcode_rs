use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Node {
    x: i32,
    y: i32,
    size: i32,
    used: i32,
    avail: i32,
}

impl Node {
    fn new(x: i32, y: i32, size: i32, used: i32, avail: i32) -> Self {
        Node {
            x,
            y,
            size,
            used,
            avail,
        }
    }
}

fn parse(input: Vec<String>) -> Vec<Node> {
    let re = Regex::new(r"(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)").unwrap();
    input
        .iter()
        .filter_map(|line| {
            re.captures(line).map(|caps| {
                Node::new(
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap(),
                    caps[5].parse().unwrap(),
                )
            })
        })
        .collect()
}

fn part_two(nodes: &Vec<Node>) -> i32 {
    let x_size = nodes.iter().max_by_key(|n| n.x).unwrap().x;
    let y_size = nodes.iter().max_by_key(|n| n.y).unwrap().y;

    let mut hole = None;
    let mut w_start = None;
    let mut grid: Vec<Vec<Option<Node>>> =
        vec![vec![None; (y_size + 1) as usize]; (x_size + 1) as usize];

    // Fill grid with nodes
    for n in nodes {
        grid[n.x as usize][n.y as usize] = Some(*n);
    }

    // Print grid for visualization (optional)
    for x in 0..=x_size {
        for y in 0..=y_size {
            match grid[x as usize][y as usize] {
                Some(n) => {
                    if x == 0 && y == 0 {
                        print!("S");
                    } else if x == x_size && y == 0 {
                        print!("G");
                    } else if n.used == 0 {
                        hole = Some(n);
                        print!("_");
                    } else if n.size > 250 {
                        if w_start.is_none() {
                            w_start = Some(grid[(x - 1) as usize][y as usize].unwrap());
                        }
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                None => print!("."),
            }
        }
        println!();
    }

    // Calculate the result
    let hole = hole.unwrap();
    let w_start = w_start.unwrap();

    let result = (hole.x - w_start.x).abs() + (hole.y - w_start.y).abs();
    let result = result + (w_start.x - x_size).abs() + w_start.y;
    result + 5 * (x_size - 1)
}

#[cfg(test)]
mod tests {
    use crate::y2016::day22::part2::{parse, part_two};
    use std::fs;

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        let nodes = parse(input.lines().map(|s| s.to_string()).collect());

        println!("Part Two = {}", part_two(&nodes));
    }
}
