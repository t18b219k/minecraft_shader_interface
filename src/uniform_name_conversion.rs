use rspirv_reflect::rspirv::spirv::CLOp::ldexp;
use std::collections::HashMap;

pub struct Rename {
    g_buffer_shader_table: HashMap<String, String>,
    shadow_shader_table: HashMap<String, String>,
    composite_and_deferred_shader_table: HashMap<String, String>,
}
impl Rename {
    pub fn init() -> Self {
        let mut g_buffer_shader_table = HashMap::new();
        g_buffer_shader_table.insert("shadow".to_string(), "shadowtex0".to_string());
        g_buffer_shader_table.insert("watershadow".to_string(), "shadowtex0".to_string());
        g_buffer_shader_table.insert("colortex4".to_string(), "gaux1".to_string());
        g_buffer_shader_table.insert("colortex5".to_string(), "gaux2".to_string());
        g_buffer_shader_table.insert("colortex6".to_string(), "gaux3".to_string());
        g_buffer_shader_table.insert("colortex7".to_string(), "gaux4".to_string());
        g_buffer_shader_table.insert("shadowcolor".to_string(), "shadowcolor0".to_string());

        let mut shadow_shader_table = HashMap::new();
        shadow_shader_table.insert("tex".to_string(), "texture".to_string());
        shadow_shader_table.insert("shadow".to_string(), "shadowtex0".to_string());
        shadow_shader_table.insert("watershadow".to_string(), "shadowtex0".to_string());
        shadow_shader_table.insert("colortex4".to_string(), "gaux1".to_string());
        shadow_shader_table.insert("colortex5".to_string(), "gaux2".to_string());
        shadow_shader_table.insert("colortex6".to_string(), "gaux3".to_string());
        shadow_shader_table.insert("colortex7".to_string(), "gaux4".to_string());
        shadow_shader_table.insert("shadowcolor".to_string(), "shadowcolor0".to_string());

        let mut composite_and_deferred_shader_table = HashMap::new();
        composite_and_deferred_shader_table.insert("gcolor".to_string(),"colortex0".to_string());
        composite_and_deferred_shader_table.insert("gdepth".to_string(),"colortex1".to_string());
        composite_and_deferred_shader_table.insert("gnormal".to_string(), "colortex2".to_string());
        composite_and_deferred_shader_table.insert("composite".to_string(), "colortex3".to_string());
        composite_and_deferred_shader_table.insert("shadow".to_string(), "shadowtex0".to_string());
        composite_and_deferred_shader_table.insert("watershadow".to_string(), "shadowtex0".to_string());
        composite_and_deferred_shader_table.insert("gdepthtex".to_string(), "depthtex0".to_string());
        composite_and_deferred_shader_table.insert("gaux1".to_string(), "colortex4".to_string());
        composite_and_deferred_shader_table.insert("gaux2".to_string(), "colortex5".to_string());
        composite_and_deferred_shader_table.insert("gaux3".to_string(), "colortex6".to_string());
        composite_and_deferred_shader_table.insert("gaux4".to_string(), "colortex7".to_string());
        composite_and_deferred_shader_table.insert("shadowcolor".to_string(), "shadowcolor0".to_string());
        Self {
            g_buffer_shader_table,
            shadow_shader_table,
            composite_and_deferred_shader_table,
        }
    }
}
