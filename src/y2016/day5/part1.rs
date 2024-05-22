use std::ops::Add;

fn part1() {
    let input = "abc";
    let mut result = String::new();
    let mut num = 0;
    println!("part1");
    for i in 0..8 {
        loop {
            let digest = md5::compute(format!("{input}{num}"));
            if digest.starts_with(&[0;2]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}