use std::collections::HashMap;
use md5::Digest;
use rustc_serialize::hex::ToHex;

fn part2() {
    let input = "ngcjuoqr";
    let mut num = 0;
    let mut count = 0;
    let mut map = HashMap::new();
    while count < 64 {
        let digest = get_additional_hash(&format!("{}{}", input, num),2017,&mut map);
        num+=1;
        if is_valid(digest.to_hex().as_bytes(),num,input,&mut map) {
            count+=1;
            println!("f n:{num}");
        }

    }
    println!("result: {}",num - 1);
}

fn get_additional_hash(str:&str,total:i32,map:&mut HashMap<String,Digest>) -> Digest {
    if map.contains_key(str) {
        return map.get(str).unwrap().clone();
    }
    let mut dg = md5::compute(str);
    for i in 0..total - 1 {
        dg = md5::compute(dg.to_hex());
    }
    map.insert(str.to_owned(),dg.clone());
    dg
}

fn is_valid(hex_bytes: &[u8],start:i32,input:&str,map:&mut HashMap<String,Digest>) -> bool {
    let mut flag = false;
    let mut is_first = false;
    let mut u:u8 = 0;
    hex_bytes.windows(3)
        .for_each(|us| {
            if is_first {
                return ;
            }
            if (us[0] == us[1])&&(us[1] == us[2]) {
                is_first = true;
                u = us[0];
                flag = is_ct(start,input,u,map);
            }
        });
    flag
}

fn is_ct(start:i32,input:&str,u:u8,map:&mut HashMap<String,Digest>) -> bool {
    let mut flag = false;
    for i in start..start+1000 {
        let digest = get_additional_hash(&format!("{}{}", input, i),2017,map);
        // let digest = md5::compute(format!("{}{}", input, i));
        digest.to_hex().as_bytes()
            .windows(5).for_each(|us| {
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
        part2();
    }
}