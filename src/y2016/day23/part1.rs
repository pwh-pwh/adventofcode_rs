use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn part1() {
    let lines: Vec<String> = include_str!("./input.txt")
    .lines()            // 按行切分成 &str
    .map(String::from)  // 转成 String
    .collect();
    adv_main(lines);
}


fn parse_reg(v: &str) -> Option<usize> {
    match v {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),    
        _ => None,
    }
}

fn parse_value(c: [i32; 4], v: &str) -> i32 {
    match v {
        "a" => c[0],
        "b" => c[1],
        "c" => c[2],
        "d" => c[3],
        _ => v.parse().unwrap(),
    }
}


pub fn adv_main(input: Vec<String>) {

    let mut input: Vec<Vec<String>> = input.iter().map(|s| s.split(" ").map(|s| s.to_string() ).collect()).collect();
    let mut cpu = [12i32, 0i32, 0i32, 0i32];

    let mut pc: i32 = 0;
    let end = input.len() as i32;

    while pc < end {
        let pcu = pc as usize; 

        let words = input[pcu].clone();
        //println!("{:?}", words);   

        match words[0].as_ref() {
            "cpy" => {
                let reg = parse_reg(&words[2]);
                if reg.is_none() {
                    continue;
                }

                cpu[ reg.unwrap() ] = parse_value(cpu, &words[1])
            },
            "inc" => {
                let reg = parse_reg(&words[1]);
                if reg.is_none() {
                    continue;
                }
                cpu[ reg.unwrap() ] += 1
            },
            "dec" => {
                let reg = parse_reg(&words[1]);
                if reg.is_none() {
                    continue;
                }

                cpu[ reg.unwrap() ] -= 1
            },
            "jnz" => {
                if parse_value(cpu, &words[1]) != 0 {
                    pc += parse_value(cpu, &words[2]);
                    continue;
                }
            },
            "tgl" => {
                input.get_mut(pcu + (parse_value(cpu, &words[1]) as usize)).map(|v| {
                    let instr = v[0].clone();
                    v[0] = match instr.as_ref() {
                        "dec" | "tgl" => "inc".to_string(),
                        "inc"         => "dec".to_string(),
                        "jnz"         => "cpy".to_string(),
                        "cpy"         => "jnz".to_string(),
                        a             => a.to_string()
                    };
                });
            }
            _ => {},
        }

        pc += 1;
    }

    println!("a: {}", cpu[0]);

}


#[cfg(test)]
mod tests {
    use crate::y2016::day23::part1::part1;

    #[test]
    fn test_part1() {
        part1()
    }
}
