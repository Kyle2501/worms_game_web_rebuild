export class PhoenixParallaxBackdrop {
    constructor(width, height) {
        this.width = width;
        this.height = height;
        
        // Precompute distinct mountain ridge offsets for deterministic rendering
        this.ranges = [
            { speed: 0.1, color: '#1a2620', amplitude: 25, frequency: 0.008, baseline: 120 }, // Far background (South Mountain silhouette)
            { speed: 0.3, color: '#25352c', amplitude: 35, frequency: 0.015, baseline: 150 }, // Mid ground (Foothills & ridges)
            { speed: 0.6, color: '#32463b', amplitude: 45, frequency: 0.025, baseline: 180 }  // Near foreground (Desert washes)
        ];
    }

    draw(ctx, cameraX) {
        // 1. Sky Gradient (Warm desert dusk fading into Industrial Moss tones)
        const skyGrad = ctx.createLinearGradient(0, 0, 0, this.height);
        skyGrad.addColorStop(0, '#101612');
        skyGrad.addColorStop(0.6, '#1e2b24');
        skyGrad.addColorStop(1, '#3c5244');
        
        ctx.fillStyle = skyGrad;
        ctx.fillRect(0, 0, this.width, this.height);

        // 2. Render Desert Sun / Glow Halo
        ctx.fillStyle = 'rgba(255, 179, 198, 0.15)'; // Fairy Floss Glitch soft tint
        ctx.beginPath();
        ctx.arc(this.width * 0.75, this.height * 0.3, 40, 0, Math.PI * 2);
        ctx.fill();

        // 3. Render Parallax Mountain Layers
        this.ranges.forEach(range => {
            ctx.fillStyle = range.color;
            ctx.beginPath();
            ctx.moveTo(0, this.height);

            const scrollOffset = cameraX * range.speed;

            for (let x = 0; x <= this.width; x += 2) {
                const sampleX = x + scrollOffset;
                // Combine sine waves for organic mountain ridge profiles
                const y = range.baseline - 
                    Math.sin(sampleX * range.frequency) * range.amplitude -
                    Math.cos(sampleX * range.frequency * 0.5) * (range.amplitude * 0.4);
                
                ctx.lineTo(x, y);
            }

            ctx.lineTo(this.width, this.height);
            ctx.closePath();
            ctx.fill();
        });
    }
}
