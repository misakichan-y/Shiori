#version 450

layout(location = 0) out vec4 out_color;

layout(push_constant) uniform Push {
    vec4 transform;
    float alpha;
} pc;

void main() {
    out_color = vec4(0.0, 0.0, 0.0, pc.alpha);
}