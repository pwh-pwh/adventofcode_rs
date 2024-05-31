use std::collections::HashMap;
use std::fs;
use regex::Regex;

fn part1() {
    let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut outputs: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pipeline: HashMap<i32, ((&str, i32), (&str, i32))> = HashMap::new();
    let data = include_str!("./input.txt");
    let instructions = data.lines();

    let re_num = Regex::new(r"-?\d+").unwrap();
    let re_type = Regex::new(r" (bot|output)").unwrap();

    for line in instructions {
        let nums: Vec<i32> = re_num.find_iter(line)
            .map(|mat| mat.as_str().parse::<i32>().unwrap())
            .collect();
        let types: Vec<&str> = re_type.find_iter(line)
            .map(|mat| mat.as_str().trim())
            .collect();

        if line.starts_with("value") {
            bots.entry(nums[1]).or_insert_with(Vec::new).push(nums[0]);
        }
        if line.starts_with("bot") {
            pipeline.insert(nums[0], ((types[0], nums[1]), (types[1], nums[2])));
        }
    }

    while !bots.is_empty() {
        let mut to_remove = Vec::new();
        let mut to_add: Vec<(String, i32, i32)> = Vec::new();

        for (k, v) in &bots {
            if v.len() == 2 {
                let mut sorted_v = v.clone();
                sorted_v.sort();
                let (v1, v2) = (sorted_v[0], sorted_v[1]);
                if v1 == 17 && v2 == 61 {
                    println!("{}", k);
                }
                let ((t1, n1), (t2, n2)) = pipeline[k];
                to_add.push((t1.to_string(), n1, v1));
                to_add.push((t2.to_string(), n2, v2));
                to_remove.push(*k);
            }
        }

        for k in to_remove {
            bots.remove(&k);
        }

        for (t, n, v) in to_add {
            if t == "bot" {
                bots.entry(n).or_insert_with(Vec::new).push(v);
            } else {
                outputs.entry(n).or_insert_with(Vec::new).push(v);
            }
        }
    }

    let a = outputs.get(&0).unwrap()[0];
    let b = outputs.get(&1).unwrap()[0];
    let c = outputs.get(&2).unwrap()[0];
    println!("{}", a * b * c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}