import init, { World } from "snake_game";

import "./style.css";

await init();
const world = new World();
console.log(world.width);
