
fn part1() {
    println!("Hello World");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        part1();
    }
}