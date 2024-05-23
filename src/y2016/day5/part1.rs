use md5::Digest;
use rustc_serialize::hex::ToHex;
use std::ops::Add;
use std::time::{Instant, SystemTime};

pub fn part1() {
    let input = "cxdnnyjw";
    let mut result = String::new();
    let mut num = 0;
    println!("part1");
    let instant = Instant::now();
    for i in 0..8 {
        loop {
            let digest = md5::compute(format!("{input}{num}").into_bytes().as_slice());
            if let Some(c) = is_five_zero(digest) {
                result.push(c);
                println!("result debug: {result:?}");
                println!("{num}");
                // panic!("err");
                num += 1;
                break;
            }
            num += 1;
            // println!("{num}");
        }
    }
    println!("result: {result:?}");
    let use_secs = instant.elapsed().as_secs();
    println!("run {use_secs} secs");
}

fn is_five_zero(hash: Digest) -> Option<char> {
    let hex = hash.to_hex();
    let hexString = hex.as_bytes();
    if (hexString[0] == 48
        && hexString[1] == 48
        && hexString[2] == 48
        && hexString[3] == 48
        && hexString[4] == 48)
    {
        Some(hexString[5] as char)
    } else {
        None
    }
}

fn is_five_zero_2(hash: Digest) -> Option<char> {
    let hexString = format!("{:x}", hash);
    if hexString.starts_with("00000") {
        Some(hexString.chars().nth(5).unwrap())
    } else {
        None
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
