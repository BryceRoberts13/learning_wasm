use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// Constants
const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 600.0;
const PADDLE_HEIGHT: f64 = 80.0;
const PADDLE_WIDTH: f64 = 10.0;
const BALL_SIZE: f64 = 10.0;
const PADDLE_SPEED: f64 = 300.0;
const BALL_SPEED: f64 = 250.0;

struct GameState {
    ball_x: f64,
    ball_y: f64,
    ball_vx: f64,
    ball_vy: f64,
    paddle1_y: f64,
    paddle2_y: f64,
    paddle1_dir: i8,
    paddle2_dir: i8,
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn init() -> String {
    let state = GameState {
        ball_x: CANVAS_WIDTH / 2.0 - BALL_SIZE / 2.0,
        ball_y: CANVAS_HEIGHT / 2.0 - BALL_SIZE / 2.0,
        ball_vx: BALL_SPEED,
        ball_vy: BALL_SPEED * 0.5,
        paddle1_y: (CANVAS_HEIGHT - PADDLE_HEIGHT) / 2.0,
        paddle2_y: (CANVAS_HEIGHT - PADDLE_HEIGHT) / 2.0,
        paddle1_dir: 0,
        paddle2_dir: 0,
    };
    GAME_STATE.with(|s| *s.borrow_mut() = Some(state));
    get_state()
}

#[wasm_bindgen]
pub fn tick(delta_ms: f64) {
    GAME_STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(ref mut g) = *state {
            let dt = delta_ms / 1000.0;

            // Move paddles
            g.paddle1_y += (g.paddle1_dir as f64) * PADDLE_SPEED * dt;
            g.paddle2_y += (g.paddle2_dir as f64) * PADDLE_SPEED * dt;

            // Clamp paddles to canvas
            g.paddle1_y = g.paddle1_y.max(0.0).min(CANVAS_HEIGHT - PADDLE_HEIGHT);
            g.paddle2_y = g.paddle2_y.max(0.0).min(CANVAS_HEIGHT - PADDLE_HEIGHT);

            // Move ball
            g.ball_x += g.ball_vx * dt;
            g.ball_y += g.ball_vy * dt;

            // Bounce off top/bottom walls
            if g.ball_y <= 0.0 {
                g.ball_y = 0.0;
                g.ball_vy = g.ball_vy.abs();
            }
            if g.ball_y >= CANVAS_HEIGHT - BALL_SIZE {
                g.ball_y = CANVAS_HEIGHT - BALL_SIZE;
                g.ball_vy = -g.ball_vy.abs();
            }

            // Paddle 1 (left) collision
            let paddle1_left = PADDLE_WIDTH;
            let paddle1_right = PADDLE_WIDTH * 2.0;
            if g.ball_vx < 0.0
                && g.ball_x <= paddle1_right
                && g.ball_x + BALL_SIZE >= paddle1_left
                && g.ball_y + BALL_SIZE >= g.paddle1_y
                && g.ball_y <= g.paddle1_y + PADDLE_HEIGHT
            {
                g.ball_x = paddle1_right;
                g.ball_vx = g.ball_vx.abs();
            }

            // Paddle 2 (right) collision
            let paddle2_left = CANVAS_WIDTH - PADDLE_WIDTH * 2.0;
            let paddle2_right = CANVAS_WIDTH - PADDLE_WIDTH;
            if g.ball_vx > 0.0
                && g.ball_x + BALL_SIZE >= paddle2_left
                && g.ball_x <= paddle2_right
                && g.ball_y + BALL_SIZE >= g.paddle2_y
                && g.ball_y <= g.paddle2_y + PADDLE_HEIGHT
            {
                g.ball_x = paddle2_left - BALL_SIZE;
                g.ball_vx = -g.ball_vx.abs();
            }

            // Ball out of bounds (left/right) - reset to center for MVP (no score)
            if g.ball_x < -BALL_SIZE || g.ball_x > CANVAS_WIDTH {
                g.ball_x = CANVAS_WIDTH / 2.0 - BALL_SIZE / 2.0;
                g.ball_y = CANVAS_HEIGHT / 2.0 - BALL_SIZE / 2.0;
                g.ball_vx = BALL_SPEED * if g.ball_vx > 0.0 { -1.0 } else { 1.0 };
                g.ball_vy = BALL_SPEED * 0.5;
            }
        }
    });
}

#[wasm_bindgen]
pub fn get_state() -> String {
    GAME_STATE.with(|s| {
        let state = s.borrow();
        if let Some(ref g) = *state {
            format!("[{},{},{},{}]", g.ball_x, g.ball_y, g.paddle1_y, g.paddle2_y)
        } else {
            "[]".to_string()
        }
    })
}

#[wasm_bindgen]
pub fn set_paddle(player: u8, direction: i8) {
    let dir = direction.max(-1).min(1);
    GAME_STATE.with(|s| {
        let mut state = s.borrow_mut();
        if let Some(ref mut g) = *state {
            match player {
                1 => g.paddle1_dir = dir,
                2 => g.paddle2_dir = dir,
                _ => {}
            }
        }
    });
}
