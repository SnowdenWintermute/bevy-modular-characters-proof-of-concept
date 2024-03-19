mod attach_part_to_main_skeleton;
mod collect_bones;
mod find_child_with_name_containing;
mod get_main_skeleton_bones_and_armature;
use self::{
    attach_part_to_main_skeleton::attach_part_to_main_skeleton,
    get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature,
};

use super::spawn_scenes::{SceneEntitiesByName, SceneName};
use bevy::prelude::*;

pub fn assemble_parts(
    mut commands: Commands,
    all_entities_with_children: Query<&Children>,
    scene_query: Query<(Entity, &SceneName), With<SceneName>>,
    scene_entities_by_name: Res<SceneEntitiesByName>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
) {
    let (main_skeleton_bones, main_armature_entity) = get_main_skeleton_bones_and_armature(
        scene_entities_by_name,
        &all_entities_with_children,
        &names,
    );

    for (part_scene_entity, part_scene_name) in &scene_query {
        if part_scene_name.0 == "sword.glb" {
            let mut sword_entity_commands = commands.entity(part_scene_entity);
            if let Some(handle_bone) = main_skeleton_bones.get("EquipmentHandle.R") {
                sword_entity_commands.set_parent(*handle_bone);
            }
        } else if part_scene_name.0 != "main_skeleton.glb" {
            attach_part_to_main_skeleton(
                &mut commands,
                &all_entities_with_children,
                &mut transforms,
                &names,
                &part_scene_name.0,
                &part_scene_entity,
                &main_armature_entity,
                &main_skeleton_bones,
            );
        }
    }
}
