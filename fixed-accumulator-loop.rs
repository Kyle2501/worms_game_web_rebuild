// Rust / WASM main loop structure
let mut last_time = performance_now();
let mut accumulator = 0.0;
let fixed_dt = 1.0 / 60.0; // 60 Hz physics tick

let mut game_loop = move || {
    let current_time = performance_now();
    let frame_time = (current_time - last_time) / 1000.0;
    last_time = current_time;
    
    // Prevent spiral of death on lag spikes
    let frame_time = if frame_time > 0.25 { 0.25 } else { frame_time };
    accumulator += frame_time;

    // Drain input queue for the upcoming tick batch
    input_system.poll_pending_inputs();

    while accumulator >= fixed_dt {
        physics_world.step(fixed_dt);
        accumulator -= fixed_dt;
    }

    renderer.draw(&physics_world);
};
