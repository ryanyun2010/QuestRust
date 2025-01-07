#[derive(Copy, Clone, Debug)]
pub struct Terrain{ // terrain is always 32x32 pixels
    pub element_id: usize,
    pub x: usize,
    pub y: usize
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TerrainTags {
    HeightLevel(usize),
    BlocksMovement,
    BlocksFlight,
    Swim(usize) //How fast you can swim in it.
}