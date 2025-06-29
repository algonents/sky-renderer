#version 330 core
layout(points) in; // Input is a single point
layout(triangle_strip, max_vertices = 3) out; // Output is an independent triangle

void main() {
    vec4 center = gl_in[0].gl_Position;

    // Emit vertices in counterclockwise order
    gl_Position = center + vec4(-0.01, -0.01, 0.0, 0.0); // Vertex 1 (bottom left)
    EmitVertex();

    gl_Position = center + vec4(0.01, -0.01, 0.0, 0.0); // Vertex 2 (bottom right)
    EmitVertex();

    gl_Position = center + vec4(0.0, 0.01, 0.0, 0.0); // Vertex 3 (top)
    EmitVertex();
}