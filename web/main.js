import init, { WebGame } from "./pkg/snake.js";

const CELL_SIZE = 18;
const TICK_MS = 115;
const RESTART_PRESS_MS = 140;
const ENTITY_INSET = 1.3;

const canvas = document.getElementById("board");
const context = canvas.getContext("2d");
const overlay = document.getElementById("overlay");
const scoreNode = document.getElementById("score");
const statusNode = document.getElementById("status");
const pauseButton = document.getElementById("pause-button");
const restartButton = document.getElementById("restart-button");
const touchControls = document.querySelector(".touch-controls");
const dpadButtons = Array.from(document.querySelectorAll(".dpad-button"));
const staticCanvas = document.createElement("canvas");
const staticContext = staticCanvas.getContext("2d");

const pauseIcon = `
  <svg viewBox="0 0 24 24" aria-hidden="true" focusable="false">
    <rect x="6" y="4.5" width="4.5" height="15" rx="1.2" fill="currentColor"></rect>
    <rect x="13.5" y="4.5" width="4.5" height="15" rx="1.2" fill="currentColor"></rect>
  </svg>
`;

let game;
let paused = false;
let gameOver = false;
let timerId;
let restartPressTimer;
let pendingDirection;
let boardWidth = 0;
let boardHeight = 0;
let currentPalette;
const themeQuery = window.matchMedia("(prefers-color-scheme: dark)");
const coarsePointerQuery = window.matchMedia("(pointer: coarse)");
const touchControlsQuery = window.matchMedia("(max-width: 760px)");

function isReady() {
  return game !== undefined;
}

function usesTouchControls() {
  if (!touchControls) {
    return false;
  }
  return window.getComputedStyle(touchControls).display !== "none";
}

function idleStatusText() {
  return usesTouchControls() ? "" : "Use arrow keys to play.";
}

function gameOverOverlayMarkup() {
  if (usesTouchControls()) {
    return "Game over. Tap Restart or D-pad.";
  }

  return [
    '<span class="overlay-line">Game over.</span>',
    '<span class="overlay-line overlay-shortcuts">',
    '<span class="keycap keycap-space">Space</span>',
    '<span class="overlay-or">or</span>',
    '<span class="keycap">Esc</span>',
    '<span class="overlay-action">to restart</span>',
    "</span>",
  ].join("");
}

function shouldRunLoop() {
  return isReady() && !paused && !gameOver && !document.hidden;
}

function themeColor(name) {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

function palette() {
  return {
    board: themeColor("--canvas-board"),
    grid: themeColor("--canvas-grid"),
    wall: themeColor("--canvas-wall"),
    food: themeColor("--canvas-food"),
    snake: themeColor("--canvas-snake"),
    snakeHead: themeColor("--canvas-snake-head"),
  };
}

function setPauseButtonState(isPressed) {
  pauseButton.classList.toggle("is-pressed", isPressed);
  pauseButton.setAttribute("aria-pressed", String(isPressed));
  pauseButton.setAttribute("aria-label", isPressed ? "Resume game" : "Pause game");
  pauseButton.setAttribute("title", isPressed ? "Resume" : "Pause");
}

function pulseRestartButton() {
  restartButton.classList.add("is-pressed");
  if (restartPressTimer !== undefined) {
    window.clearTimeout(restartPressTimer);
  }
  restartPressTimer = window.setTimeout(() => {
    restartButton.classList.remove("is-pressed");
  }, RESTART_PRESS_MS);
}

function setOverlay(markup) {
  overlay.innerHTML = markup;
  overlay.classList.toggle("hidden", markup.length === 0);
}

function drawCell(x, y, color, inset = 0) {
  const px = x * CELL_SIZE + inset;
  const py = y * CELL_SIZE + inset;
  const size = CELL_SIZE - inset * 2;
  context.fillStyle = color;
  context.fillRect(px, py, size, size);
}

function drawCellTo(targetContext, x, y, color, inset = 0) {
  const px = x * CELL_SIZE + inset;
  const py = y * CELL_SIZE + inset;
  const size = CELL_SIZE - inset * 2;
  targetContext.fillStyle = color;
  targetContext.fillRect(px, py, size, size);
}

function renderStaticLayer() {
  currentPalette = palette();

  staticContext.fillStyle = currentPalette.board;
  staticContext.fillRect(0, 0, staticCanvas.width, staticCanvas.height);

  for (let y = 0; y < boardHeight; y += 1) {
    for (let x = 0; x < boardWidth; x += 1) {
      drawCellTo(staticContext, x, y, currentPalette.grid, 0.4);
    }
  }

  for (let x = 0; x < boardWidth; x += 1) {
    drawCellTo(staticContext, x, 0, currentPalette.wall, 0.9);
    drawCellTo(staticContext, x, boardHeight - 1, currentPalette.wall, 0.9);
  }

  for (let y = 1; y < boardHeight - 1; y += 1) {
    drawCellTo(staticContext, 0, y, currentPalette.wall, 0.9);
    drawCellTo(staticContext, boardWidth - 1, y, currentPalette.wall, 0.9);
  }
}

function render() {
  context.drawImage(staticCanvas, 0, 0);

  const foodCount = game.food_len();
  for (let i = 0; i < foodCount; i += 1) {
    drawCell(game.food_x(i), game.food_y(i), currentPalette.food, ENTITY_INSET);
  }

  const snakeCount = game.snake_len();
  for (let i = 0; i < snakeCount; i += 1) {
    drawCell(game.snake_x(i), game.snake_y(i), currentPalette.snake, ENTITY_INSET);
  }

  if (snakeCount > 0) {
    drawCell(game.snake_x(0), game.snake_y(0), currentPalette.snakeHead, ENTITY_INSET);
  }
}

function createGame() {
  game = new WebGame();
  paused = false;
  gameOver = false;
  pendingDirection = undefined;
  boardWidth = game.width();
  boardHeight = game.height();

  canvas.width = boardWidth * CELL_SIZE;
  canvas.height = boardHeight * CELL_SIZE;
  staticCanvas.width = canvas.width;
  staticCanvas.height = canvas.height;

  scoreNode.textContent = "0";
  statusNode.textContent = idleStatusText();
  pauseButton.innerHTML = pauseIcon;
  setPauseButtonState(false);
  setOverlay("");
  renderStaticLayer();
  render();
  startLoop();
}

function stopLoop() {
  if (timerId !== undefined) {
    window.clearInterval(timerId);
    timerId = undefined;
  }
}

function startLoop() {
  if (!shouldRunLoop() || timerId !== undefined) {
    return;
  }
  timerId = window.setInterval(step, TICK_MS);
}

function step() {
  if (!shouldRunLoop()) {
    stopLoop();
    return;
  }

  if (pendingDirection !== undefined) {
    applyDirection(pendingDirection);
    pendingDirection = undefined;
  }

  const alive = game.tick();
  scoreNode.textContent = String(game.score());
  render();

  if (!alive) {
    gameOver = true;
    statusNode.textContent = "Game over.";
    setPauseButtonState(false);
    setOverlay(gameOverOverlayMarkup());
    stopLoop();
  }
}

function restart() {
  if (!isReady()) {
    return;
  }
  pulseRestartButton();
  stopLoop();
  createGame();
}

function togglePause() {
  if (!isReady() || gameOver) {
    return;
  }
  paused = !paused;
  if (paused) {
    stopLoop();
  } else {
    startLoop();
  }
  setPauseButtonState(paused);
  statusNode.textContent = paused ? "Paused." : idleStatusText();
  setOverlay(paused ? "Paused" : "");
}

function applyDirection(direction) {
  if (direction === "up") {
    game.move_up();
  } else if (direction === "down") {
    game.move_down();
  } else if (direction === "left") {
    game.move_left();
  } else if (direction === "right") {
    game.move_right();
  }
}

function queueDirection(direction) {
  if (!isReady() || paused || gameOver) {
    return;
  }
  if (!["up", "down", "left", "right"].includes(direction)) {
    return;
  }

  if (pendingDirection !== undefined) {
    return;
  }

  pendingDirection = direction;
}

function handleDirection(code) {
  if (code === "ArrowUp") {
    queueDirection("up");
  } else if (code === "ArrowDown") {
    queueDirection("down");
  } else if (code === "ArrowLeft") {
    queueDirection("left");
  } else if (code === "ArrowRight") {
    queueDirection("right");
  }
}

function pressDpadButton(button) {
  button.classList.add("is-pressed");
  if (gameOver) {
    restart();
    return;
  }
  queueDirection(button.dataset.direction);
}

async function main() {
  await init();
  createGame();
}

window.addEventListener("keydown", (event) => {
  const navigationalKey = event.code.startsWith("Arrow") || ["Space", "Escape"].includes(event.code);
  if (navigationalKey) {
    event.preventDefault();
  }

  if (event.repeat) {
    return;
  }

  if (event.code === "Space") {
    if (!isReady()) {
      return;
    }
    if (gameOver) {
      restart();
      return;
    }
    togglePause();
    return;
  }

  if (event.code === "Escape") {
    if (!isReady()) {
      return;
    }
    restart();
    return;
  }

  handleDirection(event.code);
});

pauseButton.addEventListener("click", togglePause);
restartButton.addEventListener("click", restart);
dpadButtons.forEach((button) => {
  const release = () => {
    button.classList.remove("is-pressed");
  };

  button.addEventListener("pointerdown", (event) => {
    if (event.pointerType === "touch") {
      return;
    }
    event.preventDefault();
    pressDpadButton(button);
  });

  button.addEventListener(
    "touchstart",
    (event) => {
      event.preventDefault();
      pressDpadButton(button);
    },
    { passive: false },
  );

  button.addEventListener("pointerup", release);
  button.addEventListener("pointercancel", release);
  button.addEventListener("pointerleave", release);
  button.addEventListener("touchend", release);
  button.addEventListener("touchcancel", release);
});

window.addEventListener("beforeunload", () => {
  stopLoop();
});

const handleThemeChange = () => {
  if (game) {
    renderStaticLayer();
    render();
  }
};

const handleInputModeChange = () => {
  if (!game) {
    return;
  }
  if (!paused && !gameOver) {
    statusNode.textContent = idleStatusText();
  }
  render();
};

const handleVisibilityChange = () => {
  if (shouldRunLoop()) {
    startLoop();
  } else {
    stopLoop();
  }
};

if (typeof themeQuery.addEventListener === "function") {
  themeQuery.addEventListener("change", handleThemeChange);
} else if (typeof themeQuery.addListener === "function") {
  themeQuery.addListener(handleThemeChange);
}

if (typeof coarsePointerQuery.addEventListener === "function") {
  coarsePointerQuery.addEventListener("change", handleInputModeChange);
} else if (typeof coarsePointerQuery.addListener === "function") {
  coarsePointerQuery.addListener(handleInputModeChange);
}

if (typeof touchControlsQuery.addEventListener === "function") {
  touchControlsQuery.addEventListener("change", handleInputModeChange);
} else if (typeof touchControlsQuery.addListener === "function") {
  touchControlsQuery.addListener(handleInputModeChange);
}

document.addEventListener("visibilitychange", handleVisibilityChange);

main().catch((error) => {
  console.error(error);
  statusNode.textContent = "Failed to start the WebAssembly build.";
  setOverlay("WebAssembly bootstrap failed. See console for details.");
});
