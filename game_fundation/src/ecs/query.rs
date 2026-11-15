use crate::ecs::entity::{Entity};
use crate::ecs::world::{World};
use crate::ecs::components::Component;
use crate::ecs::storage::ComponentStorage;

use std::any::TypeId;

pub fn get_entities_by<T: Component>(world:&World)->Vec<Entity>{
    let mut res = Vec::new();
    let Some(storage) = world.get_components()
        .get(&TypeId::of::<T>())
    else{
        return res;
    };
    let Some(components) = storage
        .as_any()
        .downcast_ref::<ComponentStorage<T>>()
    else{
        return res;
    };

    for entity in components.keys() {
        if world.is_alive(*entity) {
            res.push(*entity)
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::get_entities_by;
    use crate::ecs::world::World;

    struct Health(u32);
    struct Position;

    #[test]
    fn returns_entities_with_component() {
        let mut world = World::new();

        let player = world.spawn();
        let chest = world.spawn();

        world.add(player, Health(100));
        assert_eq!(world.get::<Health>(player).unwrap().0, 100);
        world.add(chest, Position);

        let entities = get_entities_by::<Health>(&world);

        assert_eq!(entities.len(), 1);
        assert!(entities.contains(&player));
        assert!(!entities.contains(&chest));
    }

    #[test]
    fn returns_empty_when_storage_does_not_exist() {
        let world = World::new();

        let entities = get_entities_by::<Health>(&world);

        assert!(entities.is_empty());
    }

    #[test]
    fn excludes_despawned_entities() {
        let mut world = World::new();

        let enemy = world.spawn();
        world.add(enemy, Health(50));

        assert!(world.despawn(enemy));

        let entities = get_entities_by::<Health>(&world);

        assert!(!entities.contains(&enemy));
    }
}
