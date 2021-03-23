#version 120
#extension GL_EXT_gpu_shader4 : enable
//auto attribute location
attribute vec4 position;//0
attribute vec3 normal;//1
attribute vec2 uv;//2
attribute vec4 additional_uvs[4];//3,4,5,6
attribute vec3 binormal;//7
flat varying vec3 to_frag0;
varying vec3 to_frag1;
flat varying vec3 to_frag2[4];
varying vec3 to_frag3[4];
void main() {

}
