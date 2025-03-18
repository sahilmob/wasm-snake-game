import init, { World } from "snake_game";

import "./style.css";

const CELL_SIZE = 10;

await init();
const world = new World();
const worldSize = world.size;
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
canvas.height = worldSize * CELL_SIZE;
canvas.height = worldSize * CELL_SIZE;
const ctx = canvas!.getContext("2d");

function drawWorld() {
  ctx?.beginPath();

  for (let x = 0; x <= worldSize; x++) {
    ctx?.moveTo(CELL_SIZE * x, 0);
    ctx?.lineTo(CELL_SIZE * x, worldSize * CELL_SIZE);
  }

  for (let y = 0; y <= worldSize; y++) {
    ctx?.moveTo(0, CELL_SIZE * y);
    ctx?.lineTo(worldSize * CELL_SIZE, CELL_SIZE * y);
  }

  ctx?.stroke();
}

function drawSnake() {
  const snakeIdx = world.snakeHeadIdx;
  const col = snakeIdx % worldSize;
  const row = Math.floor(snakeIdx / worldSize);

  ctx?.beginPath();

  ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

  ctx?.stroke();
}

drawWorld();
drawSnake();
