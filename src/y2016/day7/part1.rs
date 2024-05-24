
fn part1() {
    let input = include_str!("./input.txt");
    let size = input.lines().count();
    println!("size: {size}");
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test() {
        part1();
    }
}