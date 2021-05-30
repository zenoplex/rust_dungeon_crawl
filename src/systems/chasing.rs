use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(ecs: &SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    if let Some((player_pos, _)) = player.iter(ecs).next() {
        let player_idx = map_idx(player_pos.x, player_pos.y);

        let search_targes = vec![player_idx];
        let dijkstar_map =
            DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targes, map, 1024.0);

        movers.iter(ecs).for_each(|(enemy_entity, pos, _)| {
            let idx = map_idx(pos.x, pos.y);
            if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstar_map, idx, map) {
                let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

                // 1.2 ensures distance is bigger than 1.0 and 1.4 which is approx of diagonal distance
                let destination = if distance > 1.2 {
                    map.index_to_point2d(destination)
                } else {
                    *player_pos
                };

                let mut attacked = false;

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
            };
        })
    }
}
