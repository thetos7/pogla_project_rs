#line 1
layout(local_size_x = 1024) in;
#define TWO_PI 6.283185307179586
struct FireParticle
{
    float lifetime;
    float rotation;
    vec3 position;
    float angular_velocity;
    vec3 velocity;
    vec3 initial_position;
};

layout(std430, binding = 1) buffer particle_buffer {
    FireParticle particles[];
};

uniform uint particle_count;
uniform float delta_time;
uniform float max_lifetime;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    if(idx > particle_count) {
        return;
    }

    FireParticle particle = particles[idx];
    particle.rotation += particle.angular_velocity * delta_time;
    particle.rotation = mod(particle.rotation, TWO_PI);
    particle.position += particle.velocity * delta_time;
    particle.lifetime += delta_time;
    
    if (particle.lifetime > max_lifetime) {
        particle.position = particle.initial_position;
        particle.lifetime = 0.0;
    }

    particles[idx] = particle;
}