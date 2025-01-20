#[derive(Clone, Debug)]
pub struct UIElement{
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub sprite_id: usize,
    pub visible: bool,
}

pub struct UIElementDescriptor{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub sprite_id: usize,
    pub visible: bool,
}

impl UIElement{
    pub fn new(name: String, descriptor: UIElementDescriptor) -> Self{
        Self{
            name,
            z: descriptor.z,
            x: descriptor.x,
            y: descriptor.y,
            width: descriptor.width,
            height: descriptor.height,
            sprite_id: descriptor.sprite_id,
            visible: descriptor.visible,
        }
    }
}