import init, { World } from "snake_game";

import "./style.css";

const FPS = 10;
const CELL_SIZE = 10;
const WORLD_SIZE = 8;
const SNAKE_SPAWN_IDX = Date.now() % WORLD_SIZE ** 2;

await init();
const world = new World(WORLD_SIZE, SNAKE_SPAWN_IDX);
const canvas = <HTMLCanvasElement>document.getElementById("canvas");
canvas.height = WORLD_SIZE * CELL_SIZE;
canvas.height = WORLD_SIZE * CELL_SIZE;
const ctx = canvas.getContext("2d");

function drawWorld() {
  ctx?.beginPath();

  for (let x = 0; x <= WORLD_SIZE; x++) {
    ctx?.moveTo(CELL_SIZE * x, 0);
    ctx?.lineTo(CELL_SIZE * x, WORLD_SIZE * CELL_SIZE);
  }

  for (let y = 0; y <= WORLD_SIZE; y++) {
    ctx?.moveTo(0, CELL_SIZE * y);
    ctx?.lineTo(WORLD_SIZE * CELL_SIZE, CELL_SIZE * y);
  }

  ctx?.stroke();
}

function drawSnake() {
  const snakeIdx = world.snakeHeadIdx;
  const col = snakeIdx % WORLD_SIZE;
  const row = Math.floor(snakeIdx / WORLD_SIZE);

  ctx?.beginPath();

  ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

  ctx?.stroke();
}

function drawGame() {
  ctx?.clearRect(0, 0, canvas.width, canvas.height);
  world.update();
  drawWorld();
  drawSnake();
}

function play() {
  setTimeout(() => {
    drawGame();
    requestAnimationFrame(play);
  }, 1000 / FPS);
}

play();
