use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn part1() {
    let lines: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(String::from)
        .collect();

    for a in 1..10000 {
        if let Some(_) = adv_main(lines.clone(), a as i32) {
            println!("Day 25 答案是: {}", a);
            break;
        }

        // 可选：进度条
        if a % 50 == 0 {
            print!("\r正在尝试 a = {} ...", a);
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
    }
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


// 只改这几处就行！

pub fn adv_main(input: Vec<String>, init_a: i32) -> Option<Vec<i32>> {
    let mut program: Vec<Vec<String>> = input.iter()
        .map(|s| s.split(' ').map(|s| s.to_string()).collect())
        .collect();

    let mut cpu = [init_a, 0i32, 0i32, 0i32];  // a 从外部传进来！
    let mut pc: i32 = 0;
    let end = program.len() as i32;

    let mut output = Vec::new();  // 新增：记录输出

    while pc < end {
        let pcu = pc as usize;
        let words = program[pcu].clone();

        match words[0].as_ref() {
            "cpy" => {
                if let Some(reg) = parse_reg(&words[2]) {
                    cpu[reg] = parse_value(cpu, &words[1]);
                }
            }
            "inc" => {
                if let Some(reg) = parse_reg(&words[1]) {
                    cpu[reg] += 1;
                }
            }
            "dec" => {
                if let Some(reg) = parse_reg(&words[1]) {
                    cpu[reg] -= 1;
                }
            }
            "jnz" => {
                if parse_value(cpu, &words[1]) != 0 {
                    pc += parse_value(cpu, &words[2]);
                    continue;
                }
            }
            "tgl" => {
                let offset = parse_value(cpu, &words[1]) as isize;
                let target = pcu as isize + offset;
                if target >= 0 && target < program.len() as isize {
                    let instr = program[target as usize][0].clone();
                    program[target as usize][0] = match instr.as_ref() {
                        "dec" | "tgl" => "inc".to_string(),
                        "inc" => "dec".to_string(),
                        "jnz" => "cpy".to_string(),
                        "cpy" => "jnz".to_string(),
                        a => a.to_string(),
                    };
                }
            }
            "out" => {  // Day25 新增指令！
                let val = parse_value(cpu, &words[1]);
                output.push(val);

                // 早停优化：已经看到 100 个 0,1 交替就认为成功
                if output.len() >= 100 {
                    // 检查是否真的是 010101...
                    let expected: Vec<i32> = (0..100).map(|i| i % 2).collect();
                    if output == expected {
                        return Some(output);
                    } else {
                        return None;  // 一旦出错，马上放弃这个 a
                    }
                }
            }
            _ => {}
        }
        pc += 1;

        // 防止死循环（安全网）
        if pc > end + 1000 { return None; }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::y2016::day25::part1::part1;

    #[test]
    fn test_part1() {
        part1()
    }
}
