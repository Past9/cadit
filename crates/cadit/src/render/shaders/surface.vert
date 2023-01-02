#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 albedo;

layout(push_constant) uniform PushConstants {
    mat4 view_matrix;
    mat4 model_matrix;
    mat4 perspective_matrix;
} push_constants;

layout(location = 0) out vec4 v_color;
void main() {
    mat4 model_view_matrix = push_constants.view_matrix * push_constants.model_matrix;
    gl_Position = push_constants.perspective_matrix * model_view_matrix * vec4(position, 1.0);
    v_color = albedo;
}