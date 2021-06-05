use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movements(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // Replace entities position with a new position(Point)
        commands.add_component(want_move.entity, want_move.destination);

        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            // if entity has FieldOfView component and wants to move, then replace it with fov.is_dirty: true
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                //  if entity is a player then move the camera
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);

                    // Update revealed_tiles to true for player fov
                    fov.visible_tiles.iter().for_each(|pos| {
                        let idx = map_idx(pos.x, pos.y);
                        map.revealed_tiles[idx] = true;
                    });
                }
            }
        }
    }
    commands.remove(*entity);
}
