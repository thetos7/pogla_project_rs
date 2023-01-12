#line 1
// include "particle_helpers.glsl" before
#define IN_POINT_COUNT 1
#define OUT_POINT_COUNT 4

layout(points) in;
layout(triangle_strip, max_vertices=OUT_POINT_COUNT) out;

layout(location = 1) in float in_lifetime[IN_POINT_COUNT];
layout(location = 2) in float in_rotation_angle[IN_POINT_COUNT];
layout(location = 3) in vec3 in_position[IN_POINT_COUNT];

layout(location = 1) out vec2 out_uv;
layout(location = 2) out float out_lifetime;

uniform mat4 projection;
uniform mat4 view_transform;
uniform mat4 model_transform;
uniform vec3 camera_forward;
uniform vec3 camera_up;

const vec2 uvs[4] = {
    vec2(0.0, 1.0),// top left
    vec2(0.0, 0.0),// bottom left
    vec2(1.0, 1.0),// top right
    vec2(1.0, 0.0) // bottom right
};

void main() {

    vec4 vertices[4];

    generate_billboard_vertices(
        vertices,
        in_position[0],
        camera_forward,
        camera_up,
        0.1,
        in_rotation_angle[0],
        projection,
        view_transform,
        model_transform
    );

    for (int i = 0; i < 4; ++i) {
        gl_Position = vertices[i];
        out_lifetime = in_lifetime[0];
        out_uv = uvs[i];
        EmitVertex();
    }
    EndPrimitive();
}