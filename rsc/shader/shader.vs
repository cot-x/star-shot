#version 140

in vec3 iPosition;
in vec4 iColor;
in vec2 iTexCoords;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec4 Color;
out vec2 TexCoords;

void main()
{
    vec3 FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    Color = iColor;
    TexCoords = iTexCoords;
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}
