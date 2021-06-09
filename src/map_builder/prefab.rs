use super::MapArchitect;
use crate::prelude::*;

const FORTRESS: &str = "
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
";

struct PrefabArchitect {}

impl MapArchitect for PrefabArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb
    }
}

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;
    let dijkstar_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let rows: Vec<&str> = FORTRESS.trim().split('\n').map(|c| c.trim()).collect();
    let row_count = rows.len() as i32;
    let col_count = rows[0].chars().count() as i32;

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - row_count),
            rng.range(0, SCREEN_HEIGHT - col_count),
            row_count,
            col_count,
        );

        let mut can_place = false;
        dimensions.for_each(|point| {
            let idx = mb.map.point2d_to_index(point);
            let distance = dijkstar_map.map[idx];
            // Avoid to being overwritten
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != point {
                can_place = true;
            }
        });

        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            // Remove monster if it exists within points
            mb.monster_spawns.retain(|point| !points.contains(point));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .chars()
            .filter(|s| *s != '\r' && *s != '\n')
            .collect();

        let mut i = 0;
        for ty in placement.y..placement.y + row_count {
            for tx in placement.x..placement.x + col_count {
                let idx = map_idx(tx, ty);

                match string_vec[i] {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => (),
                }

                i += 1;
            }
        }
    }
}
