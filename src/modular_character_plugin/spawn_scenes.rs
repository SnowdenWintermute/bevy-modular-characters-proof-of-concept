use crate::asset_loader_plugin::MyAssets;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum SpawnScenesState {
    #[default]
    Spawning,
    Spawned,
    Done,
}

#[derive(Resource, Debug)]
pub struct SceneEntitiesByName(pub HashMap<String, Entity>);

#[derive(Resource, Debug)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

pub fn spawn_scenes(
    mut commands: Commands,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnScenesState>>,
) {
    let mut animations = HashMap::new();
    let mut scene_entities_by_name = HashMap::new();

    // SPAWN SCENES
    for (name, gltf_handle) in &asset_pack.gltf_files {
        if let Some(gltf) = assets_gltf.get(gltf_handle) {
            println!("SPAWING");
            let mut transform = Transform::from_xyz(0.0, 0.0, 0.0);

            if name == "sword.glb" {
                transform.scale = Vec3::splat(0.1)
            }

            let entity_commands = commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    transform,
                    ..Default::default()
                },
                SceneName(name.clone()),
            ));

            let entity = entity_commands.id();
            scene_entities_by_name.insert(name.clone(), entity);

            for named_animation in gltf.named_animations.iter() {
                println!("inserting animation: {}", named_animation.0);
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }
        }
    }

    commands.insert_resource(Animations(animations));
    commands.insert_resource(SceneEntitiesByName(scene_entities_by_name));

    next_state.set(SpawnScenesState::Spawned)
}
