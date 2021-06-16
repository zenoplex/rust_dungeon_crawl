use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
        FieldOfView::new(8),
    ));
}

pub fn spawn_entity(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_dungeon_map(ecs, pos),
        _ => spawn_enemy(ecs, pos, rng),
    }
}

fn spawn_enemy(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator) {
    let (hp, name, glyph) = match rng.range(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health {
            current: hp,
            max: hp,
        },
        ChasingPlayer,
        Name(name),
        FieldOfView::new(6),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Name("Amulet of Yala".to_string()),
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
    ));
}

fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        Name("Healing Potion".to_string()),
        ProvidesHealing { amount: 6 },
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        pos,
    ));
}

fn spawn_dungeon_map(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        Name("Dungeon Map".to_string()),
        ProvidesDungeonMap,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        pos,
    ));
}
