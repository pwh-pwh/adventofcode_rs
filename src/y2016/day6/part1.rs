
fn part1() {
    let input = include_str!("./input.txt");
    println!("{input}");
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn test() {
        part1();
    }
    
}