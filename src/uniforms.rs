/// this enum store the information for uniform name and
/// texture id
pub enum GBuffer {
    Texture,
    LightMap,
    Normals,
    Specular,
    Shadow(bool),
    WaterShadow,
    ShadowTex0,
    ShadowTex1,
    DepthTex0,
    DepthTex1,
    GAux(u8),
    ColorTex(u8),
    ShadowColor,
    ShadowColor0,
    ShadowColor1,
    NoiseTex,
}
pub trait TextureId {
    fn get_id(&self) -> Option<usize>;
}
impl TextureId for GBuffer {
    fn get_id(&self) -> Option<usize> {
        match self {
            GBuffer::Texture => Some(0),
            GBuffer::LightMap => Some(1),
            GBuffer::Normals => Some(2),
            GBuffer::Specular => Some(3),
            GBuffer::Shadow(water_shadow_enabled) => {
                if *water_shadow_enabled {
                    Some(5)
                } else {
                    Some(4)
                }
            }
            GBuffer::WaterShadow => Some(4),
            GBuffer::ShadowTex0 => Some(4),
            GBuffer::ShadowTex1 => Some(5),
            GBuffer::DepthTex0 => Some(6),
            GBuffer::DepthTex1 => Some(11),
            GBuffer::GAux(n) => match n {
                1 => Some(7),
                2 => Some(8),
                3 => Some(9),
                4 => Some(10),
                _ => None,
            },
            GBuffer::ColorTex(n) => match n {
                4 => Some(7),
                5 => Some(8),
                6 => Some(9),
                7 => Some(10),
                8 => Some(16),
                9 => Some(17),
                10 => Some(18),
                11 => Some(19),
                12 => Some(20),
                13 => Some(21),
                14 => Some(22),
                15 => Some(23),
                _ => None,
            },
            GBuffer::ShadowColor => Some(13),
            GBuffer::ShadowColor0 => Some(13),
            GBuffer::ShadowColor1 => Some(14),
            GBuffer::NoiseTex => Some(15),
        }
    }
}
pub enum Shadow {
    Tex,
    Texture,
    Lightmap,
    Normals,
    Specular,
    Shadow(bool),
    WaterShadow,
    ShadowTex0,
    ShadowTex1,
    GAux(u8),
    ColorTex(u8),
    ShadowColor,
    ShadowColor0,
    ShadowColor1,
    NoiseTex,
}

impl TextureId for Shadow {
    fn get_id(&self) -> Option<usize> {
        match self {
            Shadow::Texture => Some(0),
            Shadow::Lightmap => Some(1),
            Shadow::Normals => Some(2),
            Shadow::Specular => Some(3),
            Shadow::Shadow(water_shadow_enabled) => {
                if *water_shadow_enabled {
                    Some(5)
                } else {
                    Some(4)
                }
            }
            Shadow::WaterShadow => Some(4),
            Shadow::ShadowTex0 => Some(4),
            Shadow::ShadowTex1 => Some(5),

            Shadow::GAux(n) => match n {
                1 => Some(7),
                2 => Some(8),
                3 => Some(9),
                4 => Some(10),
                _ => None,
            },
            Shadow::ColorTex(n) => match n {
                4 => Some(7),
                5 => Some(8),
                6 => Some(9),
                7 => Some(10),
                8 => Some(16),
                9 => Some(17),
                10 => Some(18),
                11 => Some(19),
                12 => Some(20),
                13 => Some(21),
                14 => Some(22),
                15 => Some(23),
                _ => None,
            },
            Shadow::ShadowColor => Some(13),
            Shadow::ShadowColor0 => Some(13),
            Shadow::ShadowColor1 => Some(14),
            Shadow::NoiseTex => Some(15),
            Shadow::Tex => Some(0),
        }
    }
}
