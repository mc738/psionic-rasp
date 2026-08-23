#version 330 core
//in vec2 fUv;

//uniform sampler2D uTexture0;
in vec3 debugPos;
out vec4 FragColor;

void main()
{
    FragColor = vec4(debugPos * 0.001, 1.0);
    //FragColor = texture(uTexture0, fUv);
}