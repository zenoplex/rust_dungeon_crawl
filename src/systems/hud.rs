use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut healths = <&Health>::query().filter(component::<Player>());
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Using next because player should be single
    if let Some(health) = healths.iter(ecs).next() {
        draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");

        draw_batch.bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            health.current,
            health.max,
            ColorPair::new(RED, BLACK),
        );

        draw_batch.print_color_centered(
            0,
            format!("Health {} / {}", health.current, health.max),
            ColorPair::new(WHITE, RED),
        );
    }
    draw_batch.submit(10000).expect("Batch error");
}
