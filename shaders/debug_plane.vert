#version 330 core

layout(location = 0) in vec3 aPosition;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec2 vUV;

void main()
{
    // World position
    vec4 worldPos = uModel * vec4(aPosition, 1.0);

    // Generate UVs from XZ world coordinates
    vUV = worldPos.xz * 0.1;   // scale controls size of checks

    gl_Position = uProjection * uView * uModel * vec4(aPosition, 1.0);
}