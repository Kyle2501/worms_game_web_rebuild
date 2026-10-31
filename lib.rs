use wasm_bindgen::prelude::*;
use fixed::types::I32F32;

mod physics_input;
use physics_input::{InputQueue, CollisionResolver, Vector2D, PlayerInput};

[wasm_bindgen]
pub struct GameSession {
    input_queue: InputQueue,
    worm_pos: Vector2D,
    worm_vel: Vector2D,
    current_tick: u64,
}

[wasm_bindgen]
impl GameSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            input_queue: InputQueue::new(),
            worm_pos: Vector2D {
                x: I32F32::from_num(160),
                y: I32F32::from_num(50),
            },
            worm_vel: Vector2D {
                x: I32F32::from_num(0),
                y: I32F32::from_num(0),
            },
            current_tick: 0,
        }
    }

    /// Push input received from JS input listeners into the deterministic queue
    pub fn push_player_input(&mut self, tick: u64, aim: f32, power: f32, dir: i8, fire: bool) {
        let input = PlayerInput {
            tick,
            aim_angle: I32F32::from_num(aim),
            power_level: I32F32::from_num(power),
            move_dir: dir,
            fire,
        };
        self.input_queue.push_input(input);
    }

    /// Step the simulation forward by one fixed tick (dt = 1/60s)
    pub fn fixed_step(&mut self, terrain: &[u8], width: usize, height: usize) {
        self.current_tick += 1;
        let input = self.input_queue.poll_for_tick(self.current_tick);

        // Apply movement vector based on input
        if input.move_dir != 0 {
            let speed = I32F32::from_num(1.5);
            self.worm_vel.x = speed * I32F32::from_num(input.move_dir);
        } else {
            // Apply friction/drag
            self.worm_vel.x *= I32F32::from_num(0.85);
        }

        // Apply gravity
        let gravity = I32F32::from_num(0.2);
        self.worm_vel.y += gravity;

        // Update positions
        self.worm_pos.x += self.worm_vel.x;
        self.worm_pos.y += self.worm_vel.y;

        // Resolve collisions with terrain bitmap
        CollisionResolver::resolve_terrain(
            &mut self.worm_pos,
            &mut self.worm_vel,
            I32F32::from_num(6),
            terrain,
            width,
            height,
        );
    }

    pub fn get_worm_x(&f32) -> f32 {
        self.worm_pos.x.to_num::<f32>()
    }

    pub fn get_worm_y(&f32) -> f32 {
        self.worm_pos.y.to_num::<f32>()
    }
}
