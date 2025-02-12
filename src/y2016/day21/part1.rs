fn part1() {
    let input = include_str!("./input.txt");
    let mut password = String::from("abcdefgh");
    input.lines().for_each(|line| {
        let info: Vec<&str> = line.split_whitespace().collect();
        match info[0] {
            "swap" => {
                if info[1] == "position" {
                    let x = info[2].parse::<usize>().unwrap();
                    let y = info[5].parse::<usize>().unwrap();
                    password = swap_position(x, y, &password);
                } else {
                    let x = info[2].chars().next().unwrap();
                    let y = info[5].chars().next().unwrap();
                    password = swap_letter(x, y, &password);
                }
            }
            "rotate" => {
                if info[1] == "based" {
                    let x = info[6].chars().next().unwrap();
                    password = rotate_position(x, &password);
                } else {
                    let direction = info[1];
                    let distance = info[2].parse::<usize>().unwrap();
                    password = rotate(direction, distance, &password);
                }
            }
            "reverse" => {
                let x = info[2].parse::<usize>().unwrap();
                let y = info[4].parse::<usize>().unwrap();
                password = reverse_positions(x, y, &password);
            }
            "move" => {
                let x = info[2].parse::<usize>().unwrap();
                let y = info[5].parse::<usize>().unwrap();
                password = move_position(x, y, &password);
            }
            _ => {
                println!("this shouldn't happen");
            }
        }
    });
    println!("{}", password);
}

fn part2() {
    let input = include_str!("./input.txt");
    let p = String::from("abcdefgh");
    let mut begin = String::new();
    for mut password in get_permutations(&p) {
        begin = password.clone();
        input.lines().for_each(|line| {
            let info: Vec<&str> = line.split_whitespace().collect();
            match info[0] {
                "swap" => {
                    if info[1] == "position" {
                        let x = info[2].parse::<usize>().unwrap();
                        let y = info[5].parse::<usize>().unwrap();
                        password = swap_position(x, y, &password);
                    } else {
                        let x = info[2].chars().next().unwrap();
                        let y = info[5].chars().next().unwrap();
                        password = swap_letter(x, y, &password);
                    }
                }
                "rotate" => {
                    if info[1] == "based" {
                        let x = info[6].chars().next().unwrap();
                        password = rotate_position(x, &password);
                    } else {
                        let direction = info[1];
                        let distance = info[2].parse::<usize>().unwrap();
                        password = rotate(direction, distance, &password);
                    }
                }
                "reverse" => {
                    let x = info[2].parse::<usize>().unwrap();
                    let y = info[4].parse::<usize>().unwrap();
                    password = reverse_positions(x, y, &password);
                }
                "move" => {
                    let x = info[2].parse::<usize>().unwrap();
                    let y = info[5].parse::<usize>().unwrap();
                    password = move_position(x, y, &password);
                }
                _ => {
                    println!("this shouldn't happen");
                }
            }
        });
        if password == "fbgdceah" {
            break;
        }
    }
    println!("{}", begin);
}

fn get_permutations(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect(); // 将字符串转换为字符向量
    permute(&chars, 0, &mut result); // 调用递归函数生成排列
    result
}

// 递归生成排列的函数
fn permute(chars: &Vec<char>, start: usize, result: &mut Vec<String>) {
    if start == chars.len() {
        result.push(chars.iter().collect()); // 当所有字符都排列完成时，收集结果
        return;
    }

    for i in start..chars.len() {
        let mut chars = chars.clone(); // 克隆当前字符向量
        chars.swap(start, i); // 交换当前字符与后续字符
        permute(&chars, start + 1, result); // 递归排列剩余的部分
    }
}

fn swap_position(x: usize, y: usize, word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word.swap(x, y);
    word.iter().collect()
}

fn swap_letter(x: char, y: char, word: &str) -> String {
    word.replace(x, "?")
        .replace(y, &x.to_string())
        .replace('?', &y.to_string())
}

fn rotate(direction: &str, distance: usize, word: &str) -> String {
    let len = word.len();
    let distance = distance % len;
    if direction == "right" {
        let right_part = &word[len - distance..];
        let left_part = &word[..len - distance];
        format!("{}{}", right_part, left_part)
    } else {
        let left_part = &word[distance..];
        let right_part = &word[..distance];
        format!("{}{}", left_part, right_part)
    }
}

fn rotate_position(letter: char, word: &str) -> String {
    let index = word.find(letter).unwrap();
    let distance = if index >= 4 { index + 2 } else { index + 1 };
    rotate("right", distance, word)
}

fn reverse_positions(x: usize, y: usize, word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word[x..=y].reverse();
    word.into_iter().collect()
}

fn move_position(x: usize, y: usize, word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    let letter = word.remove(x);
    word.insert(y, letter);
    word.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        part1()
    }

    #[test]
    fn test_part2() {
        part2()
    }

    #[test]
    fn test_get_permutations() {
        println!("{:?}", get_permutations("abc"));
    }
}
