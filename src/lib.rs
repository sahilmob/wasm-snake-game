use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    snake: Snake,
}

impl Snake {
    fn new(spawn_idx: usize) -> Self {
        Self {
            direction: Direction::Left,
            body: vec![SnakeCell(spawn_idx)],
        }
    }
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize, snake_spawn_idx: usize) -> Self {
        Self {
            size,
            snake: Snake::new(snake_spawn_idx),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.size
    }

    #[wasm_bindgen(getter = "snakeHeadIdx")]
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.size;
        let col = snake_idx % self.size;

        match self.snake.direction {
            Direction::Up => {
                let next_row = (row - 1) % self.size;
                self.snake.body[0].0 = (next_row * self.size) + col;
            }
            Direction::Down => {
                let next_row = (row + 1) % self.size;
                self.snake.body[0].0 = (next_row * self.size) + col;
            }
            Direction::Right => {
                let next_col = (snake_idx + 1) % self.size;
                self.snake.body[0].0 = (row * self.size) + next_col;
            }
            Direction::Left => {
                let next_col = (snake_idx - 1) % self.size;
                self.snake.body[0].0 = (row * self.size) + next_col;
            }
        }
    }
}
