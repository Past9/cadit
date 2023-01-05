#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) flat in uint material_idx;

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

    Material material = materials.data[material_idx];
    f_color = material.diffuse;

    vec3 final_light = vec3(0.0, 0.0, 0.0);

    // Ambient lights
    vec3 ambient = vec3(0, 0, 0);
    for (int i = 0; i < ambient_lights.data.length(); i++) {
        AmbientLight light = ambient_lights.data[i];
        ambient += light.color * light.intensity;
    }
    final_light += ambient;

    // Directional lights
    vec3 directional = vec3(0, 0, 0);
    for (int i = 0; i < directional_lights.data.length(); i++) {
        DirectionalLight light = directional_lights.data[i];
        
        directional += light.color * dot(normal, -light.direction) * light.intensity;
    }
    final_light += directional;

    // Point lights
    vec3 point = vec3(0, 0, 0);
    for (int i = 0; i < point_lights.data.length(); i++) {
        PointLight light = point_lights.data[i];

        vec3 dir_to_light = light.position - position;        
        float dist_to_light = length(dir_to_light);

        point += light.color * dot(normal, normalize(dir_to_light)) * light.intensity / pow(dist_to_light, 2);
    }
    final_light += point;

    f_color.rgb *= final_light;

}