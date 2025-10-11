#version 330 core
layout (location = 0) in vec2 aPos;           // mesh-local vertex
layout (location = 1) in vec2 aInstanceXY;    // optional; if disabled => (0,0)

uniform mat4 u_zoomTransform;                 // zoom/projection only
uniform vec2 u_offset;                        // single-shape translation (uniform). Forced to 0 when instancing

void main() {
    vec2 p = aPos + (u_offset + aInstanceXY);
    gl_Position = u_zoomTransform * vec4(p, 0.0, 1.0);
}