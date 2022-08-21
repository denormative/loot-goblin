use std::collections::HashMap;

use bevy::asset::{Handle, HandleId};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use bevy_kira_audio::AudioSource;
use rand::Rng;

#[derive(Default, Debug)]
pub struct AssetStorage {
    textures: HashMap<SpriteType, Handle<Image>>,
    atlases: HashMap<SpriteType, Handle<TextureAtlas>>,
    sounds: HashMap<SoundType, Vec<Handle<AudioSource>>>,
    music: HashMap<MusicType, Vec<Handle<AudioSource>>>,
}

impl AssetStorage {
    pub fn put_texture(&mut self, asset_type: SpriteType, asset: Handle<Image>) {
        self.textures.insert(asset_type, asset);
    }
    pub fn texture(&self, asset_type: &SpriteType) -> Handle<Image> {
        (*self
            .textures
            .get(asset_type)
            .or_else(|| {
                error!("Texture asset {:?} is missing!", asset_type);
                self.textures.get(&SpriteType::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_atlas(&mut self, asset_type: SpriteType, asset: Handle<TextureAtlas>) {
        self.atlases.insert(asset_type, asset);
    }
    pub fn atlas(&self, asset_type: &SpriteType) -> Handle<TextureAtlas> {
        (*self
            .atlases
            .get(asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.atlases.get(&SpriteType::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_sound(&mut self, sound_type: SoundType, asset: Handle<AudioSource>) {
        self.sounds
            .entry(sound_type)
            .or_insert_with(Vec::new)
            .push(asset);
    }
    pub fn sound(&self, asset_type: &SoundType) -> Option<Handle<AudioSource>> {
        self
            .sounds
            .get(asset_type)
            .or_else(|| {
                error!("There are no sounds of type {:?}. Add them to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*(sounds_of_that_type.get(random_index).expect("Should not panic."))).clone()
            })
    }
    pub fn put_music(&mut self, music_type: MusicType, asset: Handle<AudioSource>) {
        self.music
            .entry(music_type)
            .or_insert_with(Vec::new)
            .push(asset);
    }
    pub fn music(&self, asset_type: &MusicType) -> Option<Handle<AudioSource>> {
        self
            .music
            .get(asset_type)
            .or_else(|| {
                error!("There is no music of type {:?}. Add it to the LoadingConfig to start using them.", asset_type);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*(sounds_of_that_type.get(random_index).expect("Should not panic."))).clone()
            })
    }
    pub fn get_all_handle_ids(&self) -> Vec<HandleId> {
        let vec = self.textures.iter().map(|item| item.1.clone().id).collect();
        // let vec = self.sounds.iter()
        //     .flat_map(|item| {
        //         item.1.clone()
        //     })
        //     .collect();
        vec
    } //TODO
}

/// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct AtlasType(pub SpriteType, pub usize);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpriteType {
    /// Fallback sprite.
    NotFound,
    Placeholder,
    Croissant,
    GreenSquare,
    SelectionSquare,
}

impl Default for SpriteType {
    fn default() -> Self {
        SpriteType::NotFound
    }
}

/// Identifies a type of sound effect. Each of these sound types could be represented by any number
/// of sound files that the game will randomly pick from.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SoundType {
    Placeholder,
}

/// Identifies a music track.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum MusicType {
    Placeholder,
}
