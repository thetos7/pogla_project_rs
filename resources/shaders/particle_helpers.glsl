

const vec2 BILLBOARD_VERTICES[4] = {
    vec2(-0.5, 0.5), // top left
    vec2(-0.5,-0.5), // bottom left
    vec2(0.5, 0.5), // top right
    vec2(0.5, -0.5), // bottom right
};

mat4 scale(float factor) {
    return mat4(
        factor, 0.0, 0.0, 0.0, // column 1
        0.0, factor, 0.0, 0.0, // column 2
        0.0, 0.0, factor, 0.0, // column 3
        0.0, 0.0, 0.0,    1.0 // column 4
    );
}
mat4 basis_change(vec3 x_axis, vec3 y_axis, vec3 z_axis) {
    return mat4(
        x_axis, 0.0, // column 1
        y_axis, 0.0, // column 2
        z_axis, 0.0, // column 3
        0.0,0.0,0.0,1.0 // column 4
    );
}

mat4 rotation_y (float angle) {
    return mat4(
        cos(angle), 0.0, -sin(angle), 0.0, // column 1
        0.0, 1.0, 0.0, 0.0, // column 2
        sin(angle), 0.0, cos(angle), 0.0, // column 3
        0.0, 0.0, 0.0, 1.0 // column 4
    );
}

mat4 translation (vec3 amount) {
    return mat4(
        1.0,0.0,0.0,0.0, // column 1
        0.0,1.0,0.0,0.0, // column 2
        0.0,0.0,1.0,0.0, // column 3
        amount,1.0 // column 4
    );
}

void generate_billboard_vertices(
    inout vec4 positions[4],
    vec3 position,
    vec3 camera_forward,
    vec3 up,
    float scale_factor,
    float rotation_angle,
    mat4 projection,
    mat4 view_transform,
    mat4 model_transform
) {
    vec3 y_axis = -camera_forward;
    vec3 x_axis = normalize(cross(up, y_axis));
    vec3 z_axis = normalize(cross(y_axis, x_axis));
    mat4 transform = 
        projection 
        * view_transform 
        * model_transform 
        * translation(position) 
        * basis_change(x_axis, y_axis, z_axis) 
        * scale(scale_factor) 
        * rotation_y(rotation_angle);
    
    for(int i = 0; i < 4; ++i) {
        vec2 vert = BILLBOARD_VERTICES[i];
        positions[i] = transform * vec4(vert.x, 0.0, vert.y, 1.0);
    }
}
