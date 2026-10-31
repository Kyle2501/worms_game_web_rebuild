// Inside your requestAnimationFrame gameLoop:
const localMoveDir = currentMoveDir; // Captured from keyboard/touch
const currentTick = globalTick + 1;

// 1. Push local input to WASM and broadcast to peer
session.push_player_input(currentTick, 0.0, 50.0, localMoveDir, false);
if (p2pManager && p2pManager.isConnected) {
    p2pManager.sendInput(currentTick, localMoveDir, 0.0, false);
}

// 2. Step physics simulation forward
session.fixed_step(terrain, width, height);
