// Constants (must match Rust)
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;
const PADDLE_WIDTH = 10;
const PADDLE_HEIGHT = 80;
const BALL_SIZE = 10;

let wasmModule = null;
let lastTime = 0;

async function init() {
  try {
    wasmModule = await import('../pkg/pong_v0.js');
    await wasmModule.default();
    wasmModule.init();
    lastTime = performance.now();
    requestAnimationFrame(gameLoop);
  } catch (err) {
    console.error('WASM load error:', err);
    const el = document.getElementById('error');
    el.textContent = `Failed to load game: ${err.message}`;
    el.style.display = 'block';
  }
}

function gameLoop(now) {
  const deltaMs = now - lastTime;
  lastTime = now;

  if (wasmModule) {
    wasmModule.tick(deltaMs);
    const stateJson = wasmModule.get_state();
    const [ballX, ballY, paddle1Y, paddle2Y] = JSON.parse(stateJson);
    draw(ballX, ballY, paddle1Y, paddle2Y);
  }

  requestAnimationFrame(gameLoop);
}

function draw(ballX, ballY, paddle1Y, paddle2Y) {
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');

  ctx.fillStyle = '#16213e';
  ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

  // Paddles
  ctx.fillStyle = '#eee';
  ctx.fillRect(0, paddle1Y, PADDLE_WIDTH, PADDLE_HEIGHT);
  ctx.fillRect(CANVAS_WIDTH - PADDLE_WIDTH, paddle2Y, PADDLE_WIDTH, PADDLE_HEIGHT);

  // Ball
  ctx.fillStyle = '#eee';
  ctx.fillRect(ballX, ballY, BALL_SIZE, BALL_SIZE);
}

function setPaddleDirection(player, direction) {
  if (wasmModule) {
    wasmModule.set_paddle(player, direction);
  }
}

document.addEventListener('keydown', (e) => {
  switch (e.key) {
    case 'w':
    case 'W':
      e.preventDefault();
      setPaddleDirection(1, -1);  // up
      break;
    case 's':
    case 'S':
      e.preventDefault();
      setPaddleDirection(1, 1);   // down
      break;
    case 'ArrowUp':
      e.preventDefault();
      setPaddleDirection(2, -1);  // up
      break;
    case 'ArrowDown':
      e.preventDefault();
      setPaddleDirection(2, 1);   // down
      break;
  }
});

document.addEventListener('keyup', (e) => {
  switch (e.key) {
    case 'w':
    case 'W':
    case 's':
    case 'S':
      setPaddleDirection(1, 0);
      break;
    case 'ArrowUp':
    case 'ArrowDown':
      setPaddleDirection(2, 0);
      break;
  }
});

init();
