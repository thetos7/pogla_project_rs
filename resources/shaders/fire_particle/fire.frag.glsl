
layout(location = 1) in vec2 in_uv;
layout(location = 2) in float in_lifetime;

out vec4 color;

uniform float max_fire_lifetime;
uniform float max_smoke_lifetime;

uniform vec4 fire_color;
uniform vec4 early_smoke_color;
uniform vec4 end_smoke_color;

void main() {
    color = mix(
        fire_color,
        early_smoke_color,
        smoothstep(0.0, max_fire_lifetime, in_lifetime)
    );
    color = mix(
        color,
        end_smoke_color,
        smoothstep(max_fire_lifetime, max_smoke_lifetime, in_lifetime)
    );
}