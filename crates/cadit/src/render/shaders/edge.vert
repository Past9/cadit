#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 expand;

layout(push_constant) uniform PushConstants {
    mat4 model_matrix;
    mat4 projection_matrix;
} push_constants;

void main() {
    vec4 unadjusted_transformed_pos = push_constants.model_matrix * vec4(position, 1.0);

    float offset = -unadjusted_transformed_pos.z / 2000;
    vec3 adjusted_pos = position + expand * offset;

    gl_Position = push_constants.projection_matrix * push_constants.model_matrix * vec4(adjusted_pos, 1.0);
}