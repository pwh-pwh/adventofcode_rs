use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn part1() {
    let mut d_nodes = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    for line in include_str!("./input.txt").lines() {
        if !line.starts_with('/') {
            continue;
        }

        let captures: Vec<usize> = re
            .find_iter(&line)
            .map(|mat| mat.as_str().parse().unwrap())
            .collect();

        if captures.len() >= 6 {
            let (x, y, _size, used, _avail, _perc) = (
                captures[0],
                captures[1],
                captures[2],
                captures[3],
                captures[4],
                captures[5],
            );
            d_nodes.insert(
                (x, y),
                Node {
                    used,
                    avail: _avail,
                    dist: usize::MAX,
                    prev: None,
                },
            );
        }
    }

    let lx = d_nodes.keys().map(|&key| key.0).max().unwrap() + 1;
    let ly = d_nodes.keys().map(|&key| key.1).max().unwrap() + 1;

    // Puzzle 1 - Count viable pairs
    let mut cnt = 0;
    let values: Vec<&Node> = d_nodes.values().collect();

    for i in 0..values.len() {
        for j in i + 1..values.len() {
            if values[i].used != 0 && values[i].used <= values[j].avail {
                cnt += 1;
            }
            if values[j].used != 0 && values[j].used <= values[i].avail {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    d_nodes: &mut HashMap<(usize, usize), Node>,
    lx: usize,
    ly: usize,
    obst: Option<(usize, usize)>,
) -> Vec<(usize, usize)> {
    // Reset distances and previous nodes
    for node in d_nodes.values_mut() {
        node.dist = usize::MAX;
        node.prev = None;
    }

    // BFS
    let mut queue = VecDeque::new();
    queue.push_back(start);
    d_nodes.get_mut(&start).unwrap().dist = 0;

    while let Some(n) = queue.pop_front() {
        let (x, y) = n;
        if y < 1 || x < 1 {
            continue;
        }
        for (dx, dy) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if dx < lx
                && dy < ly
                && d_nodes.get(&(dx, dy)).unwrap().used < 100
                && Some((dx, dy)) != obst
            {
                if d_nodes.get(&(dx, dy)).unwrap().dist > d_nodes.get(&(x, y)).unwrap().dist + 1 {
                    d_nodes.get_mut(&(dx, dy)).unwrap().dist =
                        d_nodes.get(&(x, y)).unwrap().dist + 1;
                    d_nodes.get_mut(&(dx, dy)).unwrap().prev = Some((x, y));
                    queue.push_back((dx, dy));
                }
                if (dx, dy) == end {
                    let mut path = vec![(dx, dy)];
                    while let Some(prev) = d_nodes.get(&path.last().unwrap()).unwrap().prev {
                        path.push(prev);
                    }
                    return path.into_iter().rev().skip(1).collect(); // Reverse and skip the start
                }
            }
        }
    }
    vec![]
}

#[derive(Debug, Clone)]
struct Node {
    used: usize,
    avail: usize,
    dist: usize,
    prev: Option<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use crate::y2016::day22::part1::part1;

    #[test]
    fn test_part1() {
        part1()
    }
}
