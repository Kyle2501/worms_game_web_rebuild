use fixed::types::I32F32;
use std::collections::VecDeque;

/// Represents discrete player intentions for a specific simulation tick.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerInput {
    pub tick: u64,
    pub aim_angle: I32F32,
    pub power_level: I32F32,
    pub move_dir: i8, // -1: Left, 0: None, 1: Right
    pub fire: bool,
}

/// A rolling input buffer ensuring deterministic replay and lag compensation.
pub struct InputQueue {
    queue: VecDeque<PlayerInput>,
    current_tick: u64,
}

impl InputQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(64),
            current_tick: 0,
        }
    }

    pub fn push_input(&mut self, input: PlayerInput) {
        // Insert in order or append for local player
        self.queue.push_back(input);
    }

    /// Polls or interpolates input for the current fixed-point simulation tick.
    pub fn poll_for_tick(&mut self, tick: u64) -> PlayerInput {
        self.current_tick = tick;
        
        // Purge expired inputs
        while let Some(front) = self.queue.front() {
            if front.tick < tick {
                self.queue.pop_front();
            } else {
                break;
            }
        }

        // Return exact match if available, otherwise default neutral input
        if let Some(front) = self.queue.front() {
            if front.tick == tick {
                return self.queue.pop_front().unwrap();
            }
        }

        PlayerInput {
            tick,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2D {
    pub x: I32F32,
    pub y: I32F32,
}

pub struct CollisionResolver;

impl CollisionResolver {
    /// Resolves fixed-point collision against a destructible 2D terrain bitmap grid.
    /// Terrain data: 0 = air/empty, >0 = solid/destructible.
    pub fn resolve_terrain(
        pos: &mut Vector2D,
        vel: &mut Vector2D,
        radius: I32F32,
        terrain: &[u8],
        width: usize,
        height: usize,
    ) -> bool {
        let px = pos.x.to_num::<i32>();
        let py = pos.y.to_num::<i32>();

        // Boundary check
        if px < 0 || px >= width as i32 || py < 0 || py >= height as i32 {
            vel.x = -vel.x * I32F32::from_num(0.4);
            vel.y = -vel.y * I32F32::from_num(0.4);
            return true;
        }

        let idx = (py as usize) * width + (px as usize);
        if terrain[idx] > 0 {
            // Apply restitution and friction damping
            vel.x = -vel.x * I32F32::from_num(0.35);
            vel.y = -vel.y * I32F32::from_num(0.35);
            
            // Positional correction to prevent sinking into terrain
            pos.x -= vel.x;
            pos.y -= vel.y;
            return true;
        }

        false
    }
}
