use game_fundation::ecs::world::World;

fn main() {
    let mut world = World::new();

    let player = world.spawn();
    let enemy = world.spawn();

    println!("Player: {:?}", player);
    println!("Enemy: {:?}", enemy);

    world.despawn(enemy);

    println!("Player alive: {}", world.is_alive(player));
    println!("Enemy alive: {}", world.is_alive(enemy));
}