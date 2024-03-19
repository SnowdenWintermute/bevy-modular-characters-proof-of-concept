use self::{
    assemble_parts::assemble_parts,
    link_animations::link_animations,
    paint_cubes_on_joints::paint_cubes_on_joints,
    print_scene_tree::print_scene_tree,
    run_animations::run_animations,
    spawn_scenes::{spawn_scenes, SpawnScenesState},
};
use crate::asset_loader_plugin::AssetLoaderState;
use bevy::prelude::*;
mod assemble_parts;
mod link_animations;
mod paint_cubes_on_joints;
mod print_scene_tree;
mod run_animations;
mod spawn_scenes;

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpawnScenesState>()
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_scenes)
            .add_systems(
                OnEnter(SpawnScenesState::Spawned),
                (link_animations, print_scene_tree, paint_cubes_on_joints),
            )
            .add_systems(
                OnEnter(SpawnScenesState::Done),
                (run_animations, assemble_parts),
            );
    }
}
