use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

fn part1() {

    let mut d_nodes = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();

    for line in include_str!("./input.txt").lines() {
        if !line.starts_with('/') {
            continue;
        }

        let captures: Vec<usize> = re.find_iter(&line)
            .map(|mat| mat.as_str().parse().unwrap())
            .collect();

        if captures.len() >= 6 {
            let (x, y, _size, used, _avail, _perc) = (captures[0], captures[1], captures[2], captures[3], captures[4], captures[5]);
            d_nodes.insert((x, y), Node { used, avail: _avail });
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

#[derive(Debug)]
struct Node {
    used: usize,
    avail: usize,
}

#[cfg(test)]
mod tests {
    use crate::y2016::day22::part1::part1;

    #[test]
    fn test_part1() {
        part1()
    }
}