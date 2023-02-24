#![forbid(unsafe_code)]
#![allow(dead_code)]

extern crate core;

use bevy::prelude::CoreStage::Update;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::egui::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::audio::plugin::MyAudioPlugin;
use crate::config::config_audio::{AudioConfig, AudioConfigLoader};
use crate::config::config_debug::{DebugConfig, DebugConfigLoader};
use crate::config::config_sim::{SimConfig, SimConfigLoader};
use crate::config::data_blueprint::{BlueprintData, BlueprintDataLoader};
use crate::config::data_enemies::{EnemiesData, EnemiesDataLoader};
use crate::config::data_items::{ItemsData, ItemsDataLoader};
use crate::config::data_layout::{LayoutData, LayoutDataLoader};
use crate::config::data_recipes::{RecipesData, RecipesDataLoader};
use crate::config::data_texts::{TextsData, TextsDataLoader};
use crate::game::camera::set_cam_scale;
use crate::game::GamePlugin;
use crate::game_ended::GameEndedPlugin;
use crate::loading::state::LoadingPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::mouse::{Mouse, MousePlugin};
use crate::states::{handle_escape, log_state_changes, AppState};
use crate::transition_state::TransitionPlugin;
use crate::window_event_handler::handle_window;

pub mod animation;
mod audio;
mod config;
pub mod game;
mod game_ended;
mod hud;
mod loading;
mod main_menu;
mod mouse;
mod positioning;
mod states;
mod transition_state;
mod window_event_handler;

/// Will be visible to the user as the name of the window and on the menu screen.
pub const GAME_NAME: &str = "Bag Goblin";

fn main() {
    App::new()
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //     .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.9)))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: GAME_NAME.to_string(),
                resizable: true,
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_loopless_state(AppState::Loading)
        .add_state(game::GameResult::Won)
        .add_plugin(EguiPlugin)
        .add_plugin(MyAudioPlugin)
        .add_plugin(MousePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(TransitionPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(GameEndedPlugin)
        .add_asset::<AudioConfig>()
        .init_asset_loader::<AudioConfigLoader>()
        .add_asset::<DebugConfig>()
        .init_asset_loader::<DebugConfigLoader>()
        .add_asset::<SimConfig>()
        .init_asset_loader::<SimConfigLoader>()
        .add_asset::<BlueprintData>()
        .init_asset_loader::<BlueprintDataLoader>()
        .add_asset::<EnemiesData>()
        .init_asset_loader::<EnemiesDataLoader>()
        .add_asset::<ItemsData>()
        .init_asset_loader::<ItemsDataLoader>()
        .add_asset::<LayoutData>()
        .init_asset_loader::<LayoutDataLoader>()
        .add_asset::<RecipesData>()
        .init_asset_loader::<RecipesDataLoader>()
        .add_asset::<TextsData>()
        .init_asset_loader::<TextsDataLoader>()
        .add_system(handle_window)
        .add_system(log_state_changes)
        .add_system(handle_escape)
        .add_system(set_cam_scale)
        .run();
}
