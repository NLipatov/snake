![Build](https://github.com/NLipatov/snake/actions/workflows/ci.yml/badge.svg)
[![codecov](https://codecov.io/gh/NLipatov/snake/branch/main/graph/badge.svg)](https://codecov.io/gh/NLipatov/snake)

# snake 🐍

A small Snake game written by hand in Rust, with both terminal and browser frontends.

Play in browser: https://snake.ethacore.com

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

## Controls 🎮

- Arrow keys: move on desktop
- On-screen `D-pad`: move on phones and tablets
- `Space`: pause/resume in the browser
- `Esc`: quit in terminal, restart in the browser
