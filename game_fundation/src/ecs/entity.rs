#[derive(Debug,Clone,Copy,PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

#[derive(Debug)]
struct EntitySlot {
    generation: u32,
    alive: bool,
}

#[derive(Debug, Default)]
pub struct EntityAllocator {
    slots: Vec<EntitySlot>,
    free_indices: Vec<u32>
}

impl EntityAllocator{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn spawn(&mut self) -> Entity {
        if let Some(id) = self.free_indices.pop() {
            let slot = &mut self.slots[id as usize];

            slot.alive = true;

            return Entity {
                id,
                generation: slot.generation,
            };
        }

        let id = self.slots.len() as u32;

        self.slots.push(EntitySlot{
            generation: 0,
            alive: true,
        });

        Entity {
            id,
            generation: 0,
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        let Some(slot) = self.slots.get_mut(entity.id as usize) else {
            return false;
        };

        if !slot.alive || slot.generation != entity.generation {
            return false;
        }

        slot.alive = false;
        slot.generation +=1;

        self.free_indices.push(entity.id);

        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.slots
            .get(entity.id as usize)
            .is_some_and(|slot|{
                slot.alive && slot.generation == entity.generation
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_unique_entities(){
        let mut allocator = EntityAllocator::new();

        let a = allocator.spawn();
        let b = allocator.spawn();

        assert_ne!(a,b);
    }

    #[test]
    fn despawn_entity(){
        let mut allocator = EntityAllocator::new();

        let entity = allocator.spawn();

        assert!(allocator.is_alive(entity));
        assert!(allocator.despawn(entity));
        assert!(!allocator.is_alive(entity));
    }

    #[test]
    fn reuses_id_with_new_generation() {
        let mut allocator = EntityAllocator::new();

        let old = allocator.spawn();

        allocator.despawn(old);

        let new =  allocator.spawn();

        assert_eq!(old.id, new.id);
        assert_ne!(old.generation, new.generation);

        assert!(!allocator.is_alive(old));
        assert!(allocator.is_alive(new));
    }
}