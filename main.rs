use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: String::from("Snake Game (Made in Rust)"),
        window_width: 800i32,
        window_height: 600i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn add(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn opposite(dir1: &Direction, dir2: &Direction) -> bool {
        match (dir1, dir2) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }
}

struct Snake {
    body: Vec<Point>,
    dir: Direction,
}

const FPS: f64 = 7f64;
const BACKGROUND_COLOR: Color = GRAY;
const TITLE_TEXT: &str = "snake.rs";
const TITLE_SIZE: u16 = 50;
const TITLE_COLOR: Color = LIME;
const SQUARE_SIZE: f32 = 40f32;
const ROW_SIZE: u8 = 18;
const COLUMN_SIZE: u8 = 11;
const GRID_OFFSET_X: f32 = 40f32;
const GRID_OFFSET_Y: f32 = 120f32;
const SNAKE_COLOR: Color = LIME;
const GAME_OVER_COLOR: Color = BLACK;

static GAME_OVER: AtomicBool = AtomicBool::new(false);
static BEST_SCORE: AtomicU16 = AtomicU16::new(0);
static SCORE: AtomicU16 = AtomicU16::new(0);

fn render_score(font: &Font) {
    let best_dimensions = measure_text(
        format!("Best: {}", BEST_SCORE.load(Ordering::Relaxed)),
        Some(font),
        TITLE_SIZE / 2,
        1.0,
    );
    draw_text_ex(
        format!("Best: {}", BEST_SCORE.load(Ordering::Relaxed)),
        screen_width() - (best_dimensions.width * 1.3),
        0f32 + best_dimensions.height * 1.3,
        TextParams {
            font: Some(font),
            font_size: TITLE_SIZE / 2,
            font_scale: 1f32,
            color: BLACK,
            ..Default::default()
        },
    );

    let normal_dimensions = measure_text(
        format!("Score: {}", SCORE.load(Ordering::Relaxed)),
        Some(font),
        TITLE_SIZE / 2,
        1.0,
    );
    draw_text_ex(
        format!("Score: {}", SCORE.load(Ordering::Relaxed)),
        screen_width() - (normal_dimensions.width * 1.3),
        0f32 + normal_dimensions.height * 2.3 + best_dimensions.height,
        TextParams {
            font: Some(font),
            font_size: TITLE_SIZE / 2,
            font_scale: 1f32,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn spawn_apple(apple: &mut Point) {
    apple.x = rand::gen_range(0, ROW_SIZE as i16);
    apple.y = rand::gen_range(0, COLUMN_SIZE as i16);
}

fn reset_game(snake: &mut Snake, apple: &mut Point) {
    snake.body = vec![Point { x: 0, y: 0 }];
    snake.dir = Direction::None;
    GAME_OVER.store(false, Ordering::Relaxed);
    spawn_apple(apple);
}

fn query_reset_game() -> bool {
    if is_key_pressed(KeyCode::R) {
        true
    } else {
        false
    }
}

fn update_input() -> Direction {
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        Direction::Up
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        Direction::Down
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        Direction::Left
    } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        Direction::Right
    } else {
        Direction::None
    }
}

fn update_snake(snake: &mut Snake, apple: &mut Point) {
    if snake.dir != Direction::None {
        let mut grown: bool = false;
        snake.body.push({
            let offset: Point = match snake.dir {
                Direction::Up => Point { x: 0, y: -1 },
                Direction::Down => Point { x: 0, y: 1 },
                Direction::Left => Point { x: -1, y: 0 },
                Direction::Right => Point { x: 1, y: 0 },
                _ => Point { x: 0, y: 0 },
            };

            let mut head: Point = snake.body[snake.body.len() - 1].clone();
            head.add(&offset);
            if head == *apple {
                spawn_apple(apple);
                grown = true;
                SCORE.fetch_add(1, Ordering::Relaxed);
                if SCORE.load(Ordering::Relaxed) > BEST_SCORE.load(Ordering::Relaxed) {
                    BEST_SCORE.fetch_add(1, Ordering::Relaxed);
                    quad_storage::STORAGE
                        .lock()
                        .expect("Error saving data")
                        .set("best", &BEST_SCORE.load(Ordering::Relaxed).to_string());
                }
            }
            head
        });
        if !grown {
            snake.body.remove(0);
        }
    }
    if snake.body[snake.body.len() - 1].x < 0
        || snake.body[snake.body.len() - 1].x > ROW_SIZE as i16 - 1
        || snake.body[snake.body.len() - 1].y < 0
        || snake.body[snake.body.len() - 1].y > COLUMN_SIZE as i16 - 1
    {
        GAME_OVER.store(true, Ordering::Relaxed);
    }

    let head_unit = snake.body[snake.body.len() - 1].clone();
    let snake_len: usize = snake.body.len();
    for (i, unit) in snake.body.iter_mut().enumerate() {
        if i == snake_len - 1 {
            break;
        }
        if *unit == head_unit {
            GAME_OVER.store(true, Ordering::Relaxed);
            break;
        }
    }
}

fn render(snake: &Snake, apple: &Point) {
    draw_rectangle(
        GRID_OFFSET_X,
        GRID_OFFSET_Y,
        SQUARE_SIZE * ROW_SIZE as f32,
        SQUARE_SIZE * COLUMN_SIZE as f32,
        DARKGRAY,
    );
    for row in 0..ROW_SIZE {
        for col in 0..COLUMN_SIZE {
            if apple.y as u8 == col && apple.x as u8 == row {
                draw_rectangle(
                    GRID_OFFSET_X + SQUARE_SIZE * row as f32,
                    GRID_OFFSET_Y + SQUARE_SIZE * col as f32,
                    SQUARE_SIZE as f32,
                    SQUARE_SIZE as f32,
                    RED,
                );
            }
            for unit in &snake.body {
                if unit.y as u8 == col && unit.x as u8 == row {
                    draw_rectangle(
                        GRID_OFFSET_X + SQUARE_SIZE * row as f32,
                        GRID_OFFSET_Y + SQUARE_SIZE * col as f32,
                        SQUARE_SIZE as f32,
                        SQUARE_SIZE as f32,
                        SNAKE_COLOR,
                    );
                }
            }
        }
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let font: Font = load_ttf_font("assets/Roboto Bold/Roboto Bold.ttf")
        .await
        .expect("Unable to load font");

    SCORE.store(0, Ordering::Relaxed);
    if let Some(best_data) = quad_storage::STORAGE
        .lock()
        .expect("Error loading data")
        .get("best")
    {
        BEST_SCORE.store(best_data.parse::<u16>().unwrap(), Ordering::Relaxed);
    }

    let mut snake: Snake = Snake {
        body: vec![Point { x: 0, y: 0 }],
        dir: Direction::None,
    };
    let mut apple: Point = Point {
        x: rand::gen_range(0, ROW_SIZE as i16),
        y: rand::gen_range(0, COLUMN_SIZE as i16),
    };

    let mut last_update: f64 = get_time();

    loop {
        if !GAME_OVER.load(Ordering::Relaxed) {
            clear_background(BACKGROUND_COLOR);

            let dimensions: TextDimensions =
            measure_text(TITLE_TEXT, Some(&font), TITLE_SIZE as u16, 1.0);
            draw_text_ex(
                TITLE_TEXT,
                screen_width() / 2f32 - dimensions.width / 2f32,
                screen_height() / 20f32 + dimensions.height,
                TextParams {
                    font: Some(&font),
                    font_size: TITLE_SIZE,
                    font_scale: 1.0,
                    color: TITLE_COLOR,
                    ..Default::default()
                },
            );
            
            render_score(&font);

            snake.dir = {
                let dir: Direction = update_input();
                if dir == Direction::None || Direction::opposite(&dir, &snake.dir) {
                    snake.dir
                } else {
                    dir
                }
            };

        } else {
            clear_background(BACKGROUND_COLOR);
            
            let dimensions: TextDimensions =
                measure_text(TITLE_TEXT, Some(&font), TITLE_SIZE as u16, 1.0);
            draw_text_ex(
                TITLE_TEXT,
                screen_width() / 2f32 - dimensions.width / 2f32,
                screen_height() / 20f32 + dimensions.height,
                TextParams {
                    font: Some(&font),
                    font_size: TITLE_SIZE,
                    font_scale: 1.0,
                    color: TITLE_COLOR,
                    ..Default::default()
                },
            );

            draw_rectangle(
                GRID_OFFSET_X,
                GRID_OFFSET_Y,
                SQUARE_SIZE * ROW_SIZE as f32,
                SQUARE_SIZE * COLUMN_SIZE as f32,
                MAROON,
            );

            let dimensions: TextDimensions =
                measure_text("GAME OVER", Some(&font), TITLE_SIZE as u16, 1.0);
            draw_text_ex(
                "GAME OVER",
                GRID_OFFSET_X + (SQUARE_SIZE * ROW_SIZE as f32) / 2f32 - dimensions.width / 2f32,
                GRID_OFFSET_Y
                    + (SQUARE_SIZE * COLUMN_SIZE as f32) / 2f32
                    + dimensions.height / 2f32,
                TextParams {
                    font: Some(&font),
                    font_size: TITLE_SIZE,
                    font_scale: 1.0,
                    color: GAME_OVER_COLOR,
                    ..Default::default()
                },
            );

            let game_over_text_bottom: f32 =
                dimensions.height + GRID_OFFSET_Y + (SQUARE_SIZE * COLUMN_SIZE as f32) / 2f32;
            let dimensions: TextDimensions =
                measure_text("Press 'R' to restart", Some(&font), 25, 1.0);
            draw_text_ex(
                "Press 'R' to restart",
                GRID_OFFSET_X + (SQUARE_SIZE * ROW_SIZE as f32) / 2f32 - dimensions.width / 2f32,
                game_over_text_bottom + dimensions.height / 2f32,
                TextParams {
                    font: Some(&font),
                    font_size: 25,
                    font_scale: 1.0,
                    color: GAME_OVER_COLOR,
                    ..Default::default()
                },
            );
        }

        if get_time() - last_update > (1f64 / FPS) {
            last_update = get_time();
            update_input();
            if query_reset_game() {
                reset_game(&mut snake, &mut apple);
            }

            if !GAME_OVER.load(Ordering::Relaxed) {
                update_snake(&mut snake, &mut apple);

                render(&snake, &apple);
            }
            next_frame().await;
        }
    }
}