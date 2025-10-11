#version 330 core
layout (location = 0) in vec2 aPos;

// Zoom/projection only (no per-object translate here)
uniform mat4 u_zoomTransform;

// Per-draw translation in screen/pixel coords
uniform vec2 u_offset;

void main()
{
    vec2 p = aPos + u_offset;
    gl_Position = u_zoomTransform * vec4(p, 0.0, 1.0);
}