
fn part2() {
    let input = include_str!("./input.txt");
    // println!("{input}");
    let mut record = [[0;26];8];
    input.lines().for_each(|line| {
        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            record[i][(bytes[i] - 'a' as u8) as usize] += 1;
        }
    });
    for x in record {
        let max_index = get_min_index(x);
        let c = ((max_index as u8) + 'a' as u8) as char;
        print!("{c}");
    }
}

fn get_min_index(data: [i32;26]) -> usize {
    let mut min_index = 0;
    let mut min_val = i32::MAX;
    for i in 0..26 {
        if data[i] < min_val && data[i] != 0 {
            min_val = data[i];
            min_index = i;
        }
    }
    min_index
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        part2();
    }

}