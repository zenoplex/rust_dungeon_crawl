use crate::prelude::*;

#[system]
#[read_component(Name)]
#[read_component(Point)]
#[read_component(Health)]
pub fn tooltip(ecs: &mut SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name, &Health)>::query();
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    positions
        .iter(ecs)
        .filter(|(_, pos, _, _)| **pos == map_pos)
        .for_each(|(entity, _, name, health)| {
            // align screen position because layer 2 is bigger
            let screen_pos = *mouse_pos * 4;
            println!("{:?}", health);

            // TODO: try entry_try
            let display = format!("{} : {} hp", &name.0, health.current);

            draw_batch.print(screen_pos, display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
