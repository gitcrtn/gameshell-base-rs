extern crate sdl2;

use std::collections::HashMap;
use imagesize::blob_size;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::Sdl;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_TITLE};
use crate::ImageResource;

fn draw_text_to_canvas(
    canvas: &mut WindowCanvas, texture: &Texture, text: String, x: i32, y: i32, font_height: u32, scale: f32) {
    for (i, char) in text.chars().collect::<Vec<char>>().iter().enumerate() {
        let index = u32::from(*char) - 0x20;
        let dst_height = if scale == 1.0 { font_height } else { (font_height as f32 * scale) as u32 };
        let src = Rect::new(
            (font_height * index) as i32, 0, font_height, font_height);
        let dst = Rect::new(
            x + (dst_height * (i as u32)) as i32, y, dst_height.clone(), dst_height.clone());
        canvas.copy(texture, src, dst).unwrap();
    }
}

pub struct RenderContext<IR: ImageResource> {
    pub canvas: WindowCanvas,
    pub textures: HashMap<IR::TextureId, Texture>,
    pub texture_sizes: HashMap<IR::TextureId, (u32, u32)>,
    pub resource: IR,
}

impl<IR: ImageResource> RenderContext<IR> {
    pub fn new(sdl_context: &Sdl, resource: IR) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        let window = video_subsystem.window(
            WINDOW_TITLE,
            SCREEN_WIDTH,
            SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .software()
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();

        let mut textures = {
            let mut m = HashMap::new();
            for texture_id in resource.get_image_ids() {
                let raw = resource.get_image(&texture_id);
                let texture = texture_creator.load_texture_bytes(raw).unwrap();
                m.insert(texture_id, texture);
            }
            m
        };

        let mut texture_sizes = {
            let mut m = HashMap::new();
            for texture_id in resource.get_image_ids() {
                let raw = resource.get_image(&texture_id);
                let size = blob_size(raw).unwrap();
                m.insert(texture_id, (size.width as u32, size.height as u32));
            }
            m
        };

        let default_font_id = resource.get_default_font_id();
        let font_height = resource.get_font_height(&default_font_id);

        for texture_id in resource.get_text_ids() {
            let text = resource.get_text(&texture_id);
            let width = font_height.clone() * (text.len() as u32);
            let height = font_height.clone();
            let mut texture = texture_creator
                .create_texture_target(
                    texture_creator.default_pixel_format(), width, height)
                .unwrap();
            let _ = canvas.with_texture_canvas(&mut texture, |texture_canvas| {
                let font_texture = textures.get(&default_font_id).unwrap();
                draw_text_to_canvas(texture_canvas, font_texture, text.clone(), 0, 0, font_height.clone(), 1.0);
            });
            texture_sizes.insert(texture_id.clone(), (width, height));
            textures.insert(texture_id.clone(), texture);
        }

        RenderContext {
            canvas,
            textures,
            texture_sizes,
            resource,
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT)).unwrap();
    }

    pub fn draw_bg(&mut self, id: IR::TextureId) {
        let texture = self.textures.get(&id).unwrap();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.copy(texture, None, None).unwrap();
    }

    pub fn draw(&mut self, id: IR::TextureId, x: i32, y: i32) {
        let texture = self.textures.get(&id).unwrap();
        let size = self.texture_sizes.get(&id).unwrap();
        let area = Rect::new(x, y, size.0, size.1);
        self.canvas.copy(texture, None, area).unwrap();
    }

    pub fn draw_text(&mut self, text: String, x: i32, y: i32) {
        let default_font_id = self.resource.get_default_font_id();
        let height = self.resource.get_font_height(&default_font_id);
        let texture = self.textures.get(&default_font_id).unwrap();
        draw_text_to_canvas(&mut self.canvas, texture, text, x, y, height.clone(), 1.0);
    }

    pub fn draw_text_font(&mut self, text: String, x: i32, y: i32, font: IR::TextureId) {
        let height = self.resource.get_font_height(&font);
        let texture = self.textures.get(&font).unwrap();
        draw_text_to_canvas(&mut self.canvas, texture, text, x, y, height.clone(), 1.0);
    }

    pub fn draw_text_scale(&mut self, text: String, x: i32, y: i32, scale: f32) {
        let default_font_id = self.resource.get_default_font_id();
        let height = self.resource.get_font_height(&default_font_id);
        let texture = self.textures.get(&default_font_id).unwrap();
        draw_text_to_canvas(&mut self.canvas, texture, text, x, y, height.clone(), scale);
    }
}

pub(crate) fn initialize_render<IR: ImageResource>(sdl_context: &Sdl, resource: IR) -> RenderContext<IR> {
    RenderContext::new(sdl_context, resource)
}