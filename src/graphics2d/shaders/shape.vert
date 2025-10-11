#version 330 core
layout (location = 0) in vec2 aPos;

// Zoom/projection only (no per-object translate here)
uniform mat4 zoom_transform;

// Per-draw translation in screen/pixel coords
uniform vec2 u_offset;

void main()
{
    vec2 p = aPos + u_offset;
    gl_Position = zoom_transform * vec4(p, 0.0, 1.0);
}