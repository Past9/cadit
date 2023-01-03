#version 450

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

layout(std430, set = 0, binding = 0) readonly buffer PointLightBuffer {
    PointLight lights[];
} pointLightBuffer;

layout(std430, set = 0, binding = 1) readonly buffer AmbientLightBuffer {
    AmbientLight lights[];
} ambientLightBuffer;

layout(std430, set = 0, binding = 2) readonly buffer DirectionalLightBuffer {
    DirectionalLight lights[];
} directionalLightBuffer;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(pointLightBuffer.lights[0].color.rgb, 1.0);
    f_color = vec4(directionalLightBuffer.lights[1].color.rgb, 1.0);
    f_color = vec4(ambientLightBuffer.lights[1].color.rgb, 1.0);
}