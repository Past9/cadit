#version 450

layout(location = 0) in vec3 normal;
layout(location = 1) flat in uint material_idx;

struct PointLight {
    vec3 position;
    vec3 color;
    float intensity;
};

struct AmbientLight {
    vec3 color;
    float intensity;
};

struct DirectionalLight {
    vec3 direction;
    vec3 color;
    float intensity;
};

struct Material {
    vec4 diffuse;
    float roughness;
};


layout(std140, set = 0, binding = 0) readonly buffer PointLightBuffer {
    PointLight data[];
} point_lights;

layout(std140, set = 0, binding = 1) readonly buffer AmbientLightBuffer {
    AmbientLight data[];
} ambient_lights;

layout(std140, set = 0, binding = 2) readonly buffer DirectionalLightBuffer {
    DirectionalLight data[];
} directional_lights;

layout(std140, set = 0, binding = 3) readonly buffer MaterialBuffer {
    Material data[];
} materials;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(point_lights.data[0].color.rgb, 1.0);
    f_color = vec4(directional_lights.data[1].color.rgb, 1.0);
    f_color = vec4(ambient_lights.data[1].color.rgb, 1.0);

    Material material = materials.data[material_idx];
    f_color = material.diffuse;

    vec3 ambient = vec3(0, 0, 0);
    for (int i = 0; i < ambient_lights.data.length(); i++) {
        AmbientLight light = ambient_lights.data[i];
        ambient += light.color * light.intensity;
    }
    f_color.rgb *= ambient;


}