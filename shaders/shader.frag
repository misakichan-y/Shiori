#version 450

layout(location = 0) in vec2 in_uv;
layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform sampler2D tex;

layout(push_constant) uniform Push {
    vec4 transform;
    float alpha;
} pc;

void main() {
    vec4 color = texture(tex, in_uv);

    vec3 final = mix(color.rgb, vec3(0.0), pc.alpha);

    out_color = vec4(final, 1.0);
}