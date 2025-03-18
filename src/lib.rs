use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

#[wasm_bindgen]
pub struct World {
    size: usize,
    snake: Snake,
}

impl Snake {
    fn new(spawn_idx: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_idx)],
        }
    }
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            size: 8,
            snake: Snake::new(10),
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
        self.snake.body[0].0 = (snake_idx + 1) % self.size.pow(2);
    }
}
