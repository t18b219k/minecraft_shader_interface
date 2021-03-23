in vec4 Position;
vec4 ftransform(){
    return _gl_ModelViewProjection*Position;
}
