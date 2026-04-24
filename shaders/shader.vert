#version 450

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec2 in_uv;

layout(location = 0) out vec2 out_uv;

layout(push_constant) uniform Offset {
    vec4 pos;
} offset;

void main() {
    vec2 final = in_pos + offset.pos.xy;
    gl_Position = vec4(final, 0.0, 1.0);
    out_uv = in_uv;
}