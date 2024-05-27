
enum Action {
    Rect {
        x: i32,
        y: i32
    },
    Roate {
        line: i32,
        step: i32,
        line_type: LineType
    }
}

enum LineType {
    Row,Column
}

fn part1() {

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        part1();
    }
}