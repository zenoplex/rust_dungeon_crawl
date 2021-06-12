use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[write_component(Item)]
#[write_component(Carried)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        // Query entity with Points and Player only
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                if let Some((player_entity, player_pos)) = players.iter(ecs).next() {
                    let mut items = <(Entity, &Item, &Point)>::query();
                    items
                        .iter(ecs)
                        .filter(|(_, _, item_pos)| *item_pos == player_pos)
                        .for_each(|(item_entity, _, _)| {
                            // Remove Point so Item will not be rendered
                            commands.remove_component::<Point>(*item_entity);
                            commands.add_component(*item_entity, Carried(*player_entity));
                        });
                }

                Point::zero()
            }
            _ => Point::zero(),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        let mut did_something = false;

        if delta != Point::zero() {
            let mut hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(enemy_entity, _)| {
                    hit_something = true;
                    did_something = true;

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
                did_something = true;
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

        if !did_something {
            if let Ok(mut health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = std::cmp::min(health.current + 1, health.max);
            }
        }

        // Maybe should give turn only after player delta is arrow key
        *turn_state = TurnState::PlayerTurn;
    }
}
