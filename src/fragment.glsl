#version 460 core

#define PI 3.14159265359
#define EPSILON 0.0001
#define MAX_OBJECT_COUNT 10
#define MAX_LIGHT_COUNT 5
#define LIGHT_RADIUS 0.2
#define OUTLINE_COLOR vec3(0.1f, 1.0f, 0.1f)
#define OUTLINE_WIDTH 0.04
#define MAX_BVH_NODES 32
#define BVH_LEAF -1

in vec2 uv;

out vec4 FragColor;

uniform float INFINITY;

uniform float u_time;

uniform vec2 u_screenPixels;
uniform float u_aspectRatio;
uniform vec2 u_mousePos;
uniform float u_fov;

uniform vec3 u_camPos;
uniform vec3 u_camLookAt;
uniform vec3 u_upDir;
uniform float u_rayOriginToScreenDistance;
uniform mat4 u_rotationMatrix;

uniform int u_shadowRays;
uniform int u_raysPerPixel;
uniform int u_maxBounceLimit;

uniform int u_selectedObjectIndex;

uniform sampler2D u_screenTexture;
uniform int u_accumulatedPasses;
uniform bool u_directOutputPass;
uniform int u_framePasses;



float deg2rad(float deg) {
    float rad = deg * PI / 180.0f;
    return rad;
}

struct BVHNode {
    vec3 boundsMin;
    vec3 boundsMax;
    int left;
    int right;
    int sphereIndex; // >=0 means leaf
};
uniform BVHNode u_bvhNodes[MAX_BVH_NODES];
uniform int u_bvhRoot;
uniform int u_bvhNodeCount;

struct Camera {
    vec3 position;
    vec3 llc;
    vec3 horizontal;
    vec3 vertical;
    vec3 backward;
    vec3 right;
    vec3 up;
};

Camera getCamFromLookAt(float fov, float aspectRatio, vec3 lookFrom, vec3 lookAt, vec3 vup, float rayOriginToScreenDistance) {
    float viewportHeight = rayOriginToScreenDistance * tan(deg2rad(fov)/2.0f)* 2.0f;
    float viewportWidth = viewportHeight * aspectRatio;

    Camera cam;
    cam.backward = normalize(lookFrom - lookAt);
    cam.right = normalize(cross(vup, cam.backward));
    cam.up = cross(cam.backward, cam.right);

    cam.position = lookFrom;
    cam.horizontal = cam.right * viewportWidth;
    cam.vertical = cam.up * viewportHeight;
    cam.llc = cam.position - cam.horizontal / 2.0f - cam.vertical / 2.0f - cam.backward;
    return cam;
}

struct Ray {
    vec3 origin;
    vec3 direction;
};

Ray getRay(vec3 origin, vec3 direction) {
    Ray ray;
    ray.origin = origin;
    ray.direction = direction;
    return ray;
}

Ray getRayFromCam(Camera cam, float u, float v) {
    Ray ray;
    ray.origin = cam.position;
    ray.direction = cam.llc + cam.horizontal * u + cam.vertical * v - cam.position;
    return ray;
}

struct Material {
    int type; //0 = lambertian, 1 = metal, 2 = dielectric, 3 = emissive
    vec3 color; //for type = 0,1,3
    float fuzz; //for type = 1
    float refIndex; //for type = 2
    float emissionStrength; //for type = 3
};

struct HitInfo {
    bool hit;
    float distance;
    vec3 hitPoint;
    vec3 normal;
    Material mat;
};

HitInfo initializeHitInfo() {
    HitInfo hitInfo = {false, INFINITY, vec3(0), vec3(0), {0, vec3(0), 0, 0, 0}};
    return hitInfo;
}

struct Sphere {
    vec3 center;
    float radius;
    Material mat;
};

struct Triangle {
    vec3 pointA;
    vec3 pointB;
    vec3 pointC;
    Material mat;
};

struct Light {
    int type; //0 = point, 1 = directional
    vec3 pos; //for type = 0
    vec3 dir; //for type = 1
    vec3 color;
    float maxIntensity; //inverse square law for type = 0, constant for type = 1
    bool softShadows;
};

uniform Sphere u_spheres[MAX_OBJECT_COUNT];
uniform Triangle u_triangles[MAX_OBJECT_COUNT];
uniform Light u_lights[MAX_LIGHT_COUNT];

uint nextRandom(inout uint state)
{
    state = state * 747796405 + 2891336453;
    uint result = ((state >> ((state >> 28) + 4)) ^ state) * 277803737;
    result = (result >> 22) ^ result;
    return result;
}

float randomValue(inout uint state)
{
    return nextRandom(state) / 4294967295.0; // 2^32 - 1
}

float random(uint state) {
    return randomValue(state);
}

float randomValueNormalDistribution(inout uint state)
{
    float theta = 2 * 3.1415926 * randomValue(state);
    float rho = sqrt(-2 * log(randomValue(state)));
    return rho * cos(theta);
}

vec3 randomPointInSphere(inout uint state)
{
    float x = randomValueNormalDistribution(state);
    float y = randomValueNormalDistribution(state);
    float z = randomValueNormalDistribution(state);
    return normalize(vec3(x, y, z));
}

vec2 randomPointInCircle(inout uint rngState)
{
    float angle = randomValue(rngState) * 2 * PI;
    vec2 pointOnCircle = vec2(cos(angle), sin(angle));
    return pointOnCircle * sqrt(randomValue(rngState));
}

float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
}

mat2 rot2D (float angle) {
    float s = sin(angle);
    float c = cos(angle);

    return mat2(c, -s, s, c);
}

float lerp(float a, float b, float t) {
    return a + (b-a) * t;
}

vec3 lerpVec(vec3 a, vec3 b, float t) {
    return a + (b-a) * t;
}


float scalarTriple(vec3 a, vec3 b, vec3 c) {
    return dot(cross(a, b), c);
}

float schlick(float cos, float refIndex) {
    float r0 = (1.0f - refIndex) / (1.0f + refIndex);
    r0 *= r0;
    return r0 + (1.0f - r0) * pow(1.0f - cos, 5);
}

HitInfo sphereIntersection(Ray ray, Sphere sphere) {
    HitInfo hitInfo = initializeHitInfo();

    /*
        * if (P-C).(P-C)=r^2 where P=A+tB vector and C=center
        * so expanding,
        * t^2(B.B) + 2tB.(A-C) + (A-C).(A-C) - r^2 = 0
        * so if D > 0, then the ray has hit the sphere    
    */
    vec3 oc = ray.origin - sphere.center;
    float a = dot(ray.direction, ray.direction);
    float b = 2.0f * dot(ray.direction, oc);
    float c = dot(oc, oc) - sphere.radius*sphere.radius;

    float disc = b*b - 4*a*c;

    if (disc < 0.0f) 
        return hitInfo;

    //float dist = ((-b - sqrt(disc)) / (2.0f * a)) > 0.0f ? ((-b - sqrt(disc)) / (2.0f * a)) : ((-b + sqrt(disc)) / (2.0f * a));
    float dist = (-b - sqrt(disc)) / (2.0f * a);
    if (dist <= 0.001 || dist >= INFINITY) {
        dist = (-b + sqrt(disc)) / (2.0f * a);
        if (dist <= 0.001 || dist >= INFINITY) 
            return hitInfo;
    }

    hitInfo.hit = true;
    hitInfo.distance = dist;
    hitInfo.hitPoint = ray.origin + ray.direction * dist;
    hitInfo.normal = normalize(hitInfo.hitPoint - sphere.center);
    hitInfo.mat = sphere.mat;
    return hitInfo;
}

HitInfo triangleIntersection(Ray ray, Triangle triangle) {
    vec3 AB = triangle.pointB - triangle.pointA;
    vec3 AC = triangle.pointC - triangle.pointA;
    vec3 normal = cross(AB, AC);
    vec3 ao = ray.origin - triangle.pointA;
    vec3 dao = cross(ao, ray.direction);

    float det = -dot(ray.direction, normal);

    float dist = dot(ao, normal) / det;
    float u = dot(AC, dao) / det;
    float v = -dot(AB, dao) / det;
    float w = 1 - u - v;
    
    HitInfo hitInfo = initializeHitInfo();
    hitInfo.hit = det >= EPSILON && dist >= 0 && u >= 0 && v >= 0 && w >= 0;
    hitInfo.hitPoint = ray.origin + ray.direction * dist;
    hitInfo.normal = normalize(normal);
    hitInfo.distance = dist;
    hitInfo.mat = triangle.mat;

    return hitInfo;
}

void sphereBoundingBox(Sphere sphere, inout vec3 boundsMin, inout vec3 boundsMax) {
    vec3 center = sphere.center;
    vec3 extents = vec3(sphere.radius, sphere.radius, sphere.radius);
    boundsMin = center - extents;
    boundsMax = center + extents;
}

bool rayHitBoundingBox(Ray ray, vec3 boundsMin, vec3 boundsMax) {
    vec3 invDir = 1 / ray.direction;
    vec3 tMin = (boundsMin - ray.origin) * invDir;
    vec3 tMax = (boundsMax - ray.origin) * invDir;
    vec3 t1 = min(tMin, tMax);
    vec3 t2 = max(tMin, tMax);
    float tNear = max(max(t1.x, t1.y), t1.z);
    float tFar = min(min(t2.x, t2.y), t2.z);
    return tNear <= tFar;
}

vec3 GetEnvironmentLight(vec3 dir)
{
    //if (UseSky == 0) return 0;
    const vec3 GroundColour = vec3(0.35f, 0.3f, 0.35f);
    const vec3 SkyColourHorizon = vec3(1, 1, 1);
    const vec3 SkyColourZenith = vec3(0.08f, 0.37f, 0.73f); 

    float skyGradientT = pow(smoothstep(0.0f, 0.4f, dir.y), 0.35f);
    float groundToSkyT = smoothstep(-0.01f, 0.0f, dir.y);
    vec3 skyGradient = lerpVec(SkyColourHorizon, SkyColourZenith, skyGradientT);
    //float sun = pow(max(0, dot(dir, _WorldSpaceLightPos0.xyz)), SunFocus) * SunIntensity;
    // Combine ground, sky, and sun
    vec3 composite = lerpVec(GroundColour, skyGradient, groundToSkyT);// + sun * SunColour * (groundToSkyT >= 1);
    return composite / 1.3f;
}

HitInfo traverseBVH(Ray ray) {
    HitInfo closest = initializeHitInfo();

    int stack[32];
    int stackPtr = 0;
    stack[stackPtr++] = u_bvhRoot;

    while (stackPtr > 0) {
        int nodeIndex = stack[--stackPtr];
        BVHNode node = u_bvhNodes[nodeIndex];

        if (!rayHitBoundingBox(ray, node.boundsMin, node.boundsMax))
            continue;

        if (node.sphereIndex >= 0) {
            Sphere sphere = u_spheres[node.sphereIndex];
            HitInfo hit = sphereIntersection(ray, sphere);
            if (hit.hit && hit.distance < closest.distance)
                closest = hit;
        } else {
            if (node.left >= 0)  stack[stackPtr++] = node.left;
            if (node.right >= 0) stack[stackPtr++] = node.right;
        }
    }

    return closest;
}

HitInfo rayCollision(Ray ray) {
    /*HitInfo closest = initializeHitInfo(); 

    for (int i = 0; i < u_spheres.length(); i++) {
        Sphere sphere = u_spheres[i];
        //vec3 boundsMin, boundsMax;
        //sphereBoundingBox(sphere, boundsMin, boundsMax);
        //if (!rayHitBoundingBox(ray, boundsMin, boundsMax)) continue;

        HitInfo hitInfo = sphereIntersection(ray, sphere);

        if (hitInfo.hit && hitInfo.distance < closest.distance) {
            closest = hitInfo;
        }
    }

    for (int i = 0; i < u_triangles.length(); i++) {
        Triangle triangle = u_triangles[i];
        HitInfo hitInfo = triangleIntersection(ray, triangle);

        if (hitInfo.hit && hitInfo.distance < closest.distance) {
            closest = hitInfo;
        }
    }

    return closest;*/

    return traverseBVH(ray);
}


float calculatePointLightReach(Light light, float distance) {
    float cutoffIntensity = 0.01;
    
    //float intensity = light.maxIntensity / pow((distance + sqrt(light.maxIntensity)), 2);
    float reach = sqrt(light.maxIntensity / cutoffIntensity) - sqrt(light.maxIntensity);

    return reach;
}

vec3 computeDirectIllumination(Ray ray, HitInfo hitInfo, uint seed) {
    vec3 illumination = vec3(0);

    for (int i = 0; i < u_lights.length(); i++) {
        Light light = u_lights[i];
        
        switch(light.type) {
            case(0): {
                float lightDistance = length(light.pos - hitInfo.hitPoint);
                if (lightDistance > calculatePointLightReach(light, lightDistance)) continue;
                float diffuse = clamp(dot(hitInfo.normal, normalize(light.pos - hitInfo.hitPoint)), 0, 1);

                if (light.softShadows) {
                    if (diffuse > EPSILON) {
                        int shadowRays = int(u_shadowRays * LIGHT_RADIUS * LIGHT_RADIUS / (lightDistance * lightDistance) + 1);
                        int shadowRayHits = 0;

                        for (int i = 0; i < shadowRays; i++) {
                            vec3 lightSurfacePoint = light.pos + normalize(vec3(random(i+seed+1), random(i+seed+2), random(i+seed+3))) * LIGHT_RADIUS;
                            vec3 lightDir = normalize(lightSurfacePoint - hitInfo.hitPoint);
                            vec3 shadowRayOrigin = hitInfo.hitPoint + lightDir * EPSILON;
                            float originToLightDistance = length(lightSurfacePoint - shadowRayOrigin);
                            Ray shadowRay = getRay(shadowRayOrigin, lightDir);
                            HitInfo shadowRayHit = rayCollision(shadowRay);
                            if (shadowRayHit.distance < originToLightDistance) {
                                shadowRayHits++;
                            }
                        }
                        float att = lightDistance * lightDistance;
                        illumination += light.color * light.maxIntensity * diffuse * hitInfo.mat.color * (1.0-float(shadowRayHits)/float(shadowRays)) / att;
                    }
                } else {
                    float shadow = 0;
                    if (diffuse > EPSILON) {
                        vec3 lightDir = normalize(light.pos - hitInfo.hitPoint);
                        vec3 shadowRayOrigin = hitInfo.hitPoint + lightDir * EPSILON;
                        float originToLightDistance = length(light.pos - shadowRayOrigin);
                        Ray shadowRay = getRay(shadowRayOrigin, lightDir);
                        HitInfo shadowRayHit = rayCollision(shadowRay);
                        if (shadowRayHit.distance < originToLightDistance) shadow = 1;

                        float att = lightDistance * lightDistance;
                        illumination += light.color * light.maxIntensity * diffuse * hitInfo.mat.color * (1.0-shadow) / att;
                    }
                }
                break;
            }
            case(1): {
                float diffuse = clamp(dot(hitInfo.normal, normalize(light.dir)), 0, 1);
                float shadow = 0;
                if (diffuse > EPSILON || hitInfo.mat.fuzz < 1.0f) {
                    vec3 shadowRayOrigin = hitInfo.hitPoint + light.dir * EPSILON;
                    Ray shadowRay = getRay(shadowRayOrigin, light.dir);
                    HitInfo shadowRayHit = rayCollision(shadowRay);
                    if (shadowRayHit.hit) shadow = 1;
                    illumination += light.color * light.maxIntensity * diffuse * (1.0f - shadow) * hitInfo.mat.color;
                }
                break;
            }
        }
    }

    return illumination;
}

vec3 computeRayColor(Ray ray, inout uint seed) {
    vec3 incomingLight = vec3(0);
    vec3 rayColor = vec3(1);

    for (int i = 0; i <= u_maxBounceLimit; i++) {
        HitInfo hitInfo = rayCollision(ray);

        if (hitInfo.hit) {
            ray.origin = hitInfo.hitPoint;
            vec3 emittedLight = vec3(0);
            switch (hitInfo.mat.type) {
                case (0):
                    ray.direction = hitInfo.normal + randomPointInSphere(seed);
                    break;
                case (1):
                    float f = clamp(hitInfo.mat.fuzz, 0, 1);
                    vec3 reflected = reflect(normalize(ray.direction), hitInfo.normal);
                    ray.direction = reflected + randomPointInSphere(seed) * f;
                    break;
                case (2):
                    vec3 outwardNormal = vec3(0);
                    float ni_nt = 0;
                    
                    if (dot(ray.direction, hitInfo.normal) < 0) {
                        outwardNormal = hitInfo.normal;
                        ni_nt = 1.0f / hitInfo.mat.refIndex;
                    } else {
                        outwardNormal = -hitInfo.normal;
                        ni_nt = hitInfo.mat.refIndex;
                    }

                    float cos = min(dot(-normalize(ray.direction), hitInfo.normal), 1.0f);
                    float sin = sqrt(1.0f - cos*cos);
                    bool canRefract = ni_nt * sin < 1;

                    ray.direction = canRefract || schlick(cos, hitInfo.mat.refIndex) < randomValue(seed) ? refract(normalize(ray.direction), outwardNormal, ni_nt) : reflect(normalize(ray.direction), outwardNormal);
                    break;
                case (3):
                    ray.direction = hitInfo.normal + randomPointInSphere(seed);
                    emittedLight = hitInfo.mat.color * hitInfo.mat.emissionStrength;
                    break;
            }

            rayColor *= hitInfo.mat.color;
            incomingLight += emittedLight * rayColor;

            incomingLight += rayColor * computeDirectIllumination(ray, hitInfo, seed);
        } else {
            incomingLight += GetEnvironmentLight(ray.direction) * rayColor;
            break;
        }
    }

    return incomingLight;
}

void main() {
    vec2 normalized_uv = uv / 2.0f + 0.5f; 
    vec2 n = vec2(uv.x * u_aspectRatio, uv.y);
    vec2 m = vec2(u_mousePos.x * u_aspectRatio, u_mousePos.y); 

    // Initialization
    Camera cam = getCamFromLookAt(u_fov, u_aspectRatio, u_camPos, u_camLookAt, u_upDir, u_rayOriginToScreenDistance);

    //Ray ray = getRayFromCam(cam, normalized_uv.x, normalized_uv.y);
    //if (u_directOutputPass) {

    if (true) {
        /*ray.origin.yz *= rot2D(-m.y);
        ray.direction.yz *= rot2D(-m.y);
        ray.origin.xz *= rot2D(m.x);
        ray.direction.xz *= rot2D(m.x);*/
        
        //Taking current pixel as seed for RNG
        vec2 pixelCoord = uv * u_screenPixels;
        uint pixelIndex = int(pixelCoord.y) * int(u_screenPixels.x) + int(pixelCoord.x);
        uint seed = pixelIndex;

        // Coloring
        vec3 totalIncomingLight = vec3(0);
        for (int i = 0; i < u_raysPerPixel; i++) {
            float u = ((normalized_uv.x * u_screenPixels.x) + (fract(random(pixelIndex + i)) - 0.5f)) / u_screenPixels.x;
            float v = ((normalized_uv.y * u_screenPixels.y) + (fract(random(pixelIndex + i+1)) - 0.5f)) / u_screenPixels.y;
            Ray ray = getRayFromCam(cam, u, v);
            /*ray.origin.yz *= rot2D(m.y);
            ray.direction.yz *= rot2D(m.y);
            ray.origin.xz *= rot2D(m.x);
            ray.direction.xz *= rot2D(m.x);*/
            totalIncomingLight += computeRayColor(ray, seed);

            if (u_selectedObjectIndex >= 0) {
                Sphere sphere = u_spheres[u_selectedObjectIndex];
                Sphere outlineSphere = {sphere.center, sphere.radius + OUTLINE_WIDTH, sphere.mat};

                HitInfo outline = sphereIntersection(ray, outlineSphere);
                if (outline.hit) {
                    HitInfo inner = sphereIntersection(ray, sphere);
                    if (!inner.hit) {
                        FragColor = vec4(OUTLINE_COLOR, 1);
                        return;
                    }
                }
            }
        }
        vec3 pixelColor = totalIncomingLight / u_raysPerPixel;
        
        FragColor = vec4(pixelColor, 1); 
        
    } else {
        if (u_accumulatedPasses > 0) {
            FragColor += texture(u_screenTexture, normalized_uv);
            float d = float(u_accumulatedPasses);
            FragColor.x /= d;
            FragColor.y /= d;
            FragColor.z /= d;
        }
    }
}
