# Pong WASM Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a minimal Pong game with Rust/WASM and JavaScript, scaffold the project, push to GitHub, and add CI/CD.

**Architecture:** wasm-pack + minimal JS. Rust handles game logic; JS handles canvas, input, game loop.

**Tech Stack:** Rust (wasm-bindgen), wasm-pack, vanilla JavaScript, Canvas API, GitHub Actions

---

## Task 1: Initialize pong_v0 Rust project

**Files:**
- Create: `pong_v0/Cargo.toml`
- Create: `pong_v0/src/lib.rs`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "pong_v0"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

[profile.release]
opt-level = "s"
lto = true
```

**Step 2: Create minimal lib.rs**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() -> String {
    "ok".to_string()
}
```

**Step 3: Verify build**

Run: `cd pong_v0 && cargo build`
Expected: Compiles successfully

**Step 4: Commit**

```bash
git add pong_v0/
git commit -m "chore: init pong_v0 Rust project"
```

---

## Task 2: Add wasm-pack build and WASM exports

**Files:**
- Modify: `pong_v0/Cargo.toml` (add wasm-pack config)
- Modify: `pong_v0/src/lib.rs` (game state, init, tick, get_state, set_paddle)

**Step 1: Add wasm-bindgen config to Cargo.toml**

Add under `[package]`:
```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-O4"]
```

**Step 2: Implement game logic in lib.rs**

- GameState struct: ball_x, ball_y, ball_vx, ball_vy, paddle1_y, paddle2_y
- Constants: canvas 800x600, paddle height 80, paddle width 10, ball size 10
- init() -> returns JSON string of initial state
- tick(delta_ms: f64) -> update ball, bounce off walls/paddles
- get_state() -> returns JSON string [ball_x, ball_y, paddle1_y, paddle2_y]
- set_paddle(player: u8, direction: i8) -> move paddle (player 1 or 2, -1/0/1)

**Step 3: Build WASM**

Run: `cd pong_v0 && wasm-pack build --target web`
Expected: Creates pkg/ with pong_v0.js and pong_v0_bg.wasm

**Step 4: Commit**

```bash
git add pong_v0/
git commit -m "feat: add game logic and WASM exports"
```

---

## Task 3: Create HTML and JavaScript

**Files:**
- Create: `pong_v0/index.html`
- Create: `pong_v0/js/game.js`

**Step 1: Create index.html**

- Canvas 800x600
- Script type="module" src="js/game.js"
- Load WASM from pkg/pong_v0.js

**Step 2: Create game.js**

- Import init from pkg
- Async init: load WASM, call init(), start game loop
- Game loop: requestAnimationFrame, tick(dt), get_state(), draw to canvas
- Keyboard: W/S for player 1, ArrowUp/ArrowDown for player 2
- Draw: rectangles for paddles and ball

**Step 3: Add .gitignore**

Create/update root `.gitignore`:
```
target/
pkg/
*.wasm
.DS_Store
```

**Step 4: Run locally**

Run: `cd pong_v0 && python3 -m http.server 8080` (or serve from pong_v0/)
Open: http://localhost:8080
Expected: Pong game runs (ball bounces, paddles move)

**Step 5: Commit**

```bash
git add pong_v0/ .gitignore
git commit -m "feat: add HTML, JS game loop, and canvas rendering"
```

---

## Task 4: Initialize Git and push to GitHub

**Step 1: Initialize Git (if not already)**

Run: `git init`
Run: `git add . && git status`

**Step 2: Create GitHub repo**

Run: `gh repo create learning_wasm --public --source=. --remote=origin --push`
(Alternative if no gh: create repo manually on GitHub, then `git remote add origin <url>` and `git push -u origin main`)

**Step 3: Verify**

Repo exists at github.com/<user>/learning_wasm with initial commit

---

## Task 5: Add CI/CD pipeline

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: Create workflow**

```yaml
name: CI

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: cd pong_v0 && wasm-pack build --target web --release

      - name: Verify build
        run: test -f pong_v0/pkg/pong_v0_bg.wasm
```

**Step 2: Commit and push**

```bash
git add .github/
git commit -m "ci: add GitHub Actions build workflow"
git push
```

**Step 3: Verify**

GitHub Actions run shows green check
