use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: vec![],
            monster_spawns: vec![],
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_enemies(&start, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|tile| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *tile = TileType::Floor
            } else {
                *tile = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        // checking 3 x 3 grid
        for iy in -1..=1 {
            for ix in -1..=1 {
                // Dont check center
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();

        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);

                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == TileType::Floor)
            // Derive distance between points
            .map(|(i, _)| {
                (
                    i,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(i)),
                )
            })
            // distance2d doesn't return f32::(NEG_)INFINITY nor f32::NAN so its safe to unwrap
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            // Return index item
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}
