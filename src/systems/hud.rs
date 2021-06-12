use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Name)]
#[read_component(Carried)]
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

    if let Some((player_entity, _)) = <(Entity, &Player)>::query().iter(ecs).next() {
        let mut y = 3;
        <(&Item, &Name, &Carried)>::query()
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == *player_entity)
            .enumerate()
            .for_each(|(i, (_, name, _))| {
                draw_batch.print(Point::new(3, y), format!("{}:{}", i + 1, name.0));
                y += i;
            });
    };

    draw_batch.submit(10000).expect("Batch error");
}
