use std::collections::HashMap;
use std::ops::Index;

fn part1() {
    let input = include_str!("./input.txt");
    let count = input.lines().map(|line| is_real(line)).sum::<i32>();
    println!("count: {count}");
}

//aaaaa-bbb-z-y-x-123[abxyz]
fn is_real(str: &str) -> i32 {
    let left = str.find('[').unwrap();
    let g_left = str.rfind('-').unwrap();
    let ns = &str[g_left+1..left];
    let id: i32 = ns.parse().unwrap();
    let mut cm:HashMap<char,i32> = HashMap::new();
    for c in str[0..left].chars() {
        if c.is_ascii_lowercase() {
            cm.insert(c,*cm.get(&c).unwrap_or(&0) + 1);
        }
    }
    let mut cl:Vec<char> = cm.keys().map(|c|*c).collect();
    cl.sort_by(|c1,c2| {
       //a_count.cmp(&b_count).reverse().then_with(|| a.cmp(b))
        cm.get(c1).unwrap().cmp(cm.get(c2).unwrap())
            .reverse().then_with(||c1.cmp(c2))
    });
    /*cl.sort();
    cl.sort_by_key(|c|-cm.get(c).unwrap());*/
    println!("cl: {cl:?}");
    for i in 0..5 {
        if cl[i] != str.as_bytes()[left + i + 1] as char {
            return 0
        }
    }
    id
}

#[cfg(test)]
mod tests {
    use crate::y2016::day4::part1::part1;

    #[test]
    fn test() {
        part1();
    }
}
