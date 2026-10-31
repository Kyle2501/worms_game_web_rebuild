use serde::{Serialize, Deserialize};
use fixed::types::I32F32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetworkMessage {
    PlayerInput {
        tick: u64,
        peer_id: u32,
        move_dir: i8,
        aim_angle: f32, // Serialized for transport
        fire: bool,
    },
    StateSyncCheck {
        tick: u64,
        checksum: u32,
        x: f32,
        y: f32,
    },
}
