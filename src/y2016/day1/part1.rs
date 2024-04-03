fn sayHi() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        sayHi();
    }

}