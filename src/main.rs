use std::io::{self, Write};
use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use rand::prelude::*;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'x', long, default_value_t = 20)]
    width: usize,

    #[arg(short = 'y', long, default_value_t = 20)]
    height: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    position: (i32, i32),
}

impl Food {
    fn new() -> Self {
        Food { position: (10, 10) }
    }
}

struct Snake {
    direction: Direction,
    positions: Vec<(i32, i32)>,
}

impl Snake {
    fn new() -> Self {
        Snake {
            direction: Direction::Right,
            positions: vec![(12, 10)],
        }
    }
}

struct Game {
    grid: (usize, usize),
    game_over: bool,
    snake: Snake,
    food: Food,
    score: i32,
}

impl Game {
    fn new(grid_size: (usize, usize)) -> Self {
        Game {
            grid: grid_size,
            game_over: false,
            snake: Snake::new(),
            food: Food::new(),
            score: 0,
        }
    }

    fn get_final_score(&self) -> i32 {
        self.score
    }
}

fn draw_grid(game: &Game) {
    let width = game.grid.0;
    let height = game.grid.1;
    let food = &game.food;
    let snake = &game.snake;

    execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    let mut grid_chars = vec![vec![' '; width]; height];

    for x in 0..width {
        // top border
        grid_chars[0][x] = '/';
        // bottom border
        grid_chars[height - 1][x] = '/';
    }

    for y in 0..height {
        grid_chars[y][0] = '/';
        grid_chars[y][width - 1] = '/';
    }

    if food.position.0 > 0
        && food.position.0 < width as i32 - 1
        && food.position.1 > 0
        && food.position.1 < height as i32 - 1
    {
        let x = food.position.0 as usize;
        let y = food.position.1 as usize;
        grid_chars[y][x] = '*';
    }

    for &(x, y) in &snake.positions {
        if x > 0 && x < width as i32 - 1 && y > 0 && y < height as i32 - 1 {
            let x = x as usize;
            let y = y as usize;
            grid_chars[y][x] = '#';
        }
    }

    print!("Score: {}\r\n", game.score);

    for row in grid_chars {
        for ch in row {
            print!("{}", ch);
        }
        print!("\r\n");
    }
}

fn update_game(game: &mut Game) {
    let (dx, dy) = match game.snake.direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    let current_head = game.snake.positions[0];
    let next_head = (current_head.0 + dx, current_head.1 + dy);

    if next_head.0 <= 0
        || next_head.0 >= game.grid.0 as i32 - 1
        || next_head.1 <= 0
        || next_head.1 >= game.grid.1 as i32 - 1
    {
        game.game_over = true;
        return;
    }

    if game.snake.positions[1..]
        .iter()
        .any(|&pos| pos == next_head)
    {
        game.game_over = true;
        return;
    }

    if next_head == game.food.position {
        game.snake.positions.insert(0, next_head);
        game.score += 1;

        let mut rng = rand::rng();
        let x = rng.random_range(1..game.grid.0 - 1) as i32;
        let y = rng.random_range(1..game.grid.1 - 1) as i32;
        game.food.position = (x, y);
    } else {
        game.snake.positions.insert(0, next_head);
        game.snake.positions.pop();
    }
}

fn main() {
    let args = Args::parse();
    let mut game = Game::new((args.width, args.height));

    enable_raw_mode().expect("failed to enable raw mode");

    while !game.game_over {
        draw_grid(&game);
        io::stdout().flush().unwrap();

        if event::poll(Duration::from_millis(300)).expect("failed to poll for event") {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Up => {
                        if !matches!(game.snake.direction, Direction::Down) {
                            game.snake.direction = Direction::Up;
                        }
                    }
                    KeyCode::Down => {
                        if !matches!(game.snake.direction, Direction::Up) {
                            game.snake.direction = Direction::Down;
                        }
                    }
                    KeyCode::Left => {
                        if !matches!(game.snake.direction, Direction::Right) {
                            game.snake.direction = Direction::Left;
                        }
                    }
                    KeyCode::Right => {
                        if !matches!(game.snake.direction, Direction::Left) {
                            game.snake.direction = Direction::Right;
                        }
                    }
                    KeyCode::Char('q') => {
                        game.game_over = true;
                    }
                    _ => {}
                }
            }
        }

        update_game(&mut game);
    }

    io::stdout().flush().unwrap();
    disable_raw_mode().expect("failed to disable raw mode");

    print!(
        "\nGame Over!\nYour final is score: {}\nThanks for playing.\n",
        game.get_final_score()
    );
}
