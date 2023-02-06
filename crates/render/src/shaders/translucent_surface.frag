#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) flat in uint material_idx;

#include "surface_lighting_buffers.frag"
#include "translucent_material_buffer.frag"

//layout(input_attachment_index = 0, set = 0, binding = 4) uniform subpassInputMS u_color;
layout(input_attachment_index = 1, set = 0, binding = 4) uniform subpassInputMS u_depth;

layout(location = 0) out vec4 A;
layout(location = 1) out vec3 beta;

void computeOutput(
    vec3 L_r, // Radiance
    float alpha, // Alpha factor?
    vec3 t, // Transmission
    out vec4 A, // Accumulated reflected light
    out vec3 beta // Transmission
) {
    float netCoverage = alpha * (1.0 - dot(t, vec3(1.0 / 3.0)));
    float tmp = (1.0 - gl_FragCoord.z * 0.99) * netCoverage * 10.0;
    float w = clamp(tmp * tmp * tmp, 0.01, 30.0);

    A = vec4(L_r * alpha, netCoverage) * w;
    beta = alpha * (vec3(1.0) - t) * (1.0 / 3.0);
}

void main() {
    float self_depth = gl_FragCoord.z;
    float in_depth = subpassLoad(u_depth, gl_SampleID).x;

    if (in_depth < self_depth) {
        discard;
    }

    Material material = materials.data[material_idx];
    vec3 reflected = material.diffuse.rgb;

    #include "surface_lighting.frag"

    reflected *= lighting;

    computeOutput(
        reflected,
        material.diffuse.a,
        material.diffuse.rgb, // * (1.0 - material.diffuse.a),
        //material.diffuse.rgb * (1.0 - material.diffuse.a),
        A,
        beta
    );
}