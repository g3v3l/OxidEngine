use crate::ecs::entity::{Entity, EntityAllocator};

pub struct World{
    entities: EntityAllocator,
}

impl World{
    pub fn new() -> Self {
        Self{
            entities: EntityAllocator::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool{
        self.entities.despawn(entity)
    }

    pub fn is_alive(&self, entity: Entity) ->bool{
        self.entities.is_alive(entity)
    }
}