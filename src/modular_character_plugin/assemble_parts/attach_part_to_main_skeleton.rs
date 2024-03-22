use crate::modular_character_plugin::{
    assemble_parts::{
        collect_bones::collect_bones,
        find_child_with_name_containing::find_child_with_name_containing,
    },
    AttachedPartsReparentedEntities,
};
use bevy::{prelude::*, utils::HashMap};

pub fn attach_part_to_main_skeleton(
    commands: &mut Commands,
    all_entities_with_children: &Query<&Children>,
    transforms: &mut Query<&mut Transform>,
    names: &Query<&Name>,
    part_scene_name: &String,
    part_scene_entity: &Entity,
    main_armature_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
    attached_parts_reparented_entities: &mut ResMut<AttachedPartsReparentedEntities>,
) {
    println!("attaching part: {}", part_scene_name);
    let mut reparented_entities = Vec::new();

    let root_bone_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "Root",
    );

    let part_armature_option = find_child_with_name_containing(
        all_entities_with_children,
        names,
        &part_scene_entity,
        "CharacterArmature",
    );

    if let Some(part_armature) = part_armature_option {
        let mut part_armature_entity_commands = commands.entity(part_armature);
        if let Ok(mut transform) = transforms.get_mut(part_armature) {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            transform.translation.z = 0.0;
            transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
        }

        reparented_entities.push(part_armature);
        part_armature_entity_commands.set_parent(*main_armature_entity);
    }

    if let Some(root_bone) = root_bone_option {
        let mut part_bones = HashMap::new();
        collect_bones(
            all_entities_with_children,
            names,
            &root_bone,
            &mut part_bones,
        );

        for (name, part_bone) in part_bones {
            let mut entity_commands = commands.entity(part_bone);
            let new_parent_option = main_skeleton_bones.get(&name);

            if let Some(new_parent) = new_parent_option {
                if let Ok(mut transform) = transforms.get_mut(part_bone) {
                    transform.translation.x = 0.0;
                    transform.translation.y = 0.0;
                    transform.translation.z = 0.0;
                    transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
                }

                reparented_entities.push(part_bone);
                entity_commands.set_parent(*new_parent);
            }
        }

        attached_parts_reparented_entities
            .parts_and_entities
            .insert(part_scene_name.clone(), reparented_entities);
    }
}
