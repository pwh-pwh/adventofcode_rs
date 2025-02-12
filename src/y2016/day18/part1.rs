fn part1() {
    let mut input = ".^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^...^^.^.^.......^..^^...^.^.^^..^^^^^...^.".to_string();
    let mut count = 0usize;
    count += count_safe_tiles(&input);
    for i in 0..39 {
        input = generate_next(&input);
        count += count_safe_tiles(&input);
    }
    println!("{}", count);
}

fn part2() {
    let mut input = ".^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^...^^.^.^.......^..^^...^.^.^^..^^^^^...^.".to_string();
    let mut count = 0usize;
    count += count_safe_tiles(&input);
    for i in 0..400000 - 1 {
        input = generate_next(&input);
        count += count_safe_tiles(&input);
    }
    println!("{}", count);
}

fn generate_next(input: &str) -> String {
    let mut result = String::new();
    let sl = input.len();
    let char_arr: Vec<char> = input.chars().collect();
    for i in 0..sl {
        let mut isTrap = false;
        if i == 0 {
            if (char_arr[i] == '.' && char_arr[i + 1] == '^')
                || (char_arr[i] == '^' && char_arr[i + 1] == '^')
            {
                isTrap = true;
            }
        } else if i == sl - 1 {
            if (char_arr[i] == '.' && char_arr[i - 1] == '^')
                || (char_arr[i] == '^' && char_arr[i - 1] == '^')
            {
                isTrap = true;
            }
        } else {
            if (char_arr[i] == '^' && char_arr[i + 1] == '^' && char_arr[i - 1] == '.')
                || (char_arr[i] == '^' && char_arr[i - 1] == '^' && char_arr[i + 1] == '.')
                || (char_arr[i] == '.' && char_arr[i + 1] == '.' && char_arr[i - 1] == '^')
                || (char_arr[i] == '.' && char_arr[i - 1] == '.' && char_arr[i + 1] == '^')
            {
                isTrap = true;
            }
        }
        if isTrap {
            result.push('^');
        } else {
            result.push('.');
        }
    }
    // println!("{}", result);
    result
}

fn count_safe_tiles(input: &str) -> usize {
    input.chars().filter(|c| *c == '.').count()
}

#[cfg(test)]
mod tests {
    use crate::y2016::day18::part1::{generate_next, part1, part2};

    #[test]
    fn test_gen() {
        println!("{}", generate_next(".^^^..^.^^"));
    }

    #[test]
    fn test_part1() {
        part1();
    }

    #[test]
    fn test_part2() {
        part2();
    }
}
