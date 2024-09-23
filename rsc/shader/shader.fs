#version 140

in vec4 Color;
in vec2 TexCoords;

uniform sampler2D uScreenTexture;
uniform int useTexture;

void main()
{
    gl_FragColor = (useTexture != 0) ? texture(uScreenTexture, TexCoords).rgba * Color : Color;
}
