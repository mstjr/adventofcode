struct Game{
    guard: Guard,
    obstacles: Vec<(usize, usize)>,
    visited: Vec<(usize, usize)>,
    matrix: Vec<Vec<char>>,
}

struct Guard{
    pos: (usize, usize),
    direction: Direction,
    start: (usize, usize),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new(input: &str) -> Self {
        let matrix = to_matrix(input);//start is on ^ character
        let start = matrix.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &cell)| {
                if cell == '^' {
                    Some((x, y))
                } else {
                    None
                }
            })
        }).unwrap();

        let guard = Guard {
            pos: start,
            direction: Direction::Up,
            start,
        };
        let obstacles = Self::get_obstacles(&matrix);
        let visited = vec![start];
        Self {
            guard,
            obstacles,
            visited,
            matrix,
        }
    }

    fn get_obstacles(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
        let mut obstacles = Vec::new();
        for (y, row) in matrix.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '#' {
                    obstacles.push((x, y));
                }
            }
        }
        obstacles
    }

    fn move_guard(&mut self) -> bool {
        let next;
        match self.guard.direction {
            Direction::Up => next = (self.guard.pos.0, self.guard.pos.1 - 1),
            Direction::Down => next = (self.guard.pos.0, self.guard.pos.1 + 1),
            Direction::Left => next = (self.guard.pos.0 - 1, self.guard.pos.1),
            Direction::Right => next = (self.guard.pos.0 + 1, self.guard.pos.1),
        }

        if next.0 < 0 || next.1 < 0 || next.0 >= self.matrix[0].len() || next.1 >= self.matrix.len() {
            return false;
        }

        let next_char = self.matrix[next.1][next.0];
        if next_char == '#' {
            self.guard.direction = match self.guard.direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            return true;
        }

        if self.visited.contains(&next) {
            self.guard.pos = next;
            return true;
        }

        self.visited.push(next);
        self.guard.pos = next;
        true
    }

    fn get_guard_pos(&self) -> (usize, usize) {
        self.guard.pos
    }

    fn get_visited(&self) -> &Vec<(usize, usize)> {
        &self.visited
    }
}

impl Guard {
    fn new(pos: (usize, usize), direction: Direction) -> Self {
        Self {
            pos,
            direction,
            start: pos,
        }
    }
}


fn main(){
    let input = include_str!("../../input.txt");
    let mut game = Game::new(input);

    let start = std::time::Instant::now();
    while game.move_guard() {
        println!("{:?}", game.get_guard_pos());
    }

    println!("{:?}", game.get_visited().len());
    print!("Time: {}ms", start.elapsed().as_millis());
}

fn to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
