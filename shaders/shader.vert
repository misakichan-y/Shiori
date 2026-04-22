#version 450

layout(location = 0) in vec2 instance_pos;
layout(location = 1) in float instance_rot;
layout(location = 2) in vec2 instance_scale;

vec2 positions[3] = vec2[](
    vec2(0.0, -0.5),
    vec2(0.5, 0.5),
    vec2(-0.5, 0.5)
);

void main() {
    vec2 pos = positions[gl_VertexIndex];

    pos *= instance_scale;

    float c = cos(instance_rot);
    float s = sin(instance_rot);
    pos = vec2(
        pos.x * c - pos.y * s,
        pos.x * s + pos.y * c
    );

    pos += instance_pos;

    gl_Position = vec4(pos, 0.0, 1.0);
}