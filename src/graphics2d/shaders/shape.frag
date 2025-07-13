#version 330 core
uniform vec3 geometryColor;
out vec4 FragColor;
void main()
{
    FragColor = vec4(geometryColor, 1);
}