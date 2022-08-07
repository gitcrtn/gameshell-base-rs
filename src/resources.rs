use specs::World;
use crate::input::Keys;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<Keys>,
    pub keys_pulled: Vec<Keys>,
    pub keys_downed: Vec<Keys>,
}

#[derive(Default)]
pub struct Core {
    pub pause_time: bool,
    pub reset: bool,
}

#[derive(Default)]
pub struct Time {
    pub last_ticks: u32,
    pub prev_ticks: u32,
    pub start_ticks: u32,
    pub last_performance_counter: u64,
    pub prev_performance_counter: u64,
    pub fps_queue: Vec<f64>,
    pub fps_ticks_cache: u32,
    pub fps_avg: f64,
    pub minutes: u8,
    pub seconds: u8,
}

pub(crate) fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Core::default());
    world.insert(Time::default());
}