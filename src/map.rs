use bracket_lib::prelude::to_cp437;

use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }

    pub fn is_in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < SCREEN_WIDTH as i32
            && point.y >= 0
            && point.y < SCREEN_HEIGHT as i32
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        println!("point: {:?}", &point);
        self.is_in_bounds(point)
            && self.tiles[map_idx(point.x as u32, point.y as u32)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x as u32, point.y as u32))
        }
    }
}

pub fn map_idx(x: u32, y: u32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
