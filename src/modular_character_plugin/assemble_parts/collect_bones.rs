use bevy::{prelude::*, utils::HashMap};

pub fn collect_bones(
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
    root_bone: &Entity,
    collected: &mut HashMap<String, Entity>,
) {
    if let Ok(name) = names.get(*root_bone) {
        collected.insert(format!("{}", name), *root_bone);

        if let Ok(children) = all_entities_with_children.get(*root_bone) {
            for child in children {
                collect_bones(all_entities_with_children, names, child, collected)
            }
        }
    }
}
