use rltk::Rect;
pub const MAP_BORDER: Rect = Rect {
    x1: -100,
    y1: -100,
    x2: 100,
    y2: 100,
};

//TODO make Point to_tuple as const
const MAP_DIMENSION: (usize, usize) = (
    (MAP_BORDER.x2 - MAP_BORDER.x1) as usize,
    (MAP_BORDER.y2 - MAP_BORDER.y1) as usize,
);

pub type MapField = [[MapTile; MAP_DIMENSION.1 as usize]; MAP_DIMENSION.0 as usize];

#[derive(Clone, Copy)]
pub enum MapTile {
    Ground,
    Water,
    Grass,
}

pub fn random_map() -> MapField {
    let mut map = [[MapTile::Ground; MAP_DIMENSION.1 as usize]; MAP_DIMENSION.0 as usize];
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..(MAP_DIMENSION.0 as f32 * MAP_DIMENSION.1 as f32 * 0.75) as usize {
        let x = rng.range(0, MAP_DIMENSION.0);
        let y = rng.range(0, MAP_DIMENSION.1);
        map[x][y] = MapTile::Grass;
    }
    for _i in 0..(MAP_DIMENSION.0 as f32 * MAP_DIMENSION.1 as f32 * 0.1) as usize {
        let x = rng.range(0, MAP_DIMENSION.0);
        let y = rng.range(0, MAP_DIMENSION.1);
        map[x][y] = MapTile::Water;
    }
    map
}

pub fn _canal_map() -> MapField {
    let mut map = [[MapTile::Ground; MAP_DIMENSION.1 as usize]; MAP_DIMENSION.0 as usize];
    MAP_BORDER.for_each(|p| {
        if p.x % 10 == 0 || p.y % 10 == 0 {
            map[(p.x - MAP_BORDER.x1) as usize][(p.y - MAP_BORDER.y1) as usize] = MapTile::Water;
        }
    });
    map
}
