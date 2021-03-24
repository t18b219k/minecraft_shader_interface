/// this enum store the information for uniform name and
/// texture id
pub trait TextureId {
    fn get_id(&self) -> Option<usize>;
}

pub enum GBuffersTextures {
    Texture,
    Lightmap,
    Normals,
    Specular,
    ShadowTex0,
    ShadowTex1,
    DepthTex0,
    GAux1,
    GAux2,
    GAux3,
    GAux4,
    DepthTex1,
    ShadowColor0,
    ShadowColor1,
    NoiseTex,
    ColorTex8,
    ColorTex9,
    ColorTex10,
    ColorTex11,
    ColorTex12,
    ColorTex13,
    ColorTex14,
    ColorTex15,
}

impl TextureId for GBuffersTextures {
    fn get_id(&self) -> Option<usize> {
        match self {
            GBuffersTextures::Texture => Some(0),
            GBuffersTextures::Lightmap => Some(1),
            GBuffersTextures::Normals => Some(2),
            GBuffersTextures::Specular => Some(3),
            GBuffersTextures::ShadowTex0 => Some(4),
            GBuffersTextures::ShadowTex1 => Some(5),
            GBuffersTextures::DepthTex0 => Some(6),
            GBuffersTextures::GAux1 => Some(7),
            GBuffersTextures::GAux2 => Some(8),
            GBuffersTextures::GAux3 => Some(9),
            GBuffersTextures::GAux4 => Some(10),
            GBuffersTextures::DepthTex1 => Some(12),
            GBuffersTextures::ShadowColor0 => Some(13),
            GBuffersTextures::ShadowColor1 => Some(14),
            GBuffersTextures::NoiseTex => Some(15),
            GBuffersTextures::ColorTex8 => Some(16),
            GBuffersTextures::ColorTex9 => Some(17),
            GBuffersTextures::ColorTex10 => Some(18),
            GBuffersTextures::ColorTex11 => Some(19),
            GBuffersTextures::ColorTex12 => Some(20),
            GBuffersTextures::ColorTex13 => Some(21),
            GBuffersTextures::ColorTex14 => Some(22),
            GBuffersTextures::ColorTex15 => Some(23),
        }
    }
}

pub enum ShadowTextures {
    Texture,
    LightMap,
    Normals,
    Specular,
    ShadowTex0,
    ShadowTex1,
    GAux1,
    GAux2,
    GAux3,
    GAux4,
    ShadowColor0,
    ShadowColor1,
    NoiseTex,
    ColorTex8,
    ColorTex9,
    ColorTex10,
    ColorTex11,
    ColorTex12,
    ColorTex13,
    ColorTex14,
    ColorTex15,
}

impl TextureId for ShadowTextures {
    fn get_id(&self) -> Option<usize> {
        match self {
            ShadowTextures::Texture => Some(0),
            ShadowTextures::LightMap => Some(1),
            ShadowTextures::Normals => Some(2),
            ShadowTextures::Specular => Some(3),
            ShadowTextures::ShadowTex0 => Some(4),
            ShadowTextures::ShadowTex1 => Some(5),
            ShadowTextures::GAux1 => Some(7),
            ShadowTextures::GAux2 => Some(8),
            ShadowTextures::GAux3 => Some(9),
            ShadowTextures::GAux4 => Some(10),
            ShadowTextures::ShadowColor0 => Some(13),
            ShadowTextures::ShadowColor1 => Some(14),
            ShadowTextures::NoiseTex => Some(15),
            ShadowTextures::ColorTex8 => Some(16),
            ShadowTextures::ColorTex9 => Some(17),
            ShadowTextures::ColorTex10 => Some(18),
            ShadowTextures::ColorTex11 => Some(19),
            ShadowTextures::ColorTex12 => Some(20),
            ShadowTextures::ColorTex13 => Some(21),
            ShadowTextures::ColorTex14 => Some(22),
            ShadowTextures::ColorTex15 => Some(23),
        }
    }
}

pub enum CompositeAndDeferredTextures {
    ColorTex0,
    ColorTex1,
    ColorTex2,
    ColorTex3,
    ShadowTex0,
    ShadowTex1,
    DepthTex0,
    ColorTex4,
    ColorTex5,
    ColorTex6,
    ColorTex7,
    DepthTex1,
    DepthTex2,
    ShadowColor0,
    ShadowColor1,
    NoiseTex,
    ColorTex8,
    ColorTex9,
    ColorTex10,
    ColorTex11,
    ColorTex12,
    ColorTex13,
    ColorTex14,
    ColorTex15,
}

impl TextureId for CompositeAndDeferredTextures {
    fn get_id(&self) -> Option<usize> {
        Some(match self {
            CompositeAndDeferredTextures::ColorTex0 => 0,
            CompositeAndDeferredTextures::ColorTex1 => 1,
            CompositeAndDeferredTextures::ColorTex2 => 2,
            CompositeAndDeferredTextures::ColorTex3 => 3,
            CompositeAndDeferredTextures::ShadowTex0 => 4,
            CompositeAndDeferredTextures::ShadowTex1 => 5,
            CompositeAndDeferredTextures::DepthTex0 => 6,
            CompositeAndDeferredTextures::ColorTex4 => 7,
            CompositeAndDeferredTextures::ColorTex5 => 8,
            CompositeAndDeferredTextures::ColorTex6 => 9,
            CompositeAndDeferredTextures::ColorTex7 => 10,
            CompositeAndDeferredTextures::DepthTex1 => 11,
            CompositeAndDeferredTextures::DepthTex2 => 12,
            CompositeAndDeferredTextures::ShadowColor0 => 13,
            CompositeAndDeferredTextures::ShadowColor1 => 14,
            CompositeAndDeferredTextures::NoiseTex => 15,
            CompositeAndDeferredTextures::ColorTex8 => 16,
            CompositeAndDeferredTextures::ColorTex9 => 17,
            CompositeAndDeferredTextures::ColorTex10 => 18,
            CompositeAndDeferredTextures::ColorTex11 => 19,
            CompositeAndDeferredTextures::ColorTex12 => 20,
            CompositeAndDeferredTextures::ColorTex13 => 21,
            CompositeAndDeferredTextures::ColorTex14 => 22,
            CompositeAndDeferredTextures::ColorTex15 => 23,
        })
    }
}
