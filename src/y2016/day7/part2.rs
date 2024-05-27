
fn part2() {
    let input = include_str!("./input.txt");
    let count = input.lines().filter(|line|is_aba(line)).count();
    println!("{}", count);
}

fn is_aba(slice: &str) -> bool {
    let (mut abas, mut babs) = (Vec::new(), Vec::new());
    let mut in_hypernet = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(3) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] == window[2] && window[0] != window[1] {
            if in_hypernet {
                babs.push( (window[1], window[0], window[1]) );
            } else {
                abas.push( (window[0], window[1], window[0]) );
            }
        }
    }

    for aba in &abas {
        for bab in &babs {
            if aba == bab {
                return true;
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        part2();
    }
}