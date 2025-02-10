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

fn swap_position(x: usize,y: usize,word: &str) -> String {
    let mut word:Vec<char> = word.chars().collect();
    word.swap(x,y);
    word.iter().collect()
}

fn swap_letter(x: char,y: char,word: &str) -> String {
    word.replace(x, "?").replace(y, &x.to_string()).replace('?', &y.to_string())
}

fn rotate(direction:&str,distance: usize,word: &str) -> String {
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
}