use crate::prelude::*;

#[system]
#[read_component(Name)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn tooltip(ecs: &mut SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let mut positions = <(Entity, &Point, &Name)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| player_fov.visible_tiles.contains(pos) && **pos == map_pos)
        .for_each(|(entity, _, name)| {
            // align screen position because layer 2 is bigger
            let screen_pos = *mouse_pos * 4;

            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} : {} hp", &name.0, health.current)
                } else {
                    name.0.clone()
                };

            draw_batch.print(screen_pos, display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
