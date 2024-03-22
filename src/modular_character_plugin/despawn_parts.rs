use super::{
    assemble_parts::despawn_attached_part::despawn_attached_part,
    spawn_scenes::SceneEntitiesByName, AttachedPartsReparentedEntities,
};
use bevy::prelude::*;

pub fn despawn_parts(
    mut commands: Commands,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    mut scene_entities_by_name: ResMut<SceneEntitiesByName>,
) {
    let scene_entity_option = scene_entities_by_name.0.remove("scifi_torso.glb");
    if let Some(scene_entity) = scene_entity_option {
        despawn_attached_part(
            &mut commands,
            &"scifi_torso.glb".to_string(),
            &scene_entity,
            &mut attached_parts_reparented_entities,
        )
    }
}
