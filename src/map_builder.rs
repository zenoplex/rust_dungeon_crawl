mod empty;

use crate::prelude::*;
use empty::EmptyArchitect;

const UNREACHABLE: &f32 = &f32::MAX;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect = EmptyArchitect {};
        architect.new(rng)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        // Derive map index via enumerate
        let farthest_idx = dijkstra_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, dist)| *dist < UNREACHABLE)
            // Compare dijkstra value and get max
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        self.map.index_to_point2d(farthest_idx)
    }
}

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}
