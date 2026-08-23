#version 330 core
layout(location = 0) in vec3 aPos;
//layout(location = 1) in vec2 aUV;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

//out vec2 fUv;

//out vec3 debugPos;

void main()
{
    //debugPos = aPos;

    //debugPos = aPos;
    gl_Position = uProjection * uView * uModel * vec4(aPos, 1.0);
    //gl_Position = uProjection * vec4(aPos, 1.0);
    //gl_Position = uView * vec4(aPos, 1.0);
    //gl_Position = uView * vec4(aPos.x, aPos.y, -10.0, 1.0);

    //gl_Position = uModel * vec4(aPos, 1.0);

    //gl_Position = uProjection * vec4(aPos, 1.0);
    //gl_Position = vec4(0, 0, 0, 1); // force everything to center

    //Multiplying our uniform with the vertex position, the multiplication order here does matter.
    //gl_Position = uProjection * uView * uModel * vec4(aPos, 1.0);

    //fUv = vUv;
}