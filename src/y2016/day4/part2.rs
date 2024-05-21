use std::collections::HashMap;

fn part2() {
    let input = include_str!("./input.txt");
    input.lines().for_each(|line| encode(line))
}

const C_STR: &'static str = "North Pole objects";

fn encode(str: &str) {
    let left = str.find('[').unwrap();
    let g_left = str.rfind('-').unwrap();
    let ns = &str[g_left + 1..left];
    let id: i32 = ns.parse().unwrap();
    let origin_str = &str[0..g_left - 1];
    // println!("{origin_str}");
    let mut sb = String::new();
    for x in origin_str.as_bytes() {
        if (*x) as char == '-' {
            sb.push(' ');
            continue;
        }
        sb.push(br(*x as char, id as usize));
    }
    // println!("sb: {sb}");
    if sb.starts_with("north") {
        println!("id: {id}");
    }
}

fn br(c: char, num: usize) -> char {
    // println!("c :{}",c as u8);
    let mut nc = (c as usize) + (num % 26);
    // println!("nc :{}",nc as u8);
    if nc > 'z' as usize {
        nc -= 26;
    }
    // println!("af nc :{}",nc as u8);
    (nc as u8) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part2();
    }

    #[test]
    fn testBr() {
        let r = br('q', 343);
        println!("r: {r}");
    }
}
