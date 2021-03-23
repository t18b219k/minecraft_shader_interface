pub struct ShaderParametersBool {
    pub is_alive: bool,
    pub is_burning: bool,
    pub is_child: bool,
    pub is_glowing: bool,
    pub is_hurt: bool,
    pub is_in_lava: bool,
    pub is_in_water: bool,
    pub is_invisible: bool,
    pub is_on_ground: bool,
    pub is_ridden: bool,
    pub is_riding: bool,
    pub is_sneaking: bool,
    pub is_sprinting: bool,
    pub is_wet: bool,
}

type Float1 = [f32; 1];
type Float2 = [f32; 2];
type Float3 = [f32; 3];
type Float4 = [f32; 4];
type Int1 = [i32; 1];
type Int2 = [i32; 2];
type Int3 = [i32; 3];
type Int4 = [i32; 4];
type Matrix4 = [[f32; 4]; 4];

pub struct ShaderParametersFloat1 {
    pub uniform_fog_density: Float1,
    pub uniform_frame_time: Float1,
    pub uniform_frame_time_counter: Float1,
    pub uniform_sun_angle: Float1,
    pub uniform_shadow_angle: Float1,
    pub uniform_rain_strength: Float1,
    pub uniform_aspect_ratio: Float1,
    pub uniform_view_width: Float1,
    pub uniform_view_height: Float1,
    pub uniform_near: Float1,
    pub uniform_far: Float1,
    pub uniform_wetness: Float1,
    pub uniform_eye_altitude: Float1,
    pub uniform_night_vision: Float1,
    pub uniform_blindness: Float1,
    pub uniform_screen_brightness: Float1,
    pub uniform_center_depth_smooth: Float1,
    pub uniform_player_mood: Float1,
}

pub struct ShaderParametersFloat3 {
    pub uniform_fog_color: Float3,
    pub uniform_sky_color: Float3,
    pub uniform_sun_position: Float3,
    pub uniform_moon_position: Float3,
    pub uniform_shadow_light_position: Float3,
    pub uniform_up_position: Float3,
    pub uniform_previous_camera_position: Float3,
    pub uniform_camera_position: Float3,
}

pub struct ShaderParametersFloat4 {
    pub uniform_entity_color: Float4,
    pub uniform_sprite_bounds: Float4,
}

pub struct ShaderParametersFloat2;

pub struct ShaderParametersInt1 {
    pub uniform_entity_id: Int1,
    pub uniform_block_entity_id: Int1,
    pub uniform_texture: Int1,
    pub uniform_lightmap: Int1,
    pub uniform_normals: Int1,
    pub uniform_specular: Int1,
    pub uniform_shadow: Int1,
    pub uniform_watershadow: Int1,
    pub uniform_shadowtex0: Int1,
    pub uniform_shadowtex1: Int1,
    pub uniform_depthtex0: Int1,
    pub uniform_depthtex1: Int1,
    pub uniform_shadowcolor: Int1,
    pub uniform_shadowcolor0: Int1,
    pub uniform_shadowcolor1: Int1,
    pub uniform_noisetex: Int1,
    pub uniform_gcolor: Int1,
    pub uniform_gdepth: Int1,
    pub uniform_gnormal: Int1,
    pub uniform_composite: Int1,
    pub uniform_gaux1: Int1,
    pub uniform_gaux2: Int1,
    pub uniform_gaux3: Int1,
    pub uniform_gaux4: Int1,
    pub uniform_colortex0: Int1,
    pub uniform_colortex1: Int1,
    pub uniform_colortex2: Int1,
    pub uniform_colortex3: Int1,
    pub uniform_colortex4: Int1,
    pub uniform_colortex5: Int1,
    pub uniform_colortex6: Int1,
    pub uniform_colortex7: Int1,
    pub uniform_gdepthtex: Int1,
    pub uniform_depthtex2: Int1,
    pub uniform_colortex8: Int1,
    pub uniform_colortex9: Int1,
    pub uniform_colortex10: Int1,
    pub uniform_colortex11: Int1,
    pub uniform_colortex12: Int1,
    pub uniform_colortex13: Int1,
    pub uniform_colortex14: Int1,
    pub uniform_colortex15: Int1,
    pub uniform_colorimg0: Int1,
    pub uniform_colorimg1: Int1,
    pub uniform_colorimg2: Int1,
    pub uniform_colorimg3: Int1,
    pub uniform_colorimg4: Int1,
    pub uniform_colorimg5: Int1,
    pub uniform_shadowcolorimg0: Int1,
    pub uniform_shadowcolorimg1: Int1,
    pub uniform_tex: Int1,
    pub uniform_held_item_id: Int1,
    pub uniform_held_block_light_value: Int1,
    pub uniform_held_item_id2: Int1,
    pub uniform_held_block_light_value2: Int1,
    pub uniform_fog_mode: Int1,
    pub uniform_world_time: Int1,
    pub uniform_world_day: Int1,
    pub uniform_moon_phase: Int1,
    pub uniform_frame_counter: Int1,
    pub uniform_terrain_icon_size: Int1,
    pub uniform_is_eye_in_water: Int1,
    pub uniform_hide_gui: Int1,
    pub uniform_instance_id: Int1,
    pub uniform_render_stage: Int1,
}

pub struct ShaderParametersInt2 {
    pub uniform_eye_brightness: Int2,
    pub uniform_eye_brightness_smooth: Int2,
    pub uniform_terrain_texture_size: Int2,
    pub uniform_atlas_size: Int2,
}

pub struct ShaderParametersInt3 {}

pub struct ShaderParametersInt4 {
    pub uniform_blend_func: Int4,
}

pub struct ShaderParametersMatrix4f {
    pub uniform_gbuffer_model_view: Matrix4,
    pub uniform_gbuffer_model_view_inverse: Matrix4,
    pub uniform_gbuffer_previous_projection: Matrix4,
    pub uniform_gbuffer_projection: Matrix4,
    pub uniform_gbuffer_projection_inverse: Matrix4,
    pub uniform_gbuffer_previous_model_view: Matrix4,
    pub uniform_shadow_projection: Matrix4,
    pub uniform_shadow_projection_inverse: Matrix4,
    pub uniform_shadow_model_view: Matrix4,
    pub uniform_shadow_model_view_inverse: Matrix4,
}
