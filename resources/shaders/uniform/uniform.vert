#version 450

layout(location = 0) in vec3 position;

uniform mat4 projection;
uniform mat4 model_transform;
uniform mat4 view_transform;

void main() {
    gl_Position = projection * view_transform * model_transform * vec4(position, 1.0);
}
