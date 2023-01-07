#version 450

layout(location = 0) out vec4 f_color;

void main() {
    float dist = length(gl_PointCoord - vec2(0.5));

    if (dist > 5) {
        discard;
    }

    f_color = vec4(0.0);

    if (dist <= 0.5) {
        float color = 1 - pow((dist / 0.35), 9);
        float alpha = 1 - pow((dist / 0.5), 9);
        f_color = vec4(color, color, color, alpha);
    } else {
        discard;
    }
}