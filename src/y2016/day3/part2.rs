fn get_triangle(s: &str) -> Vec<i32> {
    return s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut pos = 0;
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(line1) => {
                let line1 = get_triangle(line1);
                let line2 = get_triangle(lines.next().unwrap());
                let line3 = get_triangle(lines.next().unwrap());
                for i in 0..3 {
                    if line1[i] + line2[i] > line3[i]
                        && line2[i] + line3[i] > line1[i]
                        && line3[i] + line1[i] > line2[i]
                    {
                        pos += 1;
                    }
                }
            }
            None => break,
        }
    }
    println!("{}", pos);
}

#[cfg(test)]
mod tests {
    use crate::y2016::day3::part2::part2;

    #[test]
    fn test() {
        part2();
    }
}
