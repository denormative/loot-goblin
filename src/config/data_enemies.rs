use serde::{Deserialize, Serialize};

use crate::game::combat::Enemy;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::Resource;
use bevy::reflect::TypeUuid;

#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid, Resource)]
#[serde(deny_unknown_fields)]
#[uuid = "5286cf90-c4a5-40da-a6c7-1081af73d649"]
pub struct EnemiesData {
    pub enemies: Vec<Enemy>,
}

#[derive(Default)]
pub struct EnemiesDataLoader;

impl AssetLoader for EnemiesDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<EnemiesData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["enemies.ron"]
    }
}
