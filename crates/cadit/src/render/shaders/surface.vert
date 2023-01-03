#version 450

layout(location = 0) in vec3 position;

layout(push_constant) uniform PushConstants {
    mat4 model_matrix;
    mat4 projection_matrix;
} push_constants;

void main() {
    gl_Position = push_constants.projection_matrix * push_constants.model_matrix * vec4(position, 1.0);
}