use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::any::Any;

use crate::ecs::entity::Entity;
use crate::ecs::components::Component;

pub struct ComponentStorage<T: Component>{
    components: HashMap<Entity, T>,
}

impl<T: Component> ComponentStorage<T>{
    pub fn new() -> Self{
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T){
        self.components.insert(entity, component);
    }

    pub fn get(&self, entity: Entity) ->Option<&T>{
        self.components.get(&entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T>{
        self.components.get_mut(&entity)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T>{
        self.components.remove(&entity)
    }

    pub fn keys(&self) -> Keys<'_, Entity, T>{
        self.components.keys()
    }
}

pub trait ErasedStorage: Any {
    fn remove_entity(&mut self, entity: Entity);

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;
}

impl<T: Component> ErasedStorage for ComponentStorage<T>{
    fn remove_entity(&mut self, entity: Entity) {
        self.remove(entity);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}