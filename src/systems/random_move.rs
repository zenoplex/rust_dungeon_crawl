use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    // Not including pinning down to Player so to check enemy will not collide with other enemies
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut attacked = false;

    movers.iter(ecs).for_each(|(enemy_entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        positions
            .iter(ecs)
            .filter(|(_, pos, _)| **pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((
                        (),
                        WantsToAttack {
                            victim: *victim,
                            attacker: *enemy_entity,
                        },
                    ));
                };
                // Leaving attacked true even if no successful attack to prevent enemies to collide
                attacked = true;
            });

        if !attacked {
            commands.push((
                (),
                WantsToMove {
                    entity: *enemy_entity,
                    destination,
                },
            ));
        }
    })
}
