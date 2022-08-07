use specs::{Read, ReadStorage, RunNow, System, World, WorldExt, Write};
use gameshell_base::{AudioContext, get_keys_text, ImageResource, InputQueue, Keys, MainLoop, Position, Renderable, RenderContext, RenderingHelper, SCREEN_HEIGHT, SoundResource, Time};

pub const TILE_WIDTH: i32 = 24;
pub const MAP_OFFSET_X: i32 = 12;
pub const MAP_OFFSET_Y: i32 = 12;
pub const MAP_WIDTH: u8 = 8;
pub const MAP_HEIGHT: u8 = 9;
pub const TEXT_OFFSET_X: i32 = 10;
pub const TEXT_FOOTER_Y: i32 = SCREEN_HEIGHT as i32 - 24;

#[derive(Eq, Hash, PartialEq, Clone, Default)]
pub enum AudioId {
    #[default]
    Correct,
}

mod sound_context {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use crate::AudioId;

    lazy_static! {
        pub static ref SOUNDS: HashMap<AudioId, Vec<u8>> = {
            let mut m = HashMap::new();
            m.insert(AudioId::Correct, include_bytes!("./resources/correct.mp3").to_vec());
            m
        };
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Default)]
pub enum TextureId {
    // images
    #[default]
    HelloWorld,

    // font images
    FontDefault,

    // texts
    TextSound,
}

mod image_context {
    use std::collections::HashMap;
    use lazy_static::lazy_static;
    use crate::TextureId;

    lazy_static! {
        pub static ref IMAGES: HashMap<TextureId, Vec<u8>> = {
            let mut m = HashMap::new();
            m.insert(TextureId::HelloWorld, include_bytes!("./resources/hello_world.png").to_vec());
            m.insert(TextureId::FontDefault, include_bytes!("./resources/font_ponderosa_12px.png").to_vec());
            m
        };

        pub static ref FONT_HEIGHTS: HashMap<TextureId, u32> = {
            let mut m = HashMap::new();
            m.insert(TextureId::FontDefault, 12);
            m
        };

        pub static ref TEXTS: HashMap<TextureId, String> = {
            let mut m = HashMap::new();
            m.insert(TextureId::TextSound, "A: Sound".to_string());
            m
        };
    }
}

#[derive(Default, Clone, Copy)]
pub struct Sound;

impl<'a> SoundResource<'a> for Sound {
    type AudioId = AudioId;

    fn get_audio_ids(&self) -> Vec<Self::AudioId> {
        let ref sounds_raw = sound_context::SOUNDS;
        sounds_raw.keys().cloned().collect::<Vec<AudioId>>()
    }

    fn get_audio(&self, audio_id: &Self::AudioId) -> &'static Vec<u8> {
        let ref sounds_raw = sound_context::SOUNDS;
        sounds_raw.get(audio_id).unwrap()
    }
}

#[derive(Default)]
pub struct Image;

impl ImageResource for Image {
    type TextureId = TextureId;

    fn get_image_ids(&self) -> Vec<Self::TextureId> {
        let ref images_raw = image_context::IMAGES;
        images_raw.keys().cloned().collect::<Vec<TextureId>>()
    }

    fn get_image(&self, texture_id: &Self::TextureId) -> &Vec<u8> {
        let ref images_raw = image_context::IMAGES;
        images_raw.get(texture_id).unwrap()
    }

    fn get_text_ids(&self) -> Vec<Self::TextureId> {
        let ref texts_raw = image_context::TEXTS;
        texts_raw.keys().cloned().collect::<Vec<TextureId>>()
    }

    fn get_text(&self, texture_id: &Self::TextureId) -> &String {
        let ref texts_raw = image_context::TEXTS;
        texts_raw.get(texture_id).unwrap()
    }

    fn get_font_height(&self, texture_id: &Self::TextureId) -> &u32 {
        let ref font_heights = image_context::FONT_HEIGHTS;
        font_heights.get(texture_id).unwrap()
    }

    fn get_default_font_id(&self) -> Self::TextureId {
        TextureId::FontDefault
    }

    fn get_tile_position(&self, position: &Position) -> (i32, i32) {
        (
            position.x as i32 * TILE_WIDTH + MAP_OFFSET_X,
            position.y as i32 * TILE_WIDTH + MAP_OFFSET_Y,
        )
    }
}

pub struct RenderingSystem<'a> {
    pub helper: RenderingHelper<'a, Image>,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable<Image>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            time,
            gameplay,
            positions,
            renderables,
        ) = data;
        self.helper.draw_renderables(&positions, &renderables, &time);
        self.helper.context.draw_bg(TextureId::HelloWorld);
        self.helper.context.draw_text(format!("FPS:  {:.2}", time.fps_avg), TEXT_OFFSET_X, 16);
        self.helper.context.draw_text(format!("TIME: {:>02}:{:>02}", time.minutes, time.seconds), TEXT_OFFSET_X, 30);
        self.helper.context.draw(TextureId::TextSound, TEXT_OFFSET_X, TEXT_FOOTER_Y);
        self.helper.context.draw_text(format!("KEYS: {}", gameplay.keys_pressed_text.clone()), TEXT_OFFSET_X, 44);
        self.helper.context.present();
    }
}

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Default)]
pub struct Gameplay {
    pub keys_pressed_text: String,
}

#[derive(Debug)]
pub struct KeysDowned {
    pub text: String,
}

#[derive(Debug)]
pub enum Event {
    Beep,
    KeysDowned(KeysDowned),
}

pub struct EventSystem<'a, 'b> {
    pub audio_context: &'a mut AudioContext<'b, Sound>,
}

impl<'a, 'b> System<'a> for EventSystem<'a, 'b> {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, Gameplay>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut event_queue,
            mut gameplay,
        ) = data;

        let mut new_events = Vec::new();
        let mut keys_pressed = false;

        for event in event_queue.events.drain(..) {
            match event {
                Event::Beep => {
                    self.audio_context.play_sound(AudioId::Correct);
                },
                Event::KeysDowned(KeysDowned { text }) => {
                    keys_pressed = true;
                    gameplay.keys_pressed_text = text;
                },
            }
        }

        if !keys_pressed {
            gameplay.keys_pressed_text.clear();
        }

        event_queue.events.append(&mut new_events);
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut events,
            mut input_queue,
        ) = data;

        if let Some(key) = input_queue.keys_pressed.pop() {
            if key == Keys::A {
                events.events.push(Event::Beep);
            }
        }

        if input_queue.keys_downed.len() > 0 {
            let text = get_keys_text(&input_queue.keys_downed);
            events.events.push(Event::KeysDowned(KeysDowned { text }));
        }
    }
}

struct Loop;
impl<'a> MainLoop<'a, Image, Sound> for Loop {
    fn create_world(&self) -> World {
        let mut world = World::new();
        world.insert(EventQueue::default());
        world.insert(Gameplay::default());
        world
    }

    fn post_create_world(&self, _world: &mut World) {
        // empty
    }

    fn setup(&self, render_context: &mut RenderContext<Image>, audio_context: &mut AudioContext<'a, Sound>) {
        render_context.draw_bg(TextureId::HelloWorld);
        render_context.present();
        audio_context.play_sound(AudioId::Correct);
    }

    fn update(&self, world: &mut World, audio_context: &mut AudioContext<'a, Sound>) {
        {
            let mut is = InputSystem {};
            is.run_now(world);
        }

        {
            let mut es = EventSystem {
                audio_context,
            };
            es.run_now(world);
        }
    }

    fn reset_game(&self, _world: &mut World, _render_context: &mut RenderContext<Image>, _audio_context: &mut AudioContext<'a, Sound>) {
        // empty
    }

    fn reset_frame(&self, _world: &mut World, _render_context: &mut RenderContext<Image>, _audio_context: &mut AudioContext<'a, Sound>) {
        // empty
    }

    fn draw(&self, world: &mut World, render_context: &mut RenderContext<Image>) {
        {
            let mut rs = RenderingSystem {
                helper: RenderingHelper { context: render_context },
            };
            rs.run_now(world);
        }
    }
}

pub fn main() {
    let main_loop = Loop {};
    let image = Image {};
    let sound = Sound {};
    gameshell_base::run(main_loop, image, sound);
}