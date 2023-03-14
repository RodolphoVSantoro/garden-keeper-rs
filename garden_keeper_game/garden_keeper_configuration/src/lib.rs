//TODO: read constants from config file
pub const PLAYER_START_HP: u32 = 2;

pub const N_MAGIAS: u32 = 9;
pub const ICE_PELLET: u32 = 4;

pub const TEXT_FAGULHA: &str = "No cost(Infinite ammo)";
pub const IMG_FAGULHA: &str = "../imagens/menuBuy/fagulha.png";

pub enum MagicKind {
    NOT_MAGIC = -1,
    FAGULHA = 0,
    LASIER = 1,
    BRISA = 2,
    FIREBALL = 3,
    TESLA = 4,
    GLACIAL = 5,
    SUPERNOVA = 6,
    TROVAO = 7,
    ICE_STORM = 8,
    ICE_PELLET = 9,
}

pub const CUSTO_MAGIA: [u32; N_MAGIAS as usize] =
    [0, 2000, 3000, 7000, 7500, 8000, 15000, 12000, 18000];
pub const CUSTO_ESSENCIA: [u32; N_MAGIAS as usize] =
    [0, 500, 500, 1500, 1750, 2000, 5000, 4000, 6000];
pub const QTD_ESSENCIA: [u32; N_MAGIAS as usize] = [0, 100, 70, 50, 50, 50, 30, 80, 30];
//terceiro 800 e da ice pellet
pub const DANO_MAGIA: [u32; N_MAGIAS as usize] =
    [150, 800, 800, 6000, 800, 10000, 160000, 40000, 10000];
//12 da ice pellet
pub const SPEED_MAGIA: [u32; N_MAGIAS as usize] = [20, 100, 6, 30, 100, 40, 2, 300, 0];
//o segundo 0.06 e do tesla mesmo
pub const COOLDOWN_MAGIA: [f64; (N_MAGIAS + 1) as usize] =
    [0.15, 0.06, 0.9, 0.45, 0.3, 0.3, 3.0, 0.045, 12.0, 3.0];
//cooldown de ice pellets na icestorm
pub const COOLDOWN_PONTOS: f64 = 0.2;
pub const COOLDOWN_ICE: f64 = 0.1;
//define quanto vai durar
pub const HP_ICESTORM: u32 = 150;
pub const HP_FAGULHA: u32 = 60;
pub const HP_ICEPELLET: u32 = 30;

pub const LARGURA_MAGIA: [u32; N_MAGIAS as usize] = [14, 39, 56, 70, 27, 132, 141, 141, 141];

pub const ALTURA_MAGIA: [u32; N_MAGIAS as usize] = [14, 11, 56, 70, 27, 98, 141, 70, 101];

pub const COOLDOWN_HPMAGIA: [f64; N_MAGIAS as usize + 1] =
    [0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.01];

pub const IMG_MENU_COMPRA: [&'static str; N_MAGIAS as usize] = [
    "../imagens/menuBuy/fagulha.png",
    "../imagens/menuBuy/lasier.png",
    "../imagens/menuBuy/brisa.png",
    "../imagens/menuBuy/fireball.png",
    "../imagens/menuBuy/tesla.png",
    "../imagens/menuBuy/glacial.png",
    "../imagens/menuBuy/supernova.png",
    "../imagens/menuBuy/trovao.png",
    "../imagens/menuBuy/icestorm.png",
];

pub const IMG_MAGIA: [&'static str; N_MAGIAS as usize] = [
    "../imagens/magias/fagulha.png",
    "../imagens/magias/lasier.png",
    "../imagens/magias/brisa.png",
    "../imagens/magias/fireball.png",
    "../imagens/magias/icePellet.png",
    "../imagens/magias/glacial.png",
    "../imagens/magias/supernova.png",
    "../imagens/magias/trovao.png",
    "../imagens/magias/icestorm.png",
];

pub const NOME_MAGIA: [&'static str; N_MAGIAS as usize] = [
    "Fagulha",
    "Lasier",
    "Brisa",
    "Fireball",
    "Tesla",
    "Glacial",
    "Supernova",
    "Thundera",
    "Ice Storm",
];
