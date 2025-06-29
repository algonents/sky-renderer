#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 aColor;
uniform mat4 transform;
out vec3 ourColor;
    
void main()
{
    gl_Position = transform * vec4(aPos, 0.0, 1.0);
    ourColor = aColor;
}
    