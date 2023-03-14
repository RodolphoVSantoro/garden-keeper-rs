pub struct PhysicsProperties {
    pub points_from_hits: i32,
}

pub struct LightningProperties {
    pub LUZ_FUNDO: i32,
    pub crescente: bool,
}

impl Default for LightningProperties {
    fn default() -> LightningProperties {
        return LightningProperties {
            LUZ_FUNDO: 20,
            crescente: true,
        };
    }
}
