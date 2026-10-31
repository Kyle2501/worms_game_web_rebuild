import { PhoenixParallaxBackdrop } from './parallax_bg.js';

// Inside your main init / run loop:
const backdrop = new PhoenixParallaxBackdrop(canvas.width, canvas.height);
let cameraX = 0;

// Inside gameLoop(currentTime):
// Update camera position smoothly based on worm coordinates
const targetCameraX = Math.max(0, session.get_worm_x() - canvas.width / 4);
cameraX += (targetCameraX - cameraX) * 0.1; // Smooth lerp

// 1. Draw Parallax Background first
backdrop.draw(ctx, cameraX);

// 2. Draw Destructible Terrain bitmap (offset by camera)
ctx.save();
ctx.translate(-Math.floor(cameraX), 0);

ctx.fillStyle = '#4a6b6c'; // Copper Oxidation terrain
for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
        if (terrain[y * width + x] > 0) {
            ctx.fillRect(x * 2, y * 2, 2, 2);
        }
    }
}

// 3. Draw Worm & Projectiles
const wx = session.get_worm_x() * 2;
const wy = session.get_worm_y() * 2;
ctx.fillStyle = '#52b788'; // Bioluminescent Emitter accent
ctx.beginPath();
ctx.arc(wx, wy, 10, 0, Math.PI * 2);
ctx.fill();

ctx.restore();
