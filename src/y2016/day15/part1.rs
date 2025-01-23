fn solve_congruences(congruences: Vec<(u64,u64)>)->u64 {
    let mut t = 0;
    let mut step = 1;

    for (remainder, modulus) in congruences {
        while t % modulus != remainder {
            t += step;
        }
        step *= modulus;
    }
    t
}

fn find_earliest_time(discs: Vec<(u64, u64)>) -> u64 {
    let result:Vec<(u64, u64)> = discs.iter().enumerate().map(|(i, (total_pos, start_pos))| {
        let remainder = (total_pos - (start_pos + i as u64 + 1) % total_pos) % total_pos;
        (remainder, *total_pos)
    }).collect::<Vec<_>>();
    solve_congruences(result)
}

/*
# python impl
def find_earliest_time(discs):
    congruences = []
    for i, (total_pos, start_pos) in enumerate(discs):
        remainder = (total_pos - (start_pos + i + 1) % total_pos) % total_pos
        congruences.append((remainder, total_pos))

    # Solve using Chinese Remainder Theorem
    t = solve_congruences(congruences)
    return t

def solve_congruences(congruences):
    """
    求解任意数量的同余方程组 t ≡ r_i (mod m_i)

    :param congruences: 一个列表，每个元素是一个 (remainder, modulus) 的元组
    :return: 最小的 t，使得 t 满足所有同余条件
    """
    t = 0  # 起点
    step = 1  # 初始步长

    for remainder, modulus in congruences:
        # 不断增加 t，直到满足当前条件 t ≡ remainder (mod modulus)
        while t % modulus != remainder:
            t += step
        # 更新步长为当前步长与当前模数的最小公倍数
        step *= modulus

    return t

# Example input
discs = [
    (17, 15),  # (total_positions, start_position)
    (3, 2),
    (19,4),
    (13,2),
    (7,2),
    (5,0),
    (11,0)
]
print(find_earliest_time(discs))  # Output: 5
 */



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        println!("{}", solve_congruences(vec![(0, 5), (1, 2)]));
    }
    
    #[test]
    fn test2() {
        let arg = vec![(17, 15),
                       (3, 2),
                       (19,4),
                       (13,2),
                       (7,2),
                       (5,0),
                       (11,0)];
        println!("{}", find_earliest_time(arg));
    }
}