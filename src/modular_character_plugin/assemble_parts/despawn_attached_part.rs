use crate::modular_character_plugin::{
    spawn_scenes::SceneEntitiesByName, AttachedPartsReparentedEntities,
};
use bevy::prelude::*;

pub fn despawn_attached_part(
    commands: &mut Commands,
    part_scene_name: &String,
    part_scene_entity: &Entity,
    attached_parts_reparented_entities: &mut ResMut<AttachedPartsReparentedEntities>,
) {
    let part_scene_entity_commands = commands.entity(*part_scene_entity);

    part_scene_entity_commands.despawn_recursive();
    let reparented_entities_option = attached_parts_reparented_entities
        .parts_and_entities
        .remove(part_scene_name);
    if let Some(reparented_entities) = reparented_entities_option {
        for entity in reparented_entities {
            let commands = commands.entity(entity);
            commands.despawn_recursive();
        }
    }
}
