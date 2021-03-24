/// #define MC_VERSION  directive
#[derive(Copy, Clone, Debug)]
pub struct McVersion {
    major: u8,
    minor: u8,
    release: u8,
}

impl Default for McVersion {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 16,
            release: 2,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub enum McGLVersion {
    GL200,
    GL210,
    GL300,
    GL310,
    GL320,
    GL330,
    GL400,
    GL410,
    GL420,
    GL430,
    GL440,
    GL450,
    GL460,
}
#[derive(Copy, Clone, Debug)]
pub enum McGLSLVersion {
    GLSL110,
    GLSL120,
    GLSL130,
    GLSL140,
    GLSL150,
    GLSL330,
    GLSL400,
    GLSL410,
    GLSL420,
    GLSL430,
    GLSL440,
    GLSL450,
    GLSL460,
}
#[derive(Copy, Clone, Debug)]
pub enum McOs {
    Windows,
    Mac,
    Linux,
    Other,
}
#[derive(Copy, Clone, Debug)]
pub enum McGLVendor {
    AMD,
    ATI,
    Intel,
    Nvidia,
    Xorg,
    Other,
}
#[derive(Copy, Clone, Debug)]
pub enum McGLRenderer {
    Radeon,
    GeForce,
    Quadro,
    Intel,
    Gallium,
    Mesa,
    Other,
}
#[derive(Copy, Clone, Debug)]
pub struct McOptions {
    fxaa_level: u8,
    normal_map: bool,
    specular_map: bool,
    render_quality: f32,
    shadow_quality: f32,
    hand_depth: f32,
    old_hand_light: bool,
    old_lighting: bool,
    anisotropic_filtering: Option<f32>,
}
#[derive(Copy, Clone, Debug)]
pub enum McTexture {
    LabPBR,
    LabPBR1_3,
}

pub struct McStandardMacroDefines {
    version: McVersion,
    gl_version: McGLVersion,
    shader_version: McGLSLVersion,
    operating_system: McOs,
    vendor: McGLVendor,
    renderer: McGLRenderer,
    options: McOptions,
    textures: Option<McTexture>,
}

impl ToString for McVersion {
    fn to_string(&self) -> String {
        format!(
            "#define MC_VERSION {}{:02}{:02}\n",
            self.major, self.minor, self.release
        )
    }
}

impl ToString for McGLVersion {
    fn to_string(&self) -> String {
        format!(
            "#define MC_GL_VERSION {}\n",
            match self {
                McGLVersion::GL200 => {
                    200
                }
                McGLVersion::GL210 => {
                    210
                }
                McGLVersion::GL300 => {
                    300
                }
                McGLVersion::GL310 => {
                    310
                }
                McGLVersion::GL320 => {
                    320
                }
                McGLVersion::GL330 => {
                    330
                }
                McGLVersion::GL400 => {
                    400
                }
                McGLVersion::GL410 => {
                    410
                }
                McGLVersion::GL420 => {
                    420
                }
                McGLVersion::GL430 => {
                    430
                }
                McGLVersion::GL440 => {
                    440
                }
                McGLVersion::GL450 => {
                    450
                }
                McGLVersion::GL460 => {
                    460
                }
            }
        )
    }
}

impl ToString for McGLSLVersion {
    fn to_string(&self) -> String {
        format!(
            "#define MC_GLSL_VERSION {}\n",
            match self {
                McGLSLVersion::GLSL110 => {
                    110
                }
                McGLSLVersion::GLSL120 => {
                    120
                }
                McGLSLVersion::GLSL130 => {
                    130
                }
                McGLSLVersion::GLSL140 => {
                    140
                }
                McGLSLVersion::GLSL150 => {
                    150
                }
                McGLSLVersion::GLSL330 => {
                    330
                }
                McGLSLVersion::GLSL400 => {
                    400
                }
                McGLSLVersion::GLSL410 => {
                    410
                }
                McGLSLVersion::GLSL420 => {
                    420
                }
                McGLSLVersion::GLSL430 => {
                    430
                }
                McGLSLVersion::GLSL440 => {
                    440
                }
                McGLSLVersion::GLSL450 => {
                    450
                }
                McGLSLVersion::GLSL460 => {
                    460
                }
            }
        )
    }
}

impl ToString for McOs {
    fn to_string(&self) -> String {
        match self {
            McOs::Windows => "#define MC_OS_WINDOWS\n".to_string(),
            McOs::Mac => "#define MC_OS_MAC\n".to_string(),
            McOs::Linux => "#define MC_OS_LINUX\n".to_string(),
            McOs::Other => "#define MC_OS_OTHER\n".to_string(),
        }
    }
}

impl ToString for McGLVendor {
    fn to_string(&self) -> String {
        format!(
            "#define MC_GL_VENDOR_{}\n",
            match self {
                McGLVendor::AMD => {
                    "AMD"
                }
                McGLVendor::ATI => {
                    "ATI"
                }
                McGLVendor::Intel => {
                    "INTEL"
                }
                McGLVendor::Nvidia => {
                    "NVIDIA"
                }
                McGLVendor::Xorg => {
                    "XORG"
                }
                McGLVendor::Other => {
                    "OTHER"
                }
            }
        )
    }
}

impl ToString for McGLRenderer {
    fn to_string(&self) -> String {
        format!(
            "#define MC_GL_RENDERER_{}\n",
            match self {
                McGLRenderer::Radeon => {
                    "RADEON"
                }
                McGLRenderer::GeForce => {
                    "GEFORCE"
                }
                McGLRenderer::Quadro => {
                    "QUADRO"
                }
                McGLRenderer::Intel => {
                    "INTEL"
                }
                McGLRenderer::Gallium => {
                    "GALLIUM"
                }
                McGLRenderer::Mesa => {
                    "MESA"
                }
                McGLRenderer::Other => {
                    "OTHER"
                }
            }
        )
    }
}
impl ToString for McOptions {
    fn to_string(&self) -> String {
        format!(
            "#define MC_FXAA_LEVEL {}\n{}{}#define MC_RENDER_QUALITY {:.8}\n#define MC_SHADOW_QUALITY {:.8}\n#define MC_HAND_DEPTH {:.4}\n{}{}{}",
            self.fxaa_level,
            if self.normal_map {
                "#define MC_NORMAL_MAP \n"
            } else {
                ""
            },
            if self.specular_map {
                "#define MC_SPECULAR_MAP \n"
            } else {
                ""
            },
            self.render_quality,
            self.shadow_quality,
            self.hand_depth,
            if self.old_hand_light{
                "#define MC_OLD_HAND_LIGHT \n"
            }else{
                ""
            },
            if self.old_lighting{
                "#define MC_OLD_LIGHTING \n"
            }else{
                ""
            },
            if let Some(v)=self.anisotropic_filtering{
                format!("#define MC_ANISOTROPIC_FILTERING {:.4}\n",v)
            }else{
                "".to_string()
            }
        )
    }
}
impl ToString for McTexture {
    fn to_string(&self) -> String {
        match self {
            McTexture::LabPBR => "#define MC_TEXTURE_FORMAT_LAB_PBR\n",
            McTexture::LabPBR1_3 => "#define MC_TEXTURE_FORMAT_LAB_PBR_1_3\n",
        }
        .to_string()
    }
}
const RENDER_STAGE: &'static str = include_str!("McRenderStageDefine.h");
impl ToString for McStandardMacroDefines {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}{}",
            self.version.to_string(),
            self.gl_version.to_string(),
            self.shader_version.to_string(),
            self.operating_system.to_string(),
            self.vendor.to_string(),
            self.renderer.to_string(),
            self.options.to_string(),
            match self.textures {
                None => {
                    "".to_string()
                }
                Some(texture) => {
                    texture.to_string()
                }
            },
            RENDER_STAGE
        )
    }
}
#[test]
fn test() {
    let std_macro_define = McStandardMacroDefines {
        version: Default::default(),
        gl_version: McGLVersion::GL200,
        shader_version: McGLSLVersion::GLSL110,
        operating_system: McOs::Windows,
        vendor: McGLVendor::AMD,
        renderer: McGLRenderer::Radeon,
        options: McOptions {
            fxaa_level: 2,
            normal_map: false,
            specular_map: false,
            render_quality: 1.0,
            shadow_quality: 1.0,
            hand_depth: 0.25,
            old_hand_light: false,
            old_lighting: false,
            anisotropic_filtering: None,
        },
        textures: None,
    };
    println!("{}", std_macro_define.to_string());
}
