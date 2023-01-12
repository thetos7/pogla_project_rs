
layout(location = 1) in mediump float in_lifetime;
layout(location = 2) in mediump float in_rotation_angle;
layout(location = 3) in vec3 in_position;

layout(location = 1) out float out_lifetime;
layout(location = 2) out float out_rotation_angle;
layout(location = 3) out vec3 out_position;

void main() {
    out_lifetime = in_lifetime;
    out_rotation_angle = in_rotation_angle;
    out_position = in_position;
    gl_Position = vec4(in_position, 1.0);
}