//! export minecraft shader to any app
//!
//! * first we must replace # version xxx to #version 450
//! * second add some compatibility function like ftransform
//! * third insert #extension GL_GOOGLE_include_directive:enable to enable include
//! * third rename eliminated gl builtin variable  to _ suffix like _gl_ModelViewProjection
//! * remove all uniform and variable declarations
//! * insert old gl emulation Uniforms and variables such as gl_ModelViewProjection
//! * attach uniform and buffer definitions
//! * MRT and gl_FragColor to e.g out vec4 _gl_FragColor
//! * rename all optifine defined uniform and attributes from camelCase to snake_case
pub mod attribute_varying_transformer;
pub mod directive_rewrite;
pub mod function_rename;
pub mod render_stage;
pub mod shader_parameter;
pub mod uniform_name_conversion;
pub mod uniforms;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
