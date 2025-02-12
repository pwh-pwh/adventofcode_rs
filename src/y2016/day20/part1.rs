use std::cmp::max;

fn part1() {
    let input = include_str!("./input.txt");
    let mut ranges: Vec<(i64, i64)> = input
        .lines()
        .map(|s| {
            let narrs: Vec<_> = s.split("-").collect();
            let start = narrs[0].parse::<i64>().unwrap();
            let end = narrs[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort();
    let mut first_ip = 0;
    for (start, end) in ranges {
        if first_ip + 1 >= start {
            first_ip = max(first_ip, end);
        } else {
            first_ip = first_ip + 1;
            break;
        }
    }
    println!("{}", first_ip);
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut ranges: Vec<(i64, i64)> = input
        .lines()
        .map(|s| {
            let narrs: Vec<_> = s.split("-").collect();
            let start = narrs[0].parse::<i64>().unwrap();
            let end = narrs[1].parse::<i64>().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort();
    let mut next = 0;
    let mut count = 0;
    for (start, end) in ranges {
        if start > next {
            count += start - next - 1;
        }
        next = max(next, end);
    }
    count += 2i64.pow(32) - 1 - next;
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }
}
