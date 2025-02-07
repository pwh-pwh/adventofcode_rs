fn part1() {
    let input = "10111100110001111";
    let fill_len: usize = 35651584;
    let data = fill_data(input, fill_len);
    let r = cal_checksum(&data);
    println!("{}", r);
}

fn fill_data(data: &str, fill_len: usize) -> String {
    let mut a = data.to_string();
    while a.len() < fill_len {
        let b = a
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect::<String>();
        let r = format!("{}0{}", a, b);
        a = r;
    }
    a[..fill_len].to_string()
}

/*
    Consider each pair: 11, 00, 10, 11, 01, 00.
    These are same, same, different, same, different, same, producing 110101.
    The resulting string has length 6, which is even, so we repeat the process.
    The pairs are 11 (same), 01 (different), 01 (different).
    This produces the checksum 100, which has an odd length, so we stop.
Therefore, the checksum for 110010110100 is 100.
 */
fn cal_checksum(data: &str) -> String {
    let mut data = data.to_string();
    loop {
        data = data
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
            .collect();
        if data.len() % 2 != 0 {
            break;
        }
    }
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_data() {
        println!("{}", fill_data("10000", 20));
    }

    #[test]
    fn test_checksum() {
        println!("{}", cal_checksum("10000011110010000111"));
    }

    #[test]
    fn test() {
        part1();
    }
}
