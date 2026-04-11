import init, { WebGame } from "./pkg/snake.js";

const CELL_SIZE = 18;
const EMPTY = 0;
const WALL = 1;
const FOOD = 2;
const SNAKE = 3;
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
const dpadButtons = Array.from(document.querySelectorAll(".dpad-button"));

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
const themeQuery = window.matchMedia("(prefers-color-scheme: dark)");
const coarsePointerQuery = window.matchMedia("(pointer: coarse)");

function isReady() {
  return game !== undefined;
}

function isTouchDevice() {
  return coarsePointerQuery.matches;
}

function idleStatusText() {
  return isTouchDevice() ? "" : "Use arrow keys to play.";
}

function gameOverOverlayMarkup() {
  if (isTouchDevice()) {
    return "Game over. Tap Restart.";
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

function createGame() {
  game = new WebGame();
  paused = false;
  gameOver = false;

  canvas.width = game.width() * CELL_SIZE;
  canvas.height = game.height() * CELL_SIZE;

  scoreNode.textContent = "0";
  statusNode.textContent = idleStatusText();
  pauseButton.innerHTML = pauseIcon;
  setPauseButtonState(false);
  setOverlay("");
  render();
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

function render() {
  const colors = palette();
  context.fillStyle = colors.board;
  context.fillRect(0, 0, canvas.width, canvas.height);

  for (let y = 0; y < game.height(); y += 1) {
    for (let x = 0; x < game.width(); x += 1) {
      drawCell(x, y, colors.grid, 0.4);

      const cell = game.cell_at(x, y);
      if (cell === WALL) {
        drawCell(x, y, colors.wall, 0.9);
      } else if (cell === FOOD) {
        drawCell(x, y, colors.food, ENTITY_INSET);
      } else if (cell === SNAKE) {
        drawCell(x, y, colors.snake, ENTITY_INSET);
      }
    }
  }

  const headX = game.head_x();
  const headY = game.head_y();
  if (game.cell_at(headX, headY) === SNAKE) {
    drawCell(headX, headY, colors.snakeHead, ENTITY_INSET);
  }
}

function step() {
  if (paused || gameOver) {
    return;
  }

  const alive = game.tick();
  scoreNode.textContent = String(game.score());
  render();

  if (!alive) {
    gameOver = true;
    statusNode.textContent = "Game over.";
    setPauseButtonState(false);
    setOverlay(gameOverOverlayMarkup());
  }
}

function restart() {
  if (!isReady()) {
    return;
  }
  pulseRestartButton();
  createGame();
}

function togglePause() {
  if (!isReady() || gameOver) {
    return;
  }
  paused = !paused;
  setPauseButtonState(paused);
  statusNode.textContent = paused ? "Paused." : idleStatusText();
  setOverlay(paused ? "Paused" : "");
}

function handleDirection(code) {
  if (!isReady() || gameOver) {
    return;
  }
  if (code === "ArrowUp") {
    game.move_up();
  } else if (code === "ArrowDown") {
    game.move_down();
  } else if (code === "ArrowLeft") {
    game.move_left();
  } else if (code === "ArrowRight") {
    game.move_right();
  } else {
    return;
  }
  render();
}

function handleDirectionName(direction) {
  if (!isReady() || gameOver) {
    return;
  }
  if (direction === "up") {
    game.move_up();
  } else if (direction === "down") {
    game.move_down();
  } else if (direction === "left") {
    game.move_left();
  } else if (direction === "right") {
    game.move_right();
  } else {
    return;
  }
  render();
}

async function main() {
  await init();
  createGame();
  timerId = window.setInterval(step, TICK_MS);
}

window.addEventListener("keydown", (event) => {
  const navigationalKey = event.code.startsWith("Arrow") || ["Space", "Escape"].includes(event.code);
  if (navigationalKey) {
    event.preventDefault();
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
    event.preventDefault();
    button.classList.add("is-pressed");
    if (gameOver) {
      restart();
      return;
    }
    handleDirectionName(button.dataset.direction);
  });

  button.addEventListener("pointerup", release);
  button.addEventListener("pointercancel", release);
  button.addEventListener("pointerleave", release);
});

window.addEventListener("beforeunload", () => {
  if (timerId !== undefined) {
    window.clearInterval(timerId);
  }
});

const handleThemeChange = () => {
  if (game) {
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

main().catch((error) => {
  console.error(error);
  statusNode.textContent = "Failed to start the WebAssembly build.";
  setOverlay("WebAssembly bootstrap failed. See console for details.");
});
