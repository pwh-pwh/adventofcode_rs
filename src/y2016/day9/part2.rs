fn part2() {
    let input = include_str!("./input.txt");
    let result = decompress(input.trim());
    println!("{}", result);
}

fn decompress(s: &str) -> usize {
    if !s.contains('(') {
        return s.len();
    }
    let mut ret = 0;
    let mut s = s.to_owned();
    while s.contains('(') {
        ret += s.find('(').unwrap();
        let marker_end = s.find(')').unwrap();
        let marker = &s[1..marker_end];
        let marker: Vec<&str> = marker.split('x').collect();
        let ts = s[marker_end + 1..].to_owned();
        ret += decompress(&ts[..marker[0].parse::<usize>().unwrap()])
            * marker[1].parse::<usize>().unwrap();
        // ret += ts[..marker[0].parse::<usize>().unwrap()].len() * marker[1].parse::<usize>().unwrap();
        s = ts[marker[0].parse::<usize>().unwrap()..].to_owned();
    }
    ret += s.len();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part2();
    }
}
