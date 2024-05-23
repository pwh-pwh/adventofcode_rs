use std::collections::HashSet;
use md5::Digest;
use rustc_serialize::hex::ToHex;

fn part2() {
    let input = "cxdnnyjw";
    let mut set = HashSet::<u8>::new();
    let mut num = 0;
    let mut result = ['0'; 8];
    while set.len() != 8 {
        let digest = md5::compute(format!("{input}{num}").into_bytes().as_slice());
        if let Some((c, index)) = is_valid(digest, &set) {
            set.insert(index);
            result[index as usize] = c;
        }
        num += 1;
    }
    println!("{result:?}");
}

fn is_valid(hash: Digest, set: &HashSet<u8>) -> Option<(char, u8)> {
    let hex = hash.to_hex();
    let hexString = hex.as_bytes();
    if hexString[5] >= 56 || set.contains(&(hexString[5] - 48)) {
        None
    } else {
        if (hexString[0] == 48
            && hexString[1] == 48
            && hexString[2] == 48
            && hexString[3] == 48
            && hexString[4] == 48)
        {
            Some((hexString[6] as char, hexString[5] - 48))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part2();
    }
}
