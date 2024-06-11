use std::collections::HashMap;

fn part1() {
    let data = include_str!("./input.txt");
    let instructions = data
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let mut registers = HashMap::from([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);
    let mut ip = 0i32;
    loop {
        if ip as usize >= instructions.len() {
            break;
        }
        let ins = &instructions[ip as usize];
        if ins[0] == "cpy" {
            registers.insert(ins[2], read(ins[1], &registers));
        } else if ins[0] == "inc" {
            registers.entry(ins[1]).and_modify(|e| *e += 1);
        } else if ins[0] == "dec" {
            registers.entry(ins[1]).and_modify(|e| *e -= 1);
        } else if ins[0] == "jnz" {
            if read(ins[1], &registers) != 0 {
                ip += read(ins[2], &registers);
                ip -= 1;
            }
        }
        ip += 1;
    }
    println!("a value: {}", registers.get("a").unwrap());
}

fn read(v: &str, regs: &HashMap<&str, i32>) -> i32 {
    if let Ok(v) = v.parse::<i32>() {
        v
    } else {
        *regs.get(v).unwrap()
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
