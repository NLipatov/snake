![Build](https://github.com/NLipatov/snake/actions/workflows/ci.yml/badge.svg)
[![codecov](https://codecov.io/gh/NLipatov/snake/branch/main/graph/badge.svg)](https://codecov.io/gh/NLipatov/snake)

# snake 🐍

⚡ Blazing fast Snake game in Rust for terminal and WebAssembly-powered web.

Play now at: https://snake.ethacore.com

## Controls 🎮

### Web

- Arrow keys: move on desktop
- On-screen `D-pad`: move on phones and tablets
- `Space`: pause/resume on desktop
- `Esc`: restart on desktop
- `Restart` button or `D-pad`: restart after game over on touch devices

### Terminal

- Arrow keys: move
- `Space`: pause/resume
- `Esc`: quit

## Run In Terminal ▶️

```bash
cargo run
```

## Run In Browser 🌐

Build the WebAssembly bundle into `web/pkg`:

```bash
wasm-pack build --target web --out-dir web/pkg
```

Serve the static `web` directory:

```bash
python3 -m http.server --directory web 8000
```

Then open `http://localhost:8000`.
