import init, { GameSession } from './pkg/worms_game_web_rebuild.js';

async function run() {
    await init();
    
    const canvas = document.getElementById('game-canvas');
    const ctx = canvas.getContext('2d');
    const session = new GameSession();

    // Mock terrain buffer (320x240 grid: 0 = air, 1 = solid ground)
    const width = 320;
    const height = 240;
    const terrain = new Uint8Array(width * height);
    
    // Generate simple flat floor at y = 200
    for (let y = 200; y < height; y++) {
        for (let x = 0; x < width; x++) {
            terrain[y * width + x] = 1;
        }
    }

    let lastTime = performance.now();
    let accumulator = 0.0;
    const fixedDt = 1000 / 60; // 16.66ms per physics tick
    let globalTick = 0;

    // Input state capture
    let currentMoveDir = 0;
    window.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowLeft') currentMoveDir = -1;
        if (e.key === 'ArrowRight') currentMoveDir = 1;
    });
    window.addEventListener('keyup', (e) => {
        if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') currentMoveDir = 0;
    });

    function gameLoop(currentTime) {
        let frameTime = currentTime - lastTime;
        lastTime = currentTime;
        
        if (frameTime > 250) frameTime = 250; // Prevent spiral of death on lag spikes
        accumulator += frameTime;

        // Push current frame inputs into WASM input queue
        session.push_player_input(globalTick + 1, 0.0, 50.0, currentMoveDir, false);

        while (accumulator >= fixedDt) {
            globalTick++;
            session.fixed_step(terrain, width, height);
            accumulator -= fixedDt;
        }

        // Render pass
        ctx.fillStyle = '#101612'; // Industrial Moss background
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Draw terrain
        ctx.fillStyle = '#4a6b6c'; // Copper Oxidation terrain
        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                if (terrain[y * width + x] > 0) {
                    ctx.fillRect(x * 2, y * 2, 2, 2); // Scaled render
                }
            }
        }

        // Draw Worm
        const wx = session.get_worm_x() * 2;
        const wy = session.get_worm_y() * 2;
        ctx.fillStyle = '#52b788'; // Bioluminescent Emitter accent
        ctx.beginPath();
        ctx.arc(wx, wy, 10, 0, Math.PI * 2);
        ctx.fill();

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();
