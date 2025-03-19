use std::vec;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[wasm_bindgen]

pub enum Direction {
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
    fn new(spawn_idx: usize, size: usize) -> Self {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_idx - i));
        }

        Self {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(
        size: usize,
        #[wasm_bindgen(js_name = "snakeSpawnIdx")] snake_spawn_idx: usize,
    ) -> Self {
        Self {
            size,
            snake: Snake::new(snake_spawn_idx, 3),
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

    #[wasm_bindgen(setter = "direction")]
    pub fn set_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.snake.direction = direction;
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        let next_cell = self.generate_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let len = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.size;

        return match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.size;
                if snake_idx + 1 == threshold {
                    SnakeCell(threshold - self.size)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.size;
                if snake_idx == threshold {
                    SnakeCell(threshold + (self.size - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_idx - (row * self.size);
                if snake_idx == threshold {
                    SnakeCell((self.size.pow(2) - self.size) + threshold)
                } else {
                    SnakeCell(snake_idx - self.size)
                }
            }
            Direction::Down => {
                let threshold = snake_idx + ((self.size - row) * self.size);
                if snake_idx + self.size == threshold {
                    SnakeCell(threshold - ((row + 1) * self.size))
                } else {
                    SnakeCell(snake_idx + self.size)
                }
            }
        };
    }

    #[wasm_bindgen(getter = "snakeLength")]
    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    #[wasm_bindgen(getter = "snakeCellsPtr")]
    // this method returns a pointer to
    // the first SnakeCell
    pub fn snake_cells_ptr(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }
}
