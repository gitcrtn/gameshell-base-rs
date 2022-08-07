extern crate sdl2;

use std::collections::HashMap;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag, Music};
use crate::SoundResource;

pub struct AudioContext<'a, SR: SoundResource<'a>> {
    pub sounds: HashMap<SR::AudioId, Music<'static>>,
    pub resource: SR,
}

impl<'a, SR: SoundResource<'a>> AudioContext<'a, SR> {
    pub fn play_sound(
        &mut self,
        id: SR::AudioId) {
        self.sounds
            .get(&id)
            .unwrap()
            .play(1)
            .expect("TODO: panic message");
    }
}

pub(crate) fn initialize_sounds<'a, SR: SoundResource<'a> + Default + 'static + Copy>(resource: SR) -> AudioContext<'a, SR> {
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3).unwrap();
    sdl2::mixer::allocate_channels(4);

    let mut context = AudioContext {
        sounds: HashMap::new(),
        resource,
    };

    for audio_id in resource.get_audio_ids() {
        let raw = context.resource.get_audio(&audio_id);
        let music = Music::from_static_bytes(raw).unwrap();
        context.sounds.insert(audio_id, music);
    }

    context
}