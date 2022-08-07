use specs::{World, WorldExt};
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl, TimerSubsystem};
use std::time::Duration;
use crate::audio::{AudioContext, initialize_sounds};
use crate::input::{initialize_input, InputContext};
use crate::components::register_components;
use crate::{MainLoop, ImageResource, SoundResource};
use crate::resources::{Core, InputQueue, register_resources, Time};
use crate::texture::{initialize_render, RenderContext};

const NANO_1SEC: u32 = 1_000_000_000u32;
const NANO_1SEC_F64: f64 = NANO_1SEC as f64;
const NANO_FRAME_SEC: u32 = NANO_1SEC / 60;

pub struct Game<'a, IR: ImageResource + Default, SR: SoundResource<'a> + Default> {
    pub world: World,
    pub audio_context: AudioContext<'a, SR>,
    pub render_context: RenderContext<IR>,
    pub input_context: InputContext,
    pub timer_subsystem: TimerSubsystem,
    pub main_loop: Box<dyn MainLoop<'a, IR, SR>>,
    sdl_context: Sdl,
}

impl<'a, IR: ImageResource + Default + 'static, SR: SoundResource<'a> + Default + 'static + Copy> Game<'a, IR, SR> {
    pub fn new(main_loop: impl MainLoop<'a, IR, SR> + 'static, image_resource: IR, sound_resource: SR)
        -> Self where <IR as ImageResource>::TextureId: Send + Sync {

        let sdl_context = sdl2::init().unwrap();
        let main_loop = Box::new(main_loop);

        let mut world: World = main_loop.create_world();
        register_components::<IR>(&mut world);
        register_resources(&mut world);

        main_loop.post_create_world(&mut world);

        let audio_context = initialize_sounds::<SR>(sound_resource);
        let render_context = initialize_render::<IR>(&sdl_context, image_resource);
        let input_context = initialize_input();
        let timer_subsystem = sdl_context.timer().unwrap();

        Game {
            world,
            audio_context,
            render_context,
            input_context,
            timer_subsystem,
            main_loop,
            sdl_context,
        }
    }

    pub fn get_event_pump(&self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    pub fn setup(&mut self) {
        self.main_loop.setup(&mut self.render_context, &mut self.audio_context);
    }

    pub fn update(&mut self) {
        {
            let mut input_queue = self.world.write_resource::<InputQueue>();
            input_queue.keys_pressed.append(&mut self.input_context.get_keys_pressed());
            input_queue.keys_pulled.append(&mut self.input_context.get_keys_pulled());
            input_queue.keys_downed.clear();
            input_queue.keys_downed.append(&mut self.input_context.get_keys_downed());
        }

        self.main_loop.update(&mut self.world, &mut self.audio_context);
    }

    pub fn draw(&mut self) {
        self.main_loop.draw(&mut self.world, &mut self.render_context);
    }

    fn reset_level_if_need(&mut self) {
        let reset = {
            let gameplay = self.world.read_resource::<Core>();
            gameplay.reset
        };

        if reset {
            {
                let mut gameplay = self.world.write_resource::<Core>();
                gameplay.reset = false;

                let mut time = self.world.write_resource::<Time>();
                time.start_ticks = time.last_ticks.clone();
            }

            {
                self.main_loop.reset_game(&mut self.world, &mut self.render_context, &mut self.audio_context);
            }
        }
    }

    pub fn reset_frame(&mut self) {
        self.input_context.reset_frame();
        self.reset_level_if_need();
        self.main_loop.reset_frame(&mut self.world, &mut self.render_context, &mut self.audio_context);
    }

    pub fn on_key_down(&mut self, keycode: Keycode) {
        self.input_context.on_key_down(keycode);
    }

    pub fn on_key_up(&mut self, keycode: Keycode) {
        self.input_context.on_key_up(keycode);
    }

    pub fn sleep_frame(&mut self) {
        let mut time = self.world.write_resource::<Time>();
        let core = self.world.read_resource::<Core>();

        time.prev_performance_counter = time.last_performance_counter.clone();
        time.prev_ticks = time.last_ticks.clone();
        time.last_performance_counter = self.timer_subsystem.performance_counter();

        let end = &time.last_performance_counter;
        let start = &time.prev_performance_counter;
        let elapsed = (*end - *start) as f64 / self.timer_subsystem.performance_frequency() as f64;
        let nano_sec = (NANO_1SEC_F64 * elapsed) as u32;

        if nano_sec < NANO_FRAME_SEC {
            std::thread::sleep(Duration::new(0, NANO_FRAME_SEC - nano_sec));
        }

        time.last_performance_counter = self.timer_subsystem.performance_counter();
        time.last_ticks = self.timer_subsystem.ticks();
        time.fps_ticks_cache += time.last_ticks - time.prev_ticks;

        if !core.pause_time {
            let seconds = (time.last_ticks - time.start_ticks) / 1000;
            time.seconds = (seconds % 60) as u8;
            time.minutes = (seconds / 60) as u8;
        }

        let end = &time.last_performance_counter;
        let start = &time.prev_performance_counter;
        let elapsed = (*end - *start) as f64 / self.timer_subsystem.performance_frequency() as f64;
        let fps = 1.0 / elapsed;
        if time.fps_avg == 0.0 {
            time.fps_avg = fps;
        } else {
            time.fps_queue.push(fps);
        }
        if time.fps_ticks_cache >= 1000 {
            time.fps_avg = time.fps_queue.iter().sum::<f64>() / time.fps_queue.len() as f64;
            time.fps_ticks_cache = 0;
            time.fps_queue.clear();
        }
    }
}