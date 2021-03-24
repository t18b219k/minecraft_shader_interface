struct _gl_DepthRangeParameters{
    float near;
    float far;
    float diff;
};
struct _gl_PointParameters {
    float size;
    float sizeMin;
    float sizeMax;
    float fadeThresholdSize;
    float distanceConstantAttenuation;
    float distanceLinearAttenuation;
    float distanceQuadraticAttenuation;
};
struct _gl_MaterialParameters {
    vec4  emission;    // Ecm
    vec4  ambient;     // Acm
    vec4  diffuse;     // Dcm
    vec4  specular;    // Scm
    float shininess;   // Srm
};
struct _gl_LightSourceParameters {
    vec4  ambient;             // Acli
    vec4  diffuse;             // Dcli
    vec4  specular;            // Scli
    vec4  position;            // Ppli
    vec4  halfVector;          // Derived: Hi
    vec3  spotDirection;       // Sdli
    float spotExponent;        // Srli
    float spotCutoff;          // Crli (range: [0.0,90.0], 180.0)
    float spotCosCutoff;       // Derived: cos(Crli) (range: [1.0,0.0],-1.0)
    float constantAttenuation; // K0
    float linearAttenuation;   // K1
    float quadraticAttenuation;// K2
};
struct _gl_LightModelParameters {
    vec4  ambient;       // Acs
};
struct _gl_LightModelProducts {
    vec4  sceneColor;     // Derived. Ecm + Acm * Acs
};
struct _gl_LightProducts {
    vec4  ambient;        // Acm * Acli
    vec4  diffuse;        // Dcm * Dcli
    vec4  specular;       // Scm * Scli
};
struct _gl_FogParameters {
    vec4 color;
    float density;
    float start;
    float end;
    float scale;   // Derived:   1.0 / (end - start)
};
uniform _gl_EmbeddedUniforms{
    mat4 _gl_ModelViewMatrix;
    mat4 _gl_ProjectionMatrix;
    mat4 _gl_ModelViewProjectionMatrix;
    mat4 _gl_TextureMatrix[gl_MaxTextureCoords];
    mat3 _gl_NormalMatrix;// transpose of the inverse of the upper leftmost 3x3 of gl_ModelViewMatrix
    mat4 _gl_ModelViewMatrixInverse;
    mat4 _gl_ProjectionMatrixInverse;
    mat4 _gl_ModelViewProjectionMatrixInverse;
    mat4 _gl_TextureMatrixInverse[gl_MaxTextureCoords];
    mat4 _gl_ModelViewMatrixTranspose;
    mat4 _gl_ProjectionMatrixTranspose;
    mat4 _gl_ModelViewProjectionMatrixTranspose;
    mat4 _gl_TextureMatrixTranspose[gl_MaxTextureCoords];
    mat4 _gl_ModelViewMatrixInverseTranspose;
    mat4 _gl_ProjectionMatrixInverseTranspose;
    mat4 _gl_ModelViewProjectionMatrixInverseTranspose;
    mat4 _gl_TextureMatrixInverseTranspose[gl_MaxTextureCoords];
    float _gl_NormalScale;
    _gl_DepthRangeParameters _gl_DepthRange;
    vec4 _gl_ClipPlane[gl_MaxClipPlanes];
    _gl_PointParameters _gl_Point;
    _gl_MaterialParameters  _gl_FrontMaterial;
    _gl_MaterialParameters  _gl_BackMaterial;
    _gl_LightSourceParameters  _gl_LightSource[gl_MaxLights];
    _gl_LightModelParameters _gl_LightModel;
    _gl_LightModelProducts _gl_FrontModelProduct;
    _gl_LightModelProducts _gl_BackLightModelProduct;
    _gl_LightProducts _gl_FrontLightProduct[gl_MaxLights];
    _gl_LightProducts _gl_BackLightProduct[gl_MaxLights];
    vec4  _gl_TextureEnvColor[gl_MaxTextureUnits];
    vec4  _gl_EyePlaneS[gl_MaxTextureCoords];
    vec4  _gl_EyePlaneT[gl_MaxTextureCoords];
    vec4  _gl_EyePlaneR[gl_MaxTextureCoords];
    vec4  _gl_EyePlaneQ[gl_MaxTextureCoords];
    vec4  _gl_ObjectPlaneS[gl_MaxTextureCoords];
    vec4  _gl_ObjectPlaneT[gl_MaxTextureCoords];
    vec4  _gl_ObjectPlaneR[gl_MaxTextureCoords];
    vec4  _gl_ObjectPlaneQ[gl_MaxTextureCoords];
    _gl_FogParameters _gl_Fog;
};