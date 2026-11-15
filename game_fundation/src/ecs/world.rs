use std::any::{TypeId};
use std::collections::HashMap;

use crate::ecs::entity::{Entity, EntityAllocator};
use crate::ecs::components::Component;
use crate::ecs::storage::{ComponentStorage, ErasedStorage};

pub struct World{
    entities: EntityAllocator,
    components: HashMap<TypeId, Box<dyn ErasedStorage>>,
}

impl World{
    pub fn new() -> Self {
        Self{
            entities: EntityAllocator::new(),
            components: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool{
        if !self.entities.is_alive(entity){
            return false;
        }

        for storage in self.components.values_mut() {
            storage.remove_entity(entity);
        }

        self.entities.despawn(entity)
    }

    pub fn is_alive(&self, entity: Entity) ->bool{
        self.entities.is_alive(entity)
    }

    pub fn add<T: Component>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> bool{
        if !self.entities.is_alive(entity){
             eprintln!("Error: intentando agregar un componente a una entidad inválida.");
            return false;
        }

        if self.get::<T>(entity).is_some() {
            eprintln!("Warning: intentando agregar un componente duplicado.");
            return false;
        }

        let storage = self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(||{
                Box::new(ComponentStorage::<T>::new())
            });

        
        let storage = storage.as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .expect("Component storage type mismatch");
        
        storage.insert(entity, component);

        true
    }
    pub fn get<T: Component>(
        &self,
        entity: Entity,
    ) -> Option<&T>{
        if !self.entities.is_alive(entity){
            return None;
        }

        let storage = self.components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<ComponentStorage<T>>()?;

        storage.get(entity)
    }

    pub fn get_mut<T: Component>(
        &mut self,
        entity: Entity,
    ) -> Option<&mut T>{
        if !self.entities.is_alive(entity){
            return None;
        }

        let storage = self.components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()?;

        storage.get_mut(entity)
    }

    pub fn remove<T: Component>(
        &mut self,
        entity: Entity,
    ) -> Option<T> {
        if !self.entities.is_alive(entity){
            return None;
        }

        let storage = self.components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()?;
        
        storage.remove(entity)
    }
    pub fn get_components(&self)->&HashMap<TypeId, Box<dyn ErasedStorage>>{
        &self.components
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[derive(Debug, Eq, PartialEq)]
    struct Health(u32);
    #[derive(Debug, PartialEq)]
    struct Position(f32, f32);

    #[test]
    fn crud_entity(){
        let mut world = World::new();
        
        let player = world.spawn();

        assert!(world.add(player, Health(10)));//create
        assert_eq!(world.get::<Health>(player).unwrap().0,10,"deberia de retornar 10");//read
        world.get_mut::<Health>(player).unwrap().0 = 100;
        assert_eq!(world.get::<Health>(player).unwrap().0,100,"deberia de ser 100");//update

        let removed = world.remove::<Health>(player);
        assert_eq!(removed, Some(Health(100)));
        assert!(world.get::<Health>(player).is_none());
    }
    #[test]
    fn add_component_to_deleted_entity(){
        let mut world = World::new();
        
        let player = world.spawn();

        assert!(world.despawn(player));

        assert!(!world.add(player, Health(10)));

        assert!(world.get::<Health>(player).is_none());
    }
    #[test]
    fn reused_entity_id_does_not_retain_old_components() {
        let mut world = World::new();

        let old_player = world.spawn();
        assert!(world.add(old_player, Health(100)));

        assert!(world.despawn(old_player));

        let new_player = world.spawn();

        assert_eq!(old_player.id, new_player.id);
        assert_ne!(old_player.generation, new_player.generation);

        assert!(world.get::<Health>(new_player).is_none());
    }
    #[test]
    fn despawn_removes_all_components() {
        let mut world = World::new();

        let player = world.spawn();

        assert!(world.add(player, Health(100)));
        assert!(world.add(player, Position(10.0, 20.0)));

        assert!(world.despawn(player));

        assert!(world.get::<Health>(player).is_none());
        assert!(world.get::<Position>(player).is_none());
    }
    #[test]
    fn not_override_components_with_add(){
        let mut world = World::new();

        let player = world.spawn();

        assert!(world.add(player, Health(100)));
        assert!(!world.add(player, Health(10)));
        assert_eq!(world.get::<Health>(player).unwrap().0, 100);
    }
}