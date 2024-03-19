use super::{
    link_animations::AnimationEntityLink,
    spawn_scenes::{Animations, SceneEntitiesByName, SceneName},
};
use bevy::prelude::*;

pub fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    scene_and_animation_player_link_query: Query<
        (&SceneName, &AnimationEntityLink),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
) {
    let main_skeleton_scene_entity = scene_entities_by_name
        .0
        .get("main_skeleton.glb")
        .expect("the scene to be registered");

    let (_, animation_player_entity_link) = scene_and_animation_player_link_query
        .get(*main_skeleton_scene_entity)
        .expect("the scene to exist");

    let mut animation_player = animation_player_query
        .get_mut(animation_player_entity_link.0)
        .expect("to have an animation player on the main skeleton");

    animation_player
        .play(
            animations
                .0
                .get("Sword_Slash")
                .expect("to have an animation by this name")
                .clone_weak(),
        )
        .repeat()
        .set_speed(0.5);
}
