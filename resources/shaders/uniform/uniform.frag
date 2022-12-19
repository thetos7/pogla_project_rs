#version 450

layout(location = 0) out vec4 color;

uniform vec4 object_color;

void main() {
    color = object_color;
}
