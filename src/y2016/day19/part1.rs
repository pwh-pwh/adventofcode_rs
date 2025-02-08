fn part1() {
    println!("{}", winning_elf(3012210));
}

fn part2() {
    println!("{}", winning_elf2(3012210));
}

fn winning_elf(n: u32) -> u32 {
    // 计算不超过 n 的最大 2 的幂对应的指数 m
    let m = ((n as f64).log2().floor()) as u32;
    // 计算 2^m
    let power = 2u32.pow(m);
    // 计算 L = n - 2^m
    let L = n - power;
    // 返回结果 2 * L + 1
    2 * L + 1
}

fn winning_elf2(n: u32) -> u32 {
    let mut i = 1;
    while i * 3 < n {
        i *= 3;
    }
    n - i
}

#[cfg(test)]
mod tests {
    use crate::y2016::day19::part1::{part1, part2};

    #[test]
    fn test_part1() {
        part1();
    }
    
    #[test]
    fn test_part2() {
        part2();
    }
}