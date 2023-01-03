#version 450

layout(location = 0) in vec4 albedo;

struct Light {
    vec3 position;
    vec3 color;
    float intensity;
};

layout(std430, set = 0, binding = 0) readonly buffer LightBuffer {
    Light lights[];
} lightBuffer;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(lightBuffer.lights[0].color.rgb, 1.0);
    //f_color = vec4(vec3(gl_FragCoord.z), 1.0);

}