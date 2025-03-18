use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);

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

    #[wasm_bindgen(setter = "direction")]
    pub fn set_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.idx_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Direction::Up => ((row - 1) % self.size, col),
            Direction::Down => ((row + 1) % self.size, col),
            Direction::Right => (row, (snake_idx + 1) % self.size),
            Direction::Left => (row, (snake_idx - 1) % self.size),
        };

        self.set_snake_head(self.cell_to_idx(row, col));
    }

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn idx_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.size, idx % self.size)
    }

    fn cell_to_idx(&self, row: usize, col: usize) -> usize {
        (row * self.size) + col
    }
}
