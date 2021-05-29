use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        // Query entity with Points and Player only
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        if delta != Point::zero() {
            let mut hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(enemy_entity, _)| {
                    hit_something = true;

                    println!("hit");

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *enemy_entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    // Legion does not support single component insertion
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        // Maybe should give turn only after player delta is arrow key
        *turn_state = TurnState::EnemyTurn;
    }
}
