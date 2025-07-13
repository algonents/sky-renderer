#version 330 core
out vec4 FragColor;

uniform vec3 geometryColor;

void main() {
    // Coordinates in gl_Point are from (0,0) to (1,1), center at (0.5, 0.5)
    vec2 coord = gl_PointCoord - vec2(0.5);
    float dist = length(coord);

    // Discard anything outside radius
    if (dist > 0.5) {
        discard;
    }

    FragColor = vec4(geometryColor, 1.0);
}