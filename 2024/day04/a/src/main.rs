fn main() {
    let start = std::time::Instant::now();

    let input = include_str!("../../input.txt");
    let matrix = to_matrix(input);
    let count = count_xmas_occurrences(matrix);
    println!("{}", count);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 15000μs
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_xmas_occurrences(grid: Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let word = "XMAS";
    let word_len = word.len();

    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let is_word_in_direction = |row: usize, col: usize, row_delta: isize, col_delta: isize| -> bool {
        for i in 0..word_len {
            let new_row = row as isize + i as isize * row_delta;
            let new_col = col as isize + i as isize * col_delta;

            if new_row < 0 || new_row >= rows as isize || new_col < 0 || new_col >= cols as isize {
                return false;
            }

            if grid[new_row as usize][new_col as usize] != word.chars().nth(i).unwrap() {
                return false;
            }
        }
        true
    };

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            for &(row_delta, col_delta) in &directions {
                if is_word_in_direction(row, col, row_delta, col_delta) {
                    count += 1;
                }
            }
        }
    }

    count
}
