#version 330 core

in vec2 TexCoord;
out vec4 FragColor;

// texture samples
uniform sampler2D texture1;

void main() {
    FragColor = texture(texture1, TexCoord);
}