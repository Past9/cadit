struct Material {
    vec4 diffuse;
    float roughness;
};

layout(std140, set = 0, binding = 3) readonly buffer MaterialBuffer {
    Material data[];
} materials;