#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) flat in uint material_idx;

#include "surface_lighting_buffers.frag"
#include "translucent_material_buffer.frag"

//layout(input_attachment_index = 0, set = 0, binding = 4) uniform subpassInputMS u_color;
layout(input_attachment_index = 1, set = 0, binding = 4) uniform subpassInputMS u_depth;

layout(location = 0) out vec4 color;

void main() {
    float self_depth = gl_FragCoord.z;
    float in_depth = subpassLoad(u_depth, gl_SampleID).x;

    if (in_depth < self_depth) {
        discard;
    }

    Material material = materials.data[material_idx];
    color = vec4(material.diffuse);

    #include "surface_lighting.frag"

    color.rgb *= lighting;
}