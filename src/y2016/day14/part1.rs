use rustc_serialize::hex::ToHex;

fn part1() {
    let input = "ngcjuoqr";
    let mut num = 0;
    let mut count = 0;
    while count < 64 {
        let digest = md5::compute(format!("{}{}", input, num));
        num += 1;
        if is_valid(digest.to_hex().as_bytes(), num, input) {
            count += 1;
            println!("f n:{num}");
        }
    }
    println!("result: {}", num - 1);
}

fn is_valid(hex_bytes: &[u8], start: i32, input: &str) -> bool {
    let mut flag = false;
    let mut is_first = false;
    let mut u: u8 = 0;
    hex_bytes.windows(3).for_each(|us| {
        if is_first {
            return;
        }
        if (us[0] == us[1]) && (us[1] == us[2]) {
            is_first = true;
            u = us[0];
            flag = is_ct(start, input, u);
        }
    });
    flag
}

fn is_ct(start: i32, input: &str, u: u8) -> bool {
    let mut flag = false;
    for i in start..start + 1000 {
        let digest = md5::compute(format!("{}{}", input, i));
        digest.to_hex().as_bytes().windows(5).for_each(|us| {
            if (us[0] == u) && (us[1] == u) && (us[2] == u) && (us[3] == u) && (us[4] == u) {
                flag = true;
            }
        });
    }
    flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}
