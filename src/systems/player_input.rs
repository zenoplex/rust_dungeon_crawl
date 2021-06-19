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
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            _ => Point::zero(),
        };

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
        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).next() {
        let item_entity = <(Entity, &Item, &Carried)>::query()
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == *player_entity)
            .enumerate()
            .filter(|(i, (_, _, _))| *i == n)
            // Get first matching entity
            .find_map(|(_, (item_entity, _, _))| Some(item_entity));

        if let Some(item) = item_entity {
            println!("ActivateItem");
            commands.push((
                (),
                ActivateItem {
                    used_by: *player_entity,
                    item: *item,
                },
            ));
        }
    }

    Point::zero()
}
