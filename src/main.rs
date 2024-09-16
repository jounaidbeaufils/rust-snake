use pancurses::{endwin, initscr, noecho, Input};
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WIDTH: i32 = 40;
const HEIGHT: i32 = 20;
const FRAME_DURATION: Duration = Duration::from_millis(200); // Adjust the snake speed here

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // Initialize the window
    let window = initscr();
    window.nodelay(true); // Non-blocking input
    window.keypad(true);
    noecho();

    // Initialize the snake
    let mut snake = VecDeque::new();
    let start_pos = Position {
        x: WIDTH / 2,
        y: HEIGHT / 2,
    };
    snake.push_back(start_pos);

    // Initial direction
    let mut dir = Direction::Right;

    // Place the first food
    let mut food = generate_food(&snake);

    // Game loop
    let mut score = 0;
    loop {
        let frame_start = Instant::now();

        // Input handling
        match window.getch() {
            Some(Input::KeyLeft) if !matches!(dir, Direction::Right) => dir = Direction::Left,
            Some(Input::KeyRight) if !matches!(dir, Direction::Left) => dir = Direction::Right,
            Some(Input::KeyUp) if !matches!(dir, Direction::Down) => dir = Direction::Up,
            Some(Input::KeyDown) if !matches!(dir, Direction::Up) => dir = Direction::Down,
            Some(Input::Character('q')) => break,
            _ => {}
        }

        // Calculate new head position
        let mut new_head = *snake.front().unwrap();
        match dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        // Check for collisions with walls
        if new_head.x <= 0
            || new_head.x >= WIDTH - 1
            || new_head.y <= 0
            || new_head.y >= HEIGHT - 1
        {
            break;
        }

        // Check for collisions with self
        if snake.contains(&new_head) {
            break;
        }

        // Add new head to the snake
        snake.push_front(new_head);

        // Check for food consumption
        if new_head == food {
            score += 1;
            food = generate_food(&snake);
        } else {
            snake.pop_back(); // Remove tail if no food consumed
        }

        // Rendering
        window.clear();

        // Draw borders
        for x in 0..WIDTH {
            window.mvaddch(0, x, '#');
            window.mvaddch(HEIGHT - 1, x, '#');
        }
        for y in 0..HEIGHT {
            window.mvaddch(y, 0, '#');
            window.mvaddch(y, WIDTH - 1, '#');
        }

        // Draw snake
        for pos in &snake {
            window.mvaddch(pos.y, pos.x, 'O');
        }

        // Draw food
        window.mvaddch(food.y, food.x, '*');

        // Display score
        window.mvprintw(HEIGHT, 0, format!("Score: {}", score));

        window.refresh();

        // Maintain consistent frame rate
        let frame_duration = Instant::now() - frame_start;
        if FRAME_DURATION > frame_duration {
            std::thread::sleep(FRAME_DURATION - frame_duration);
        }
    }

    // End the window
    endwin();
    println!("Game Over! Your score was: {}", score);
}

fn generate_food(snake: &VecDeque<Position>) -> Position {
    let mut rng = rand::thread_rng();
    loop {
        let pos = Position {
            x: rng.gen_range(1..WIDTH - 1),
            y: rng.gen_range(1..HEIGHT - 1),
        };
        if !snake.contains(&pos) {
            return pos;
        }
    }
}
