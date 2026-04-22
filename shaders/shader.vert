#version 450

// 🔥 Camera (push constant)
layout(push_constant) uniform Camera {
    mat3 cam;
} camera;

// 🔥 Instance data (from buffer)
layout(location = 0) in vec2 instance_pos;
layout(location = 1) in float instance_rot;
layout(location = 2) in vec2 instance_scale;

// 🔥 Triangle vertices (hardcoded)
vec2 positions[3] = vec2[](
    vec2(0.0, -0.5),
    vec2(0.5, 0.5),
    vec2(-0.5, 0.5)
);

void main() {
    vec2 pos = positions[gl_VertexIndex];

    // 🔹 Scale
    pos *= instance_scale;

    // 🔹 Rotate
    float c = cos(instance_rot);
    float s = sin(instance_rot);

    pos = vec2(
        pos.x * c - pos.y * s,
        pos.x * s + pos.y * c
    );

    // 🔹 Translate
    pos += instance_pos;

    // 🔥 Apply camera
    vec3 world = vec3(pos, 1.0);
    vec3 final_pos = camera.cam * world;

    gl_Position = vec4(final_pos.xy, 0.0, 1.0);
}