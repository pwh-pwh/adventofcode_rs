fn part1() {
    let input = include_str!("./input.txt");
    let count = input.lines().filter(|line| is_abba(line)).count();
    println!("{count}");
}

fn is_abba(slice: &str) -> bool {
    let mut in_hypernet = false;
    let mut valid = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(4) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] != window[1] && window[1] == window[2] && window[0] == window[3] {
            if in_hypernet {
                return false;
            } else {
                valid = true;
            }
        }
    }

    valid
}

fn is_support_tls(line: &str) -> bool {
    let cs = line.chars().enumerate()
        .collect::<Vec<(usize, char)>>();
    // state 0 -> init -1 -> unable 1 -> valid
    let mut state = 0;
    cs.windows(4)
        .for_each(|iter| {
            if state == -1 {
                return;
            }
            let [i1, i2, i3, i4] = iter else { todo!() };
            if i1.1 == i4.1 && i2.1 == i3.1 && i1.1 != i2.1 {
                let valid = is_valid(line, i1.0);
                if valid {
                    state = 1;
                } else {
                    state = -1;
                }
            }
        });
    if state == 1 {
        true
    } else {
        false
    }
}

fn is_valid(line: &str, index: usize) -> bool {
    let cs: Vec<char> = line.chars().collect();
    for i in (0..index).rev() {
        let c = cs[i];
        if c == '[' {
            return false;
        }
        if c == ']' {
            break;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let valid = is_valid("ffe[ee]sss", 2);
        println!("{}", valid);
    }

    #[test]
    fn test() {
        part1();
    }
}