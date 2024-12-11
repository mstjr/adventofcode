fn main() {
    let start = std::time::Instant::now();

    let input = include_str!("../../input.txt");
    let matrix = to_matrix(input);
    let count = count_x_mas_occurrences(matrix);
    println!("{}", count);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 1700μs
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_x_mas_occurrences(grid: Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for x in 0..rows-1 {
        for y in 0..cols-1 {
            if(y < 1 || y >= cols-1) {
                continue;
            }

            if(x < 1 || x >= rows-1) {
                continue;
            }

            let cell = grid[x][y];
            if cell != 'A' {
                continue;
            }

            /*
            M.S
            .A.
            M.S
             */

            let mut first_diagonal = String::new();
            let mut second_diagonal = String::new();

            first_diagonal.push(grid[x-1][y-1]);
            first_diagonal.push(grid[x][y]);
            first_diagonal.push(grid[x+1][y+1]);

            second_diagonal.push(grid[x-1][y+1]);
            second_diagonal.push(grid[x][y]);
            second_diagonal.push(grid[x+1][y-1]);

            if (first_diagonal == "MAS" || first_diagonal == "SAM") && (second_diagonal == "MAS" || second_diagonal == "SAM") {
                count += 1;
            }
        }
    }

    count
}
