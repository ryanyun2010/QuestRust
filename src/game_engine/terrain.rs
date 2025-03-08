#[derive(Copy, Clone, Debug)]
pub struct Terrain{ // terrain is always 32x32 pixels
    pub element_id: usize,
    pub x: usize,
    pub y: usize
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TerrainTags {
    BlocksMovement,
    Sprite(usize)
}
