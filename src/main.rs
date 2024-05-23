use std::fmt::{self, Display};

/// Tiles are used to construct a board. The board is a grid of tiles the player needs to navigate.
#[derive(PartialEq, Eq)]
enum Tiles {
    Empty,
    Hole,
    Finish,
    Start,
    Player,
}

struct Board {
    tiles: Vec<Tiles>,
    width: usize,
    height: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y * self.width + x];
                let c = match tile {
                    Tiles::Empty => '.',
                    Tiles::Hole => 'O',
                    Tiles::Finish => 'F',
                    Tiles::Start => 'S',
                    Tiles::Player => 'P',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        // TODO: do I know the size, do I need a vec or can I use an array?
        let mut tiles = Vec::new();
        for _ in 0..width * height {
            tiles.push(Tiles::Empty);
        }

        tiles[0] = Tiles::Start;
        tiles[width * height - 1] = Tiles::Finish;

        let n_holes = (width * height) / 10;
        for _ in 0..n_holes {
            let mut index = rand::random::<usize>() % (width * height);
            // If the tile is not empty, create a new index and try again.
            while tiles[index] != Tiles::Empty {
                index = rand::random::<usize>() % (width * height);
            }
            tiles[index] = Tiles::Hole;
        }

        return Board {
            tiles,
            width,
            height,
        };
    }
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
enum State {
    Playing,
    Win,
    Lose,
}

enum Actions {
    Up,
    Down,
    Left,
    Right,
}

impl Actions {
    fn sample() -> Self {
        match rand::random::<usize>() % 4 {
            0 => Actions::Up,
            1 => Actions::Down,
            2 => Actions::Left,
            3 => Actions::Right,
            _ => panic!("Invalid action"),
        }
    }
}

// TODO: impl Display for Env
struct Env {
    board: Board,
    agent: Position,
    state: State,
}

impl Env {
    fn new(width: usize, height: usize) -> Self {
        let board = Board::new(width, height);
        let agent = Position { x: 0, y: 0 };
        let state = State::Playing;
        Env {
            board,
            agent,
            state,
        }
    }

    fn step(&mut self, action: Actions) {
        if self.state != State::Playing {
            return;
        }

        let (dx, dy) = match action {
            Actions::Up => (0, -1),
            Actions::Down => (0, 1),
            Actions::Left => (-1, 0),
            Actions::Right => (1, 0),
        };

        let x = self.agent.x as isize + dx;
        let y = self.agent.y as isize + dy;

        if x < 0 || x >= self.board.width as isize || y < 0 || y >= self.board.height as isize {
            return;
        }

        let new_index = y as usize * self.board.width + x as usize;
        let new_tile = &self.board.tiles[new_index];

        if *new_tile == Tiles::Hole {
            self.state = State::Lose;
            return;
        }

        if *new_tile == Tiles::Finish {
            self.state = State::Win;
            return;
        }

        self.agent.x = x as usize;
        self.agent.y = y as usize;
    }
}

fn main() {
    let env = Env::new(10, 10);
}
