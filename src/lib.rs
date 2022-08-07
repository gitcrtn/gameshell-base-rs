use std::hash::Hash;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use specs::World;
use crate::game::Game;

pub use crate::render::RenderingHelper;
pub use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};
pub use crate::components::{Position, Renderable};
pub use crate::resources::{Core, Time, InputQueue};
pub use crate::audio::AudioContext;
pub use crate::texture::RenderContext;
pub use crate::input::{Keys, get_keys_text};

mod audio;
mod components;
mod constants;
mod game;
mod input;
mod resources;
mod texture;
mod render;

pub trait ImageResource {
    type TextureId: Eq + Hash + PartialEq + Clone + Default;

    fn get_image_ids(&self) -> Vec<Self::TextureId>;
    fn get_image(&self, texture_id: &Self::TextureId) -> &Vec<u8>;

    fn get_text_ids(&self) -> Vec<Self::TextureId>;
    fn get_text(&self, texture_id: &Self::TextureId) -> &String;
    fn get_font_height(&self, texture_id: &Self::TextureId) -> &u32;
    fn get_default_font_id(&self) -> Self::TextureId;

    fn get_tile_position(&self, position: &Position) -> (i32, i32);
}

pub trait SoundResource<'a> {
    type AudioId: Eq + Hash + PartialEq + Clone + Default;

    fn get_audio_ids(&self) -> Vec<Self::AudioId>;
    fn get_audio(&self, audio_id: &Self::AudioId) -> &'static Vec<u8>;
}

pub trait MainLoop<'a, IR: ImageResource, SR: SoundResource<'a>> {
    fn create_world(&self) -> World;
    fn post_create_world(&self, world: &mut World);
    fn setup(&self, render_context: &mut RenderContext<IR>, audio_context: &mut AudioContext<'a, SR>);
    fn update(&self, world: &mut World, audio_context: &mut AudioContext<'a, SR>);
    fn reset_game(&self, world: &mut World, render_context: &mut RenderContext<IR>, audio_context: &mut AudioContext<'a, SR>);
    fn reset_frame(&self, world: &mut World, render_context: &mut RenderContext<IR>, audio_context: &mut AudioContext<'a, SR>);
    fn draw(&self, world: &mut World, render_context: &mut RenderContext<IR>);
}

pub fn run<
        'a,
        IR: ImageResource + Default + 'static,
        SR: SoundResource<'a> + Default + 'static + Copy,
        ML: MainLoop<'a, IR, SR> + 'static>
    (main_loop: ML, image_resource: IR, sound_resource: SR)
    where <IR as ImageResource>::TextureId: Send + Sync {

    let mut game = Game::new(main_loop, image_resource, sound_resource);
    game.setup();

    let mut event_pump = game.get_event_pump();

    'running: loop {
        game.reset_frame();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'running;
                    } else {
                        game.on_key_down(keycode);
                    }
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    game.on_key_up(keycode);
                },
                _ => {}
            }
        }

        game.update();
        game.draw();
        game.sleep_frame();
    }
}