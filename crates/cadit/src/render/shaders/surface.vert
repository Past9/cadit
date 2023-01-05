#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in uint material_idx;

layout(push_constant) uniform PushConstants {
    mat4 model_matrix;
    mat4 projection_matrix;
} push_constants;

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out uint v_material_idx;

void main() {
    gl_Position = push_constants.projection_matrix * push_constants.model_matrix * vec4(position, 1.0);
    v_position = position;
    v_normal = normal;
    v_material_idx = material_idx;
}