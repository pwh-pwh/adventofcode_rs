// src/y2016/day24/part1.rs

use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input.txt");

pub fn part1() {
    let grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // 收集所有数字位置，用 Vec 保证索引 0 永远对应数字 0
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut pos_of: HashMap<usize, (usize, usize)> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            if let Some(digit) = grid[r][c].to_digit(10) {
                let id = digit as usize;
                pos_of.insert(id, (r, c));
            }
        }
    }

    // 确保数字 0 在 points[0]
    let max_id = *pos_of.keys().max().unwrap();
    points.resize(max_id + 1, (0, 0)); // 先占位

    for (&id, &pos) in &pos_of {
        points[id] = pos;
    }

    let n = points.len(); // 例如 0~7 → n = 8

    // BFS 计算每对关键点之间的最短距离
    let mut dist = vec![vec![0usize; n]; n];
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..n {
        let (sr, sc) = points[i];
        let mut visited = vec![vec![false; cols]; rows];
        let mut q = VecDeque::new();

        q.push_back((sr as i32, sc as i32, 0));
        visited[sr][sc] = true;

        while let Some((r, c, step)) = q.pop_front() {
            let (r, c) = (r as usize, c as usize);

            // 如果当前格子是数字，记录距离
            if grid[r][c].is_digit(10) {
                let j = grid[r][c].to_digit(10).unwrap() as usize;
                if i != j {
                    dist[i][j] = step;
                }
            }

            for &(dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32
                    && !visited[nr as usize][nc as usize]
                    && grid[nr as usize][nc as usize] != '#'
                {
                    visited[nr as usize][nc as usize] = true;
                    q.push_back((nr, nc, step + 1));
                }
            }
        }
    }

    // 状态压缩 DP
    const INF: usize = usize::MAX / 2;
    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[1][0] = 0; // 只访问了 0，位置在 0，步数 0

    for mask in 0..1 << n {
        for u in 0..n {
            if dp[mask][u] == INF { continue; }
            for v in 0..n {
                if (mask & (1 << v)) != 0 { continue; } // v 还没访问
                let nmask = mask | (1 << v);
                dp[nmask][v] = dp[nmask][v].min(dp[mask][u] + dist[u][v]);
            }
        }
    }

    let full = (1 << n) - 1;
    //part1
    // let answer = dp[full].iter().copied().min().unwrap();

    // println!("Part 1 最少步数 = {}", answer);

    let answer = dp[full].iter().enumerate()
        .map(|(i, &d)| d + dist[i][0])
        .min()
        .unwrap();
    println!("Part 2 最少步数（回到0）= {}", answer);

}

#[cfg(test)]
mod tests {
    use crate::y2016::day24::part1::part1;

    #[test]
    fn test_part1() {
        part1()
    }
}
