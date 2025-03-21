import init, { World, Direction } from "snake_game";

import "./style.css";

const FPS = 10;
const CELL_SIZE = 10;
const WORLD_SIZE = 8;
const snakeSpawnIdx = Date.now() % WORLD_SIZE ** 2;

const wasm = await init();
const world = new World(WORLD_SIZE, snakeSpawnIdx);
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
  const snakeCellPtr = world.snakeCellsPtr;
  const snakeLength = world.snakeLength;
  const snakeCells = new Uint32Array(
    wasm.memory.buffer,
    snakeCellPtr,
    snakeLength
  );

  for (const i in snakeCells) {
    const idx = snakeCells[i];
    const col = idx % WORLD_SIZE;
    const row = Math.floor(idx / WORLD_SIZE);

    ctx!.fillStyle = +i === 0 ? "#7878db" : "#000000";

    ctx?.beginPath();

    ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  }

  ctx?.stroke();
}

function drawRewardCell() {
  const rewardCell = world.rewardCell;
  const col = rewardCell % WORLD_SIZE;
  const row = Math.floor(rewardCell / WORLD_SIZE);
  ctx?.beginPath();
  ctx!.fillStyle = "#ff0000";
  ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

  ctx?.stroke();
}

function drawGame() {
  ctx?.clearRect(0, 0, canvas.width, canvas.height);
  world.step();
  drawWorld();
  drawSnake();
  drawRewardCell();
}

function play() {
  setTimeout(() => {
    drawGame();
    requestAnimationFrame(play);
  }, 1000 / FPS);
}

document.addEventListener("keydown", (e) => {
  switch (e.code) {
    case "ArrowUp":
      world.direction = Direction.Up;
      break;
    case "ArrowRight":
      world.direction = Direction.Right;
      break;
    case "ArrowDown":
      world.direction = Direction.Down;
      break;
    case "ArrowLeft":
      world.direction = Direction.Left;
      break;
  }
});

play();
