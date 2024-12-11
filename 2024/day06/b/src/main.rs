use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

struct LoopFinder {
    matrix: Vec<Vec<char>>,
    start: (usize, usize),
    width: usize,
    height: usize,
}

impl LoopFinder {
    fn new(input: &str) -> Self {
        let matrix = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
        let width = matrix[0].len();
        let height = matrix.len();
        let start = matrix.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| {
                if cell == '^' { Some((x, y)) } else { None }
            })
        }).expect("No start position found");

        Self { matrix, start, width, height }
    }

    fn is_valid_obstacle(&self, x: usize, y: usize) -> bool {
        // Can't place obstacle on start or existing obstacles
        self.matrix[y][x] != '^' && self.matrix[y][x] != '#'
    }

    fn check_loop(&self, obstacle_x: usize, obstacle_y: usize) -> bool {
        let mut matrix = self.matrix.clone();
        matrix[obstacle_y][obstacle_x] = '#';

        let mut visited = HashSet::new();
        let mut pos = self.start;
        let mut direction = Direction::Up;

        // Track visited positions to detect loops
        let mut path_visits = vec![pos];

        loop {
            let next = match direction {
                Direction::Up => (pos.0, pos.1.wrapping_sub(1)),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0.wrapping_sub(1), pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };

            // Check bounds
            if next.0 >= self.width || next.1 >= self.height {
                return false;
            }

            let next_char = matrix[next.1][next.0];
            if next_char == '#' {
                direction = match direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                continue;
            }

            let state = State { x: next.0, y: next.1, direction };

            // If we've visited this exact state before, we have a loop
            if visited.contains(&state) {
                return true;
            }

            visited.insert(state);
            pos = next;
            path_visits.push(pos);

            // Prevent infinite loop
            if path_visits.len() > self.width * self.height * 2 {
                return false;
            }
        }
    }

    fn count_loops(&self) -> usize {
        let mut loop_count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                println!("Checking ({}, {})", x, y);
                if self.is_valid_obstacle(x, y) {
                    if self.check_loop(x, y) {
                        loop_count += 1;
                    }
                }
            }
        }

        loop_count
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let finder = LoopFinder::new(input);
    let result = finder.count_loops();
    println!("Number of loop-creating obstacles: {}", result);
}