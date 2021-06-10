use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Box<dyn MapTheme>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            // Check if point is renderable beforehand otherwise can cause index error
            if map.is_in_bounds(pt) {
                let idx = map_idx(pt.x, pt.y);
                let is_revlead = map.revealed_tiles[idx];
                let is_visible = player_fov.visible_tiles.contains(&pt);

                if is_visible | is_revlead {
                    let tint = if is_visible { WHITE } else { DARK_GRAY };
                    let glpth = theme.tile_to_render(map.tiles[idx]);

                    draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), glpth);
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
