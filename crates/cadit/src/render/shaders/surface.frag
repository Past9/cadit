#version 450

layout(location = 0) in vec4 albedo;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = albedo;
}