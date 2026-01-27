#define GLM_ENABLE_EXPERIMENTAL
#define GLM_SWIZZLE_XYZW
#define GLM_SWIZZLE_STQP
#include "shader.h"
#include <string>
#include <cmath>
#include <vector>
#include <algorithm>
#include "random.h"
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <glm/common.hpp>
#include <glm/ext/matrix_float2x2.hpp>
#include <glm/gtx/string_cast.hpp>
#include <glm/gtx/vec_swizzle.hpp>

namespace Scene {

extern float timeElapsed;
extern glm::vec2 mousePos;

extern int shadowRays;
extern int raysPerPixel;
extern int maxBounceLimit;

extern glm::vec2 screenPixels;
extern float aspectRatio;

extern float fov;
extern glm::vec3 camPos;
extern glm::vec3 camLookAt;
extern glm::vec3 upDir;
extern float rayOriginToScreenDistance;
extern bool useMouseForCamera;
extern glm::vec3 camPosFromSettings;
extern glm::vec3 camLookAtFromSettings;

extern int selectedObjectIndex;

extern bool refresh;

struct Camera {
    glm::vec3 position;
    glm::vec3 llc;
    glm::vec3 horizontal;
    glm::vec3 vertical;
    glm::vec3 forward;
    glm::vec3 right;
    glm::vec3 up;
};
Camera getCam(float fov, float aspectRatio, glm::vec3 lookFrom, glm::vec3 lookAt, glm::vec3 vup, float rayOriginToScreenDistance);

struct Ray {
    glm::vec3 origin;
    glm::vec3 direction;
};
Ray getRayToScreen(Camera cam, float u, float v);

struct Material {
    int type; //0 = lambertian, 1 = metal, 2 = dielectric, 3 = emissive
    glm::vec3 color; //for type = 0,1,3
    float fuzz; //for type = 1
    float refIndex; //for type = 2
    float emissionStrength; //for type = 3
    Material(int type, glm::vec3 color, float fuzz, float refIndex, float emissionStrength);
    Material();
};
struct Sphere {
    glm::vec3 center;
    float radius;
    Material mat;
    Sphere(glm::vec3 center, float radius, Material mat);
    Sphere();
};
struct Triangle {
    glm::vec3 pointA;
    glm::vec3 pointB;
    glm::vec3 pointC;
    Material mat;
    Triangle(glm::vec3 pointA, glm::vec3 pointB, glm::vec3 pointC, Material mat);
    Triangle();
};
struct Light {
    int type; //0 = point, 1 = directional
    glm::vec3 pos; //for type = 0
    glm::vec3 dir; //for type = 1
    glm::vec3 color;
    float maxIntensity; //inverse square law for type = 0, constant for type = 1
    bool softShadows;
    Light(int type, glm::vec3 pos, glm::vec3 dir, glm::vec3 color, float maxIntensity, bool softShadows);
    Light();
};
extern std::vector<Sphere> spheres;
extern std::vector<Light> lights;

struct BVHNodeCPU {
    glm::vec3 boundsMin;
    glm::vec3 boundsMax;
    int left;
    int right;
    int sphereIndex;
};
extern std::vector<BVHNodeCPU> bvhNodes;
extern int bvhRoot;

int buildBVH(std::vector<int>& indices);

void initialize(Shader shader, int screenWidth, int screenHeight, int scrTexture);
void initializeUniforms(Shader shader);
void update(Shader shader, GLFWwindow* window, int screenWidth, int screenHeight, bool directoutpass, int accpasses, double dTime);
void updateUniforms(Shader shader, GLFWwindow* window, int scrWidth, int scrHeight);
bool mouseCameraMovement(GLFWwindow* window, int screenWidth, int screenHeight);
void moveCamToSelectedSphere(glm::vec3 newPos, glm::vec3 newLookAt, float speed);
glm::vec3 lerpVec(glm::vec3 a, glm::vec3 b, float t);
Ray getRayToScreen(Camera cam, float u, float v);
bool sphereIntersection(Sphere sphere, Ray ray, float* dist);
void sphereSelect(float mouseX, float mouseY, int screenWidth, int screenHeight);
Sphere addNew();
}
