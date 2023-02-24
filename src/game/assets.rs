use std::collections::HashMap;

use bevy::asset::{Handle, HandleId};
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::config::data_blueprint::BlueprintData;
use crate::config::data_enemies::EnemiesData;
use crate::config::data_items::ItemsData;
use crate::config::data_layout::LayoutData;
use crate::config::data_recipes::RecipesData;
use crate::config::data_texts::TextsData;
use crate::{AudioConfig, DebugConfig, SimConfig};

#[derive(Default, Debug, Resource)]
pub struct AssetStorage {
    textures: HashMap<TextureId, Handle<Image>>,
    atlases: HashMap<TextureId, Handle<TextureAtlas>>,
    sounds: HashMap<SoundId, Vec<Handle<AudioSource>>>,
    music: HashMap<AlbumId, Vec<(Handle<AudioSource>, String)>>,
    fonts: HashMap<FontId, Handle<Font>>,
    pub audio: Handle<AudioConfig>,
    pub debug: Handle<DebugConfig>,
    pub sim: Handle<SimConfig>,
    pub blueprint: Handle<BlueprintData>,
    pub enemies: Handle<EnemiesData>,
    pub items: Handle<ItemsData>,
    pub layout: Handle<LayoutData>,
    pub recipes: Handle<RecipesData>,
    pub texts: Handle<TextsData>,
}

impl AssetStorage {
    pub fn put_texture(&mut self, asset_type: TextureId, asset: Handle<Image>) {
        self.textures.insert(asset_type, asset);
    }

    pub fn texture(&self, asset_type: &TextureId) -> Handle<Image> {
        (*self
            .textures
            .get(asset_type)
            .or_else(|| {
                error!("Texture asset {:?} is missing!", asset_type);
                self.textures.get(&TextureId::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_atlas(&mut self, asset_type: TextureId, asset: Handle<TextureAtlas>) {
        self.atlases.insert(asset_type, asset);
    }
    pub fn atlas(&self, asset_type: &TextureId) -> Handle<TextureAtlas> {
        (*self
            .atlases
            .get(asset_type)
            .or_else(|| {
                error!("Spritesheet asset {:?} is missing!", asset_type);
                self.atlases.get(&TextureId::NotFound)
            })
            .expect("Fallback asset also missing."))
        .clone()
    }

    pub fn put_sfx(&mut self, sound_type: SoundId, asset: Handle<AudioSource>) {
        self.sounds
            .entry(sound_type)
            .or_insert_with(Vec::new)
            .push(asset);
    }
    pub fn sfx(&self, asset_type: &SoundId) -> Option<Handle<AudioSource>> {
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
    pub fn put_music(
        &mut self,
        music_type: AlbumId,
        asset: Handle<AudioSource>,
        file_name: String,
    ) {
        self.music
            .entry(music_type)
            .or_insert_with(Vec::new)
            .push((asset, file_name));
    }
    pub fn album_len(&self, album: &AlbumId) -> usize {
        self.music.get(album).map_or(0, |vec| vec.len())
    }
    pub fn album_track(
        &self,
        album: &AlbumId,
        track: usize,
    ) -> Option<(Handle<AudioSource>, String)> {
        self.music
            .get(album)
            .and_then(|vec| vec.get(track).cloned())
    }
    /// Returns a random track from the given music set.
    pub fn music_random(&self, album: &AlbumId) -> Option<(Handle<AudioSource>, String)> {
        self
            .music
            .get(album)
            .or_else(|| {
                error!("There is no music of type {:?}. Add it to the LoadingConfig to start using them.", album);
                None
            })
            .map(|sounds_of_that_type| {
                let random_index = rand::thread_rng().gen_range(0..sounds_of_that_type.len());
                (*(sounds_of_that_type.get(random_index).expect("Should not panic."))).clone()
            })
    }
    pub fn put_font(&mut self, asset_type: FontId, asset: Handle<Font>) {
        self.fonts.insert(asset_type, asset);
    }
    pub fn font(&self, asset_type: &FontId) -> Handle<Font> {
        (*self.fonts.get(asset_type).expect("Font asset is missing.")).clone()
    }

    pub fn get_all_handle_ids(&self) -> Vec<HandleId> {
        let mut vec = Vec::new();
        vec.append(&mut self.textures.iter().map(|item| item.1.clone().id()).collect());
        // vec.append(&mut self.atlases.iter().map(|item| item.1.clone().id).collect());
        vec.append(&mut self.fonts.iter().map(|item| item.1.clone().id()).collect());
        vec.append(
            &mut self
                .music
                .iter()
                .flat_map(|(_, value)| value.iter())
                .map(|(handle, _)| handle.id())
                .collect(),
        );
        vec.append(
            &mut self
                .sounds
                .iter()
                .flat_map(|(_, value)| value.iter())
                .map(|handle| handle.id())
                .collect(),
        );
        vec.push(self.audio.clone().id());
        vec.push(self.debug.clone().id());
        vec.push(self.sim.clone().id());
        vec.push(self.blueprint.clone().id());
        vec.push(self.enemies.clone().id());
        vec.push(self.items.clone().id());
        vec.push(self.layout.clone().id());
        vec.push(self.recipes.clone().id());
        vec.push(self.texts.clone().id());
        vec
    }
}

/// Contains both a handle to the sprite sheet and the number of the sprite on the sheet.
#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct AtlasId(pub TextureId, pub usize);

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum TextureId {
    /// Fallback sprite. Will be used if the intended sprite failed to load.
    NotFound,
    UiPanelTexture,
    TooltipBackground,
    /// Large image for the menu screen
    Backpack,
    MenuCaveBg,
    Overseer,
    OverseerEyesWhite,
    OverseerIris,
    /// Unused at the moment, but might be used later.
    Cursor,
    RecordPlayer,
    Croissant,
    Athelas,
    HealthPotion,
    Vial,
    TurtleHerb,
    CandleStick,
    EmptyLantern,
    FilledLantern,
    LitLantern,
    FireEssence,
    MediumShield,
    TileSixteen,
    TileEight,
    TileThirtyTwo,
    // New items
    Scroll,
    HerbRed,
    HerbGreen,
    HerbViolet,
    EssenceMight,
    EssenceVitality,
    EssenceAlacrity,
    FlaskHealing,
    FlaskStrength,
    FlaskSkill,
    FlaskToughness,
    SwordRusty,
    Sword,
    SwordMasterwork,
    SwordOfWounding,
    MasterworkSwordOfWounding,
    SwordOfSpeed,
    MasterworkSwordOfSpeed,
    ShieldRusty,
    Shield,
    ShieldMasterwork,
    ArmorRusty,
    Armor,
    ArmorMasterwork,
    AxeRusty,
    Axe,
    AxeMasterwork,
    CombineButton,
}

impl Default for TextureId {
    fn default() -> Self {
        TextureId::NotFound
    }
}

/// Identifies a type of sound effect. Each of these sound types could be represented by any number
/// of sound files that the game will randomly pick from.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SoundId {
    EnterRat,
    EnterLittleMonster,
    EnterBigMonster,
    EnterSkeleton,
    EnterZombie,
    DoorCreak,
    GoblinAhah,
    SlashHit,
    SwordClang,
    WaterDripping,
    /// Combining potions and stuff
    CombineAlchemy,
    /// Combining swords and stuff.
    CombineSmithing,
    /// Failing to combine.
    CombineCant,
}

/// Identifies a music track or album.
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum AlbumId {
    Jazz,
    Ominous,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum FontId {
    FiraSansLight,
    FiraSansRegular,
    /// Thicker than Regular.
    FiraSansMedium,
    FiraSansBold,
    FiraSansItalic,
}
