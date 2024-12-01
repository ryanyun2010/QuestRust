enum Element{
    Terrain,
    Entity,
}

struct Chunk{
    x: i32,
    y: i32,
    elements: mut array: [[Element; 16]; 16]
}


struct World{
    player: Player,
    chunks: mut vec<Chunk>
}

impl World{
    pub fn getTerrain(&self, x: i32, y: i32) -> Terrain{

    }
}

struct Terrain{
    x: i32,
    y: i32,
    id: i32
}

struct Entity{
    x: i32,
    y: i32,
    id: i32,
}

struct Player {

}