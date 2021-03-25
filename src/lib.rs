//! export minecraft shader to any app
//!
//! *  we must replace # version xxx to #version 450
//! *  add some compatibility function like ftransform
//! *  insert #extension GL_GOOGLE_include_directive:enable to enable include
//! *  rename eliminated gl gl_builtin variable  to _ prefix like _gl_ModelViewProjection
//! *  remove all uniform and variable declarations
//! * insert old gl emulation Uniforms and variables such as gl_ModelViewProjection
//! *  uniform and buffer definitions
//! * MRT and gl_FragColor to e.g out vec4 _gl_FragColor
//! * rename all optifine defined uniform and attributes from camelCase to snake_case
use crate::optifine_standard_macros::{
    McGLRenderer, McGLSLVersion, McGLVendor, McGLVersion, McOptions, McOs, McStandardMacroDefines,
};
use shaderc::ShaderKind;

pub mod attribute_varying_transformer;
pub mod directive_rewrite;
pub mod function_rename;
mod optifine_standard_macros;
pub mod render_stage;
pub mod shader_parameter;
pub mod uniform_name_conversion;
pub mod uniforms;

mod embedded_rename;

pub fn transpile<Source: AsRef<str>>(
    source: Source,
    kind: ShaderKind,
    std_macro_defines: McStandardMacroDefines,
) -> String {
    let lines = source.as_ref().lines();
    let mut line_stack: Vec<&str> = lines.collect();

    let std_macro = std_macro_defines.to_string();
    line_stack.insert(2, &std_macro);
    if ShaderKind::Vertex == kind {
        let vertex_builtin_attribute_definition =
            include_str!("../gl_builtin/vertex_shader_builtin_attributes.glsl");
        for line in vertex_builtin_attribute_definition.lines().rev() {
            line_stack.insert(3, line);
        }
        let compatibility = include_str!("../gl_builtin/vertex_old_gl_compat.glsl");
        line_stack.insert(17, compatibility);
    }

    let mut source = String::new();

    for line in line_stack {
        source.push_str(line);
        source.push('\n')
    }

    #[cfg(debug_assertions)]
    println!(
        "standard macro and compatibility support code inserted source:\n{}",
        source
    );

    match kind {
        ShaderKind::Vertex => String::new(),
        ShaderKind::Fragment => String::new(),
        ShaderKind::Compute => String::new(),
        ShaderKind::Geometry => String::new(),
        _ => {
            panic!("not supported shader type ")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::optifine_standard_macros::{
        McGLRenderer, McGLSLVersion, McGLVendor, McGLVersion, McOptions, McOs,
        McStandardMacroDefines,
    };
    use crate::transpile;
    use shaderc::ShaderKind;

    #[test]
    fn it_works() {
        let vulkan_environment_std = McStandardMacroDefines {
            version: Default::default(),
            gl_version: McGLVersion::GL450,
            shader_version: McGLSLVersion::GLSL450,
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
        let source = include_str!("../test_shader/final.vsh");
        let source = transpile(source, ShaderKind::Vertex, vulkan_environment_std);
    }
}
