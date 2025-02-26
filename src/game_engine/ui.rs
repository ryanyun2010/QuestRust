#[derive(Clone, Debug)]
pub struct UIElement{
    pub name: String,
    pub sprite: UIESprite,
    pub visible: bool,
}


#[derive(Clone, Debug)]
pub struct UIESprite {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub sprite: String,
}

pub struct UIElementDescriptor{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub sprite: String,
    pub visible: bool,
}

impl UIElement{
    pub fn new(name: String, descriptor: UIElementDescriptor) -> Self{
        Self{
            name,
            sprite: UIESprite{
                x: descriptor.x,
                y: descriptor.y,
                z: descriptor.z,
                width: descriptor.width,
                height: descriptor.height,
                sprite: descriptor.sprite,
            },
            visible: descriptor.visible,
        }
    }
}
