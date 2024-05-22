use std::ops::Add;
use md5::Digest;

fn part1() {
    let input = "abc";
    let mut result = String::new();
    let mut num = 0;
    println!("part1");
    for i in 0..2 {
        loop {
            let digest = md5::compute(format!("{input}{num}"));
            if is_five_zero(digest) {
                let d = format!("{:x}", (*digest.get(5).unwrap()));
                result.push_str(&d);
                println!("result debug: {result:?}");
                println!("{num}");
                // panic!("err");
                num+=1;
                break
            }
            num += 1;
            // println!("{num}");
        }
    }
    println!("result: {result:?}");
}

fn is_five_zero(hash:Digest) -> bool {
    let hexString = hash.to_vec();
    hexString[0] == 0 && hexString[1] == 0 && hexString[2] == 0 && hexString[3] == 0 && hexString[4] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}