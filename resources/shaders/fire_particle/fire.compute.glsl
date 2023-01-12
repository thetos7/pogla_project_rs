#line 1
layout(local_size_x = 1024) in;

precision mediump float;

struct FireParticle
{
    float lifetime;
    float rotation;
    vec3 position;
    float angular_velocity;
    vec3 velocity;
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
    particle.position += particle.velocity * delta_time;
    particle.lifetime += delta_time;
    
    if (particle.lifetime > max_lifetime) {
        particle.position = vec3(0.0);
        particle.lifetime = 0.0;
    }

    particles[idx] = particle;
}