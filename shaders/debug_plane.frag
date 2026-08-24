#version 330 core

in vec2 vUV;
out vec4 fragColor;

void main() {
    // Checker pattern
    float check = mod(floor(vUV.x) + floor(vUV.y), 2.0);

    // Two colors
    vec3 colorA = vec3(0.85, 0.85, 0.85);
    vec3 colorB = vec3(0.65, 0.65, 0.65);

    if (vUV.x < 0.02 && vUV.x > -0.02 && vUV.y >= 0.)
    {
        fragColor = vec4(0., 1., 0., 1.);

    }
    else
    {
        fragColor = vec4(mix(colorA, colorB, check), 1.0);
    }

    //fragColor = vec4(0.2, 0.2, 0.2, 1.0);
}