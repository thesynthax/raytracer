#include "scene.h"

namespace Scene {

float timeElapsed;
double deltaTime;
glm::vec2 mousePos;

int shadowRays;
int raysPerPixel;
int maxBounceLimit;

glm::vec2 screenPixels;
float aspectRatio;

float fov;
glm::vec3 camPos;
glm::vec3 camLookAt;
glm::vec3 upDir;
float rayOriginToScreenDistance;
bool useMouseForCamera;
glm::vec3 camPosFromSettings;
glm::vec3 camLookAtFromSettings;

int selectedObjectIndex;
bool movingCamToSelectedObject;
float camMoveSpeed;
float camToObjDist;
glm::vec3 currRot;
float mouseSensitivity;

int screenTexture;
int accumulatedPasses;
bool directOutputPass;
bool refresh;

Camera getCam(float fov, float aspectRatio, glm::vec3 lookFrom, glm::vec3 lookAt, glm::vec3 vup, float rayOriginToScreenDistance) {
    float viewportHeight = rayOriginToScreenDistance * tan(glm::radians(fov)/2.0f)* 2.0f;
    float viewportWidth = viewportHeight * aspectRatio;

    Camera cam;
    cam.forward = glm::normalize(lookAt - lookFrom);
    cam.right = glm::normalize(cross(cam.forward, vup));
    cam.up = glm::cross(cam.right, cam.forward);

    cam.position = lookFrom;
    cam.horizontal = cam.right * viewportWidth;
    cam.vertical = cam.up * viewportHeight;
    cam.llc = cam.position - cam.horizontal / 2.0f - cam.vertical / 2.0f + cam.forward;
    return cam;
}

Material::Material() = default;
Material::Material(int type, glm::vec3 color, float fuzz, float refIndex, float emissionStrength) {
    this->type = type;
    this->color = color;
    this->fuzz = fuzz;
    this->refIndex = refIndex;
    this->emissionStrength = emissionStrength;
}

Sphere::Sphere() = default;
Sphere::Sphere(glm::vec3 center, float radius, Material mat) {
    this->center = center;
    this->radius = radius;
    this->mat = mat;
}

Triangle::Triangle() = default;
Triangle::Triangle(glm::vec3 pointA, glm::vec3 pointB, glm::vec3 pointC, Material mat) {
    this->pointA = pointA;
    this->pointB = pointB;
    this->pointC = pointC;
    this->mat = mat;
}

Light::Light() = default;
Light::Light(int type, glm::vec3 pos, glm::vec3 dir, glm::vec3 color, float maxIntensity, bool softShadows) {
    this->type = type;
    this->pos = pos;
    this->dir = dir;
    this->color = color;
    this->maxIntensity = maxIntensity;
    this->softShadows = softShadows;
}

std::vector<Sphere> spheres;
std::vector<Light> lights;

glm::mat2 rot2d(float angle) {
    float s = std::sin(angle);
    float c = std::cos(angle);

    return glm::mat2(c, -s, s, c);
}

void initialize(Shader shader, int screenWidth, int screenHeight, int scrTexture) {
    screenTexture = scrTexture;
    screenPixels = glm::vec2(screenWidth, screenHeight);
    aspectRatio = float(screenWidth)/float(screenHeight);
    fov = 70.0f;
    camPos = glm::vec3(0, 0, -10);
    camLookAt = glm::vec3(0, 0, 0);
    upDir = glm::vec3(0, 1, 0);
    rayOriginToScreenDistance = 1;
    raysPerPixel = 1;
    maxBounceLimit = 20;
    shadowRays = 10;
    useMouseForCamera = false;
    selectedObjectIndex = -1;
    movingCamToSelectedObject = false;
    camMoveSpeed = 3;
    camPosFromSettings = camPos;
    camLookAtFromSettings = camLookAt;
    camToObjDist = 4;
    mouseSensitivity = 0.02f;
    directOutputPass = true;
    accumulatedPasses = 0;

    spheres.push_back(Sphere(glm::vec3(0), 1, Material(1, glm::vec3(1), 0.1f, 0, 0)));
    spheres.push_back(Sphere(glm::vec3(3,0,5), 1, Material(3, glm::vec3(0.2f, 0.8f, 0.2f), 0, 0, 6)));
    spheres.push_back(Sphere(glm::vec3(-3,0,5), 1, Material(0, glm::vec3(0.8f, 0.2f, 0.2f), 0, 0, 0)));
    spheres.push_back(Sphere(glm::vec3(0, -21, 5), 20, Material(0, glm::vec3(0.4f, 0.1f, 0.5f), 0, 0, 0)));

    lights.push_back(Light(0, glm::vec3(0), glm::vec3(0), glm::vec3(1), 9, true));
    lights.push_back(Light(1, glm::vec3(0), glm::vec3(0), glm::vec3(1), 1, false));

    initializeUniforms(shader);
}

void initializeUniforms(Shader shader) {
    shader.setFloat("INFINITY", INFINITY);
    shader.setVec2("u_screenPixels", screenPixels);
    shader.setFloat("u_aspectRatio", aspectRatio);
    shader.setFloat("u_fov", fov);
    shader.setVec3("u_camPos", camPos);
    shader.setVec3("u_camLookAt", camLookAt);
    shader.setVec3("u_upDir", upDir);
    shader.setFloat("u_rayOriginToScreenDistance", rayOriginToScreenDistance);
    shader.setInt("u_raysPerPixel", raysPerPixel);
    shader.setInt("u_maxBounceLimit", maxBounceLimit);
    shader.setInt("u_shadowRays", shadowRays);
    shader.setInt("u_selectedObjectIndex", selectedObjectIndex);
    shader.setBool("u_directOutputPass", directOutputPass);
    shader.setBool("u_accumulatedPasses", accumulatedPasses);

    /*for (int i = 0; i < 9; i++) {
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].center").c_str(), rng(-2, 2), rng(-1, 1), rng(2, 6));
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].radius").c_str(), rng(0.25f, 1));
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].mat.color").c_str(), rng(0, 1), rng(0, 1), rng(0, 1));
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].mat.emission").c_str(), rng(0, 1), rng(0, 1), rng(0, 1));
    }*/
    
    for (int i = 0; i < spheres.size(); i++) {
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].center").c_str(), spheres[i].center);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].radius").c_str(), spheres[i].radius);
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].mat.color").c_str(), spheres[i].mat.color);
        shader.setInt(std::string("u_spheres[").append(std::to_string(i)).append("].mat.type").c_str(), spheres[i].mat.type);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.fuzz").c_str(), spheres[i].mat.fuzz);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.refIndex").c_str(), spheres[i].mat.refIndex);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.emissionStrength").c_str(), spheres[i].mat.emissionStrength);
    }

    for (int i = 0; i < lights.size(); i++) {
        shader.setInt(std::string("u_lights[").append(std::to_string(i)).append("].type").c_str(), lights[i].type);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].pos").c_str(), lights[i].pos);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].dir").c_str(), lights[i].dir);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].color").c_str(), lights[i].color);
        shader.setFloat(std::string("u_lights[").append(std::to_string(i)).append("].maxIntensity").c_str(), lights[i].maxIntensity);
        shader.setBool(std::string("u_lights[").append(std::to_string(i)).append("].softShadows").c_str(), lights[i].softShadows);
    }

    shader.setVec3(std::string("u_triangles[").append(std::to_string(0)).append("].pointA").c_str(), 0, -1, 5);
    shader.setVec3(std::string("u_triangles[").append(std::to_string(0)).append("].pointB").c_str(), 3, 4, 5);
    shader.setVec3(std::string("u_triangles[").append(std::to_string(0)).append("].pointC").c_str(), 0, 2, 5);
    shader.setInt(std::string("u_triangles[").append(std::to_string(0)).append("].mat.type").c_str(), 3);
    shader.setInt(std::string("u_triangles[").append(std::to_string(0)).append("].mat.emissionStrength").c_str(), 9);
    shader.setVec3(std::string("u_triangles[").append(std::to_string(0)).append("].mat.color").c_str(), 1, 1, 1);
}

double mouseX, mouseY;
void update(Shader shader, GLFWwindow* window, int screenWidth, int screenHeight, bool directoutpass, int accpasses, double dTime) {
    timeElapsed = glfwGetTime();
    glfwGetCursorPos(window, &mouseX, &mouseY);
    mousePos = glm::vec2(float(mouseX/screenWidth), float(mouseY/screenHeight));

    lights[0].pos = glm::vec3(-(mouseX - float(screenWidth)/2)/100, -(mouseY - float(screenHeight)/2)/100, 5);
    lights[1].dir = glm::vec3(-(mouseX - float(screenWidth)/2)/20, -(mouseY - float(screenHeight)/2)/20, -5);

    deltaTime = dTime;

    accumulatedPasses = accpasses;
    directOutputPass = directoutpass;

    mouseCameraMovement(window, screenWidth, screenHeight);

    updateUniforms(shader, window, screenWidth, screenHeight);
}

void updateUniforms(Shader shader, GLFWwindow* window, int scrWidth, int scrHeight) {
    shader.setFloat("u_time", timeElapsed);
    shader.setVec2("u_mousePos", mousePos);
    shader.setFloat("u_fov", fov);
    shader.setVec3("u_camPos", camPos);
    shader.setVec3("u_camLookAt", camLookAt);
    shader.setFloat("u_rayOriginToScreenDistance", rayOriginToScreenDistance);
    shader.setInt("u_raysPerPixel", raysPerPixel);
    shader.setInt("u_maxBounceLimit", maxBounceLimit);
    shader.setInt("u_shadowRays", shadowRays);
    shader.setInt("u_selectedObjectIndex", selectedObjectIndex);
    shader.setInt("u_accumulatedPasses", accumulatedPasses);
    shader.setBool("u_directOutputPass", directOutputPass);

    for (int i = 0; i < spheres.size(); i++) {
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].center").c_str(), spheres[i].center);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].radius").c_str(), spheres[i].radius);
        shader.setVec3(std::string("u_spheres[").append(std::to_string(i)).append("].mat.color").c_str(), spheres[i].mat.color);
        shader.setInt(std::string("u_spheres[").append(std::to_string(i)).append("].mat.type").c_str(), spheres[i].mat.type);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.fuzz").c_str(), spheres[i].mat.fuzz);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.refIndex").c_str(), spheres[i].mat.refIndex);
        shader.setFloat(std::string("u_spheres[").append(std::to_string(i)).append("].mat.emissionStrength").c_str(), spheres[i].mat.emissionStrength);
    }

    for (int i = 0; i < lights.size(); i++) {
        shader.setInt(std::string("u_lights[").append(std::to_string(i)).append("].type").c_str(), lights[i].type);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].pos").c_str(), lights[i].pos);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].dir").c_str(), lights[i].dir);
        shader.setVec3(std::string("u_lights[").append(std::to_string(i)).append("].color").c_str(), lights[i].color);
        shader.setFloat(std::string("u_lights[").append(std::to_string(i)).append("].maxIntensity").c_str(), lights[i].maxIntensity);
        shader.setBool(std::string("u_lights[").append(std::to_string(i)).append("].softShadows").c_str(), lights[i].softShadows);
    }
}

float xRot, yRot;
bool mouseCameraMovement(GLFWwindow* window, int screenWidth, int screenHeight) {
    Camera cam = getCam(fov, aspectRatio, camPos, camLookAt, upDir, rayOriginToScreenDistance);
    glm::vec3 prevCamPos = camPos;
    bool moved = false;
	
	double mouseX;
	double mouseY;
	glfwGetCursorPos(window, &mouseX, &mouseY);

	float xOffset = (float)(mouseX - screenWidth/2.0);
	float yOffset = (float)(mouseY - screenHeight/2.0);


    xRot = xOffset/screenWidth * mouseSensitivity;
	yRot = -yOffset/screenHeight * mouseSensitivity;

	if (yRot > 1.5707F)
		yRot = 1.5707F;
	if (yRot < -1.5707F)
		yRot = -1.5707F;

    glm::vec2 camYZ = glm::vec2(camPos.y, camPos.z);
    camYZ = rot2d(yRot) * camYZ;
    glm::vec2 camXZ = glm::vec2(camPos.x, camYZ.y);
    camXZ = rot2d(xRot) * camXZ;
    camPos = glm::vec3(camXZ.x, camYZ.x, camXZ.y);

    if (movingCamToSelectedObject) {
        glm::vec3 finalPos;
        glm::vec3 finalLookAt;

        glm::vec3 dir;

        if (selectedObjectIndex >= 0) {
            finalLookAt = spheres[selectedObjectIndex].center;
            dir = glm::normalize(camPos - finalLookAt);
            finalPos = finalLookAt + dir * camToObjDist * spheres[selectedObjectIndex].radius;
        }
        else {
            finalPos = camPosFromSettings;
            finalLookAt = camLookAtFromSettings;
        }

        moveCamToSelectedSphere(finalPos, finalLookAt, camMoveSpeed);
    } else if (selectedObjectIndex == -1 && !movingCamToSelectedObject && (camPos != camPosFromSettings || camLookAt != camLookAtFromSettings)) {
        moveCamToSelectedSphere(camPosFromSettings, camLookAtFromSettings, camMoveSpeed);
    }

    moved = glm::length(prevCamPos - camPos) > 0.01;
    return moved;
}

void moveCamToSelectedSphere(glm::vec3 newPos, glm::vec3 newLookAt, float speed) {
    camPos = lerpVec(camPos, newPos, speed * deltaTime);
    camLookAt = lerpVec(camLookAt, newLookAt, speed * deltaTime);
    if (glm::length(camLookAt - newLookAt) < 0.0001 && glm::length(camPos - newPos) < 0.0001)
        movingCamToSelectedObject = false;
}

glm::vec3 lerpVec(glm::vec3 a, glm::vec3 b, float t) {
    return a + (b-a) * t;
}

Ray getRayToScreen(Camera cam, float u, float v) {
    Ray ray;
    ray.origin = cam.position;
    ray.direction = cam.llc + cam.horizontal * u + cam.vertical * v - cam.position;
    return ray;
}

bool sphereIntersection(Sphere sphere, Ray ray, float* dist) {
    bool hit = true;
    glm::vec3 oc = ray.origin - sphere.center;
    float a = glm::dot(ray.direction, ray.direction);
    float b = 2.0f * glm::dot(ray.direction, oc);
    float c = glm::dot(oc, oc) - sphere.radius*sphere.radius;

    float disc = b*b - 4*a*c;

    if (disc < 0.0f)
        return false;

    *dist = (-b - std::sqrt(disc)) / (2.0f * a);
    if (*dist <= 0.001 || *dist >= INFINITY) {
        *dist = (-b + std::sqrt(disc)) / (2.0f * a);
        //std::cout << glm::to_string(ray.direction) << std::endl;
        if (*dist <= 0.001 || *dist >= INFINITY) 
            return false;
    }
    return true;
}

void sphereSelect(float mouseX, float mouseY, int screenWidth, int screenHeight) {
    float u = mouseX / screenWidth;
    float v = 1.0f - mouseY / screenHeight;
    Camera cam = getCam(fov, aspectRatio, camPos, camLookAt, upDir, rayOriginToScreenDistance);
    Ray ray = getRayToScreen(cam, u, v);

    float minDist = INFINITY;
    selectedObjectIndex = -1;
    for (int i = 0; i < spheres.size(); i++) {
        float dist;
        if (sphereIntersection(spheres[i], ray, &dist)) {
            if (dist < minDist) {
                minDist = dist;
                selectedObjectIndex = i;
                movingCamToSelectedObject = true;
            }
        }
    }
}

Sphere addNew() {
    Camera cam = getCam(fov, aspectRatio, camPos, camLookAt, upDir, rayOriginToScreenDistance);
    Ray ray = getRayToScreen(cam, 0.5f, 0.5f);
    glm::vec3 center = ray.origin + ray.direction * camToObjDist;

    Sphere sphere = Sphere(center, 1, Material(0, glm::vec3(1), 0, 0, 0));
    spheres.push_back(sphere);
    return sphere;
}
}
