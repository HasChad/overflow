use macroquad::texture::{Texture2D, load_texture};

pub struct Resources {
    pub arrow: Texture2D,
    pub block: Texture2D,
    pub wall: Texture2D,
}

impl Resources {
    pub async fn load_textures() -> Self {
        Self {
            arrow: load_texture("arrow.png").await.unwrap(),
            block: load_texture("block.png").await.unwrap(),
            wall: load_texture("wall.png").await.unwrap(),
        }
    }
}
