use concat_arrays::concat_arrays;
use engine_extension_types::{LightningProperties, PhysicsProperties};
use garden_keeper_types::{EnemyProperties, RelativeWindowSize, WindowProperties};

pub struct GameProperties {
    pub LIGHTNING_PROPERTIES: LightningProperties,
    pub RELATIVE_WINDOW_SIZE: RelativeWindowSize,
    pub WINDOW_PROPERTIES: WindowProperties,
    pub ENEMY_PROPERTIES: EnemyProperties,
    pub PHYSICS_PROPERTIES: PhysicsProperties,
    pub PLAYER_SPEED_COOLDOWN: f64,
}

pub fn get_window_properties(game_properties: GameProperties) -> WindowProperties {
    return game_properties.WINDOW_PROPERTIES;
}

pub fn set_window_dimensions(game_properties: &mut GameProperties, width: i32, height: i32) {
    game_properties.WINDOW_PROPERTIES.window_width = width;
    game_properties.WINDOW_PROPERTIES.window_height = height;
}

impl Default for GameProperties {
    fn default() -> GameProperties {
        return GameProperties {
            LIGHTNING_PROPERTIES: LightningProperties {
                LUZ_FUNDO: 20,
                crescente: true,
            },
            RELATIVE_WINDOW_SIZE: RelativeWindowSize {
                relative_height: 0.0,
                relative_width: 0.0,
            },
            WINDOW_PROPERTIES: WindowProperties {
                //botaoMenuCompra: OnceCell::new(),
                fundo_menu_compra: 0,
                id_fonte_HUD: 0,
                window_width: 0,
                window_height: 0,
            },
            ENEMY_PROPERTIES: EnemyProperties {
                ENEMY_HEALTH: concat_arrays!(
                    [0, 100, 150, 250, 350, 500, 650, 750, 850, 950, 1100],
                    [0; 199]
                ),
                MAX_HP_ANT: 0,
                ENEMY_QUANTITY_ROUND: [6],
            },
            PHYSICS_PROPERTIES: PhysicsProperties {
                points_from_hits: 10,
            },
            PLAYER_SPEED_COOLDOWN: 0.016,
        };
    }
}

impl GameProperties {
    pub fn init() -> GameProperties {
        return GameProperties::default();
    }
}
