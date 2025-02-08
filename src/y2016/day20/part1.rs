use std::cmp::max;

fn part1() {
    let input = include_str!("./input.txt");
    let mut ranges: Vec<(i64, i64)> = input.lines().map(|s| {
        let narrs:Vec<_> = s.split("-").collect();
        let start = narrs[0].parse::<i64>().unwrap();
        let end = narrs[1].parse::<i64>().unwrap();
        (start, end)
    }).collect();
    ranges.sort();
    let mut first_ip = 0;
    for (start,end) in ranges {
        if first_ip + 1 >= start {
            first_ip = max(first_ip, end);
        } else {
            first_ip = first_ip + 1;
            break
        }
    }
    println!("{}", first_ip);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1();
    }
}