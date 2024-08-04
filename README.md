# Raytracer (C++, OpenGL and GLSL)
This is a re-write of my old [raytracer](https://github.com/thesynthax/raytracer-legacy), which used the CPU for rendering scenes and did not work in real-time.
This new raytracer works in real-time by using a fragment shader to compute raytracing in parallel using the GPU. The GUI is implemented using Dear ImGui in C++, which provides options to add, remove, and change settings of the current objects that are being rendered.

### To build and run:
- `git clone https://github.com//thesynthax/raytracer`
- Build and run with `./run`

### Goals accomplished:
- Successfully Setup an OpenGL environment (the hardest of them all)
- Triangle rendered (even harder, not even joking)
- Shaders setup, experimented with shaders and raymarching
- Sphere rendered without lighting
- Basic Lambertian lighting
- Diffuse, Metallic, Fuzzy, Dielectric and Emissive Materials
- Point and Directional Light added with soft and hard shadows
- Anti-aliasing
- GUI
- Interaction using mouse
- Camera Movement

### To be accomplished:
- Progressive Rendering
- Bloom
- Mesh import and rendering
- Optimizations using BVH, LoDs
