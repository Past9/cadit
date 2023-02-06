#version 450

layout(input_attachment_index = 1, set = 0, binding = 0) uniform subpassInputMS u_opaque;
layout(input_attachment_index = 1, set = 0, binding = 1) uniform subpassInputMS u_accum;
layout(input_attachment_index = 1, set = 0, binding = 2) uniform subpassInputMS u_transmit;

layout(location = 0) out vec4 color;

void main() {
    vec4 opaque = subpassLoad(u_opaque, gl_SampleID);
    vec4 accum = subpassLoad(u_accum, gl_SampleID);
    vec4 transmit = subpassLoad(u_transmit, gl_SampleID);
    color = opaque * vec4(transmit.rgb, 1.0) + accum;
}