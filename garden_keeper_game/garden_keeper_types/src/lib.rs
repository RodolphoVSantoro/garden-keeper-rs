pub const STANDARD_HEIGHT: i32 = 1080;
pub const STANDARD_WIDTH: i32 = 1920;

pub struct Size2D {
    pub height: i32,
    pub width: i32,
}

pub struct Position2D {
    pub x: i32,
    pub y: i32,
}

pub struct RelativeWindowSize {
    pub relative_height: f64,
    pub relative_width: f64,
}

pub struct WindowProperties {
    //botaoMenuCompra: [Button; N_MAGIAS as usize],
    pub fundo_menu_compra: i32,
    pub id_fonte_HUD: i32,
    pub window_width: i32,
    pub window_height: i32,
}

pub struct EnemyProperties {
    pub ENEMY_HEALTH: [u32; 210],
    pub MAX_HP_ANT: u32,
    pub ENEMY_QUANTITY_ROUND: [u32; 1],
}
