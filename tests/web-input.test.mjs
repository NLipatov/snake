import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";
import { initSync, WebGame } from "../web/pkg/snake.js";

initSync({ module: readFileSync(new URL("../web/pkg/snake_bg.wasm", import.meta.url)) });
const source = readFileSync(new URL("../web/main.js", import.meta.url), "utf8").replace(
  /^import .* from "\.\/pkg\/snake\.js";\n/,
  "",
);

// Run the browser entry point with real WASM and controlled events and time.
async function startGame(t) {
  let now = 0;
  let nextTimerId = 0;
  const timers = new Map();
  const keyHandlers = new Map();
  const drawing = { fillRect() {}, drawImage() {} };
  const element = () => ({
    handlers: new Map(),
    classList: { add() {}, remove() {}, toggle() {} },
    setAttribute() {},
    addEventListener(type, handler) {
      this.handlers.set(type, handler);
    },
    getContext: () => drawing,
  });
  const nodes = new Map();
  const buttons = ["up", "down", "left", "right"].map((direction) =>
    Object.assign(element(), { dataset: { direction } }),
  );
  const document = {
    hidden: false,
    documentElement: {},
    getElementById(id) {
      if (!nodes.has(id)) nodes.set(id, element());
      return nodes.get(id);
    },
    createElement: element,
    querySelector: element,
    querySelectorAll: () => buttons,
    addEventListener() {},
  };
  const window = {
    matchMedia: () => ({ matches: false, addEventListener() {} }),
    getComputedStyle: () => ({ display: "none" }),
    addEventListener(type, handler) {
      keyHandlers.set(type, handler);
    },
    setInterval(callback, interval) {
      const id = ++nextTimerId;
      timers.set(id, { callback, interval, next: now + interval });
      return id;
    },
    clearInterval(id) {
      timers.delete(id);
    },
    // The restart button's visual feedback does not affect movement.
    setTimeout() {
      return ++nextTimerId;
    },
    clearTimeout() {},
  };
  const context = vm.createContext({
    document,
    window,
    console,
    WebGame,
    init: async () => {},
    getComputedStyle: () => ({ getPropertyValue: () => "#000" }),
  });
  // Check browser compatibility in every scenario without changing Node's built-ins.
  vm.runInContext("delete Object.hasOwn; delete Array.prototype.at;", context);
  vm.runInContext(source, context, { filename: "web/main.js" });
  await new Promise((resolve) => setImmediate(resolve));
  t.after(() => vm.runInContext("game.free()", context));

  return {
    key(code, repeat = false) {
      keyHandlers.get("keydown")({ code, repeat, preventDefault() {} });
    },
    touch(direction) {
      const button = buttons.find((candidate) => candidate.dataset.direction === direction);
      button.handlers.get("touchstart")({ preventDefault() {} });
    },
    advance(ms) {
      const target = now + ms;
      while (true) {
        const timer = [...timers.values()]
          .filter((candidate) => candidate.next <= target)
          .sort((a, b) => a.next - b.next)[0];
        if (!timer) break;
        now = timer.next;
        timer.next += timer.interval;
        timer.callback();
      }
      now = target;
    },
    head() {
      return Array.from(vm.runInContext("[game.snake_x(0), game.snake_y(0)]", context));
    },
  };
}

test("a single turn takes effect on the next tick, without an extra tick", async (t) => {
  const game = await startGame(t);
  game.advance(10);
  game.key("ArrowUp");
  game.advance(104);
  assert.deepEqual(game.head(), [5, 5]);
  game.advance(1);
  assert.deepEqual(game.head(), [5, 4]);
});

for (const input of ["keyboard", "touch"]) {
  test(`${input} preserves two quick turns on consecutive ticks`, async (t) => {
    const game = await startGame(t);
    if (input === "keyboard") {
      game.key("ArrowUp");
      game.key("ArrowLeft");
    } else {
      game.touch("up");
      game.touch("left");
    }
    game.advance(115);
    assert.deepEqual(game.head(), [5, 4]);
    game.advance(115);
    assert.deepEqual(game.head(), [4, 4]);
  });
}

for (const ignoredKey of ["ArrowRight", "ArrowLeft"]) {
  test(`${ignoredKey} while moving right does not block a valid turn`, async (t) => {
    const game = await startGame(t);
    game.key(ignoredKey);
    game.key("ArrowUp");
    game.advance(115);
    assert.deepEqual(game.head(), [5, 4]);
  });
}

test("queued turns are validated against the last queued direction", async (t) => {
  const game = await startGame(t);
  game.key("ArrowUp");
  game.key("ArrowUp");
  game.key("ArrowDown");
  game.key("ArrowLeft");
  game.advance(115);
  assert.deepEqual(game.head(), [5, 4]);
  game.advance(115);
  assert.deepEqual(game.head(), [4, 4]);
  game.key("ArrowLeft");
  game.key("ArrowRight");
  game.key("ArrowDown");
  game.advance(115);
  assert.deepEqual(game.head(), [4, 5]);
});

test("the buffer keeps at most two turns and accepts input after a tick", async (t) => {
  const game = await startGame(t);
  game.key("ArrowUp");
  game.key("ArrowLeft");
  game.key("ArrowDown");
  game.advance(115);
  assert.deepEqual(game.head(), [5, 4]);
  game.key("ArrowUp");
  game.advance(115);
  assert.deepEqual(game.head(), [4, 4]);
  game.advance(115);
  assert.deepEqual(game.head(), [4, 3]);
  game.advance(115);
  assert.deepEqual(game.head(), [4, 2]);
});

test("keyboard auto-repeat does not occupy the buffer", async (t) => {
  const game = await startGame(t);
  game.key("ArrowDown", true);
  game.key("ArrowUp");
  game.key("ArrowLeft");
  game.advance(230);
  assert.deepEqual(game.head(), [4, 4]);
});

test("pause preserves queued turns until resumed", async (t) => {
  const game = await startGame(t);
  game.key("ArrowUp");
  game.key("ArrowLeft");
  game.key("Space");
  game.advance(230);
  assert.deepEqual(game.head(), [5, 5]);
  game.key("Space");
  game.advance(230);
  assert.deepEqual(game.head(), [4, 4]);
  game.advance(115);
  assert.deepEqual(game.head(), [3, 4]);
});

test("keyboard and touch input are ignored while paused", async (t) => {
  const game = await startGame(t);
  game.key("Space");
  game.key("ArrowUp");
  game.touch("left");
  game.key("Space");
  game.advance(230);
  assert.deepEqual(game.head(), [7, 5]);
});

test("restart clears pending turns and resets direction validation", async (t) => {
  const game = await startGame(t);
  game.key("ArrowUp");
  game.advance(115);
  game.key("ArrowLeft");
  game.key("ArrowDown");
  game.key("Escape");
  game.advance(115);
  assert.deepEqual(game.head(), [6, 5]);
  game.key("ArrowUp");
  game.advance(115);
  assert.deepEqual(game.head(), [6, 4]);
});
