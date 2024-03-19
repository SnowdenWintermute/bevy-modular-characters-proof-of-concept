use asset_loader_plugin::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
mod asset_loader_plugin;
mod camera_plugin;
mod modular_character_plugin;
use camera_plugin::CameraPlugin;
use modular_character_plugin::ModularCharacterPlugin;

fn main() {
    App::new()
        // BEVY
        .insert_resource(ClearColor(Color::rgb(0.1, 0.15, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        // SELF MADE
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ModularCharacterPlugin)
        // EXTERNAL DEPENDENCIES
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(BillboardPlugin)
        .run();
}
