#ifndef SHADER_H
#define SHADER_H

#include <cwchar>
#include <glad/glad.h>
#include <glm/glm.hpp> 
#include <string>
#include <fstream>
#include <sstream>
#include <iostream>
  
std::string get_file_contents(const std::string filename);


class Shader {
public:
    GLuint ID;
    Shader(const std::string vertexPath, const std::string fragmentPath); 
    void use();
    void setBool(const char* name, bool value) const;
    void setInt(const char* name, int value) const;
    void setFloat(const char* name, float value) const;
    void setVec2(const char* name, float v1, float v2) const;
    void setVec3(const char* name, float v1, float v2, float v3) const; 
    void setVec2(const char* name, glm::vec2 v) const;
    void setVec3(const char* name, glm::vec3 v) const;
    bool getBool(const char* name) const;
    int getInt(const char* name) const;
    float getFloat(const char* name) const;
    void deactivate();
    

private:
    void checkCompileErrors(GLuint shader);
    void checkLinkErrors(GLuint program);
};
  
#endif
