#include <iostream>
#include "gui.h"
#include <glad/glad.h>
#include "imgui/imgui.h"
#include <string>
#include <chrono>
#include <filesystem>

const unsigned int SCR_WIDTH = 1280;
const unsigned int SCR_HEIGHT = 720;

unsigned int screenWidth = SCR_WIDTH;
unsigned int screenHeight = SCR_HEIGHT;

std::filesystem::path srcDir = std::filesystem::current_path().parent_path().concat("/src");
const std::string VERT_PATH = (std::string)srcDir + "/vertex.glsl";
const std::string FRAG_PATH = (std::string)srcDir + "/fragment.glsl";

double deltaTime = 0.0f;

GLuint screenTexture;

void frameBufferSizeCallback(GLFWwindow* window, int width, int height) {
    glViewport(0, 0, width, height);
    screenWidth = width;
    screenHeight = height;

	glBindTexture(GL_TEXTURE_2D, screenTexture);
	glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA32F, screenWidth, screenHeight, 0, GL_RGBA, GL_FLOAT, NULL);

    Scene::refresh = true;
}

void mouseButtonCallback(GLFWwindow* window, int button, int action, int mods) {
    if (button == GLFW_MOUSE_BUTTON_1 && action == GLFW_PRESS) {
        if (!ImGui::GetIO().WantCaptureMouse) {
            double mouseX, mouseY;
            glfwGetCursorPos(window, &mouseX, &mouseY);

            Scene::sphereSelect(mouseX, mouseY, screenWidth, screenHeight);
        }
    }
}

int main() {
    //Window Initialization
	glfwInit();
	glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
	glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
	glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
	GLFWwindow* window = glfwCreateWindow(SCR_WIDTH, SCR_HEIGHT, "GraphicsEngine", NULL, NULL);
	if (window == NULL)
	{
		std::cout << "Failed to create GLFW window" << std::endl;
		glfwTerminate();
		return -1;
	}
    glfwSetFramebufferSizeCallback(window, frameBufferSizeCallback);
    glfwSetMouseButtonCallback(window, mouseButtonCallback);

    double mouseX, mouseY;
	glfwMakeContextCurrent(window);
	gladLoadGL();
	glViewport(0, 0, SCR_WIDTH, SCR_HEIGHT);

    //Vertex Data and Vertex buffer Initialization
    GLfloat vertices[] =
	{
        -1.0f, -1.0f, 0.0f, 
        1.0f, -1.0f, 0.0f, 
        1.0f, 1.0f, 0.0f, 
        -1.0f, -1.0f, 0.0f, 
        1.0f, 1.0f, 0.0f, 
        -1.0f, 1.0f, 0.0f, 
	};
	GLuint VAO, VBO;
	glGenVertexArrays(1, &VAO);
	glGenBuffers(1, &VBO);
	glBindVertexArray(VAO);
	glBindBuffer(GL_ARRAY_BUFFER, VBO);
	glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
	glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
	glEnableVertexAttribArray(0);
	glBindBuffer(GL_ARRAY_BUFFER, 0);
	glBindVertexArray(0);

    GLuint FBO;
    glGenTextures(1, &screenTexture);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, screenTexture);
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA32F, SCR_WIDTH, SCR_HEIGHT, 0, GL_RGBA, GL_FLOAT, NULL);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glGenFramebuffers(1, &FBO);
	glBindFramebuffer(GL_FRAMEBUFFER, FBO);
	glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, screenTexture, 0);
	if (glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) {
		std::cout << "ERROR: Framebuffer is not complete!" << std::endl;
		return -1;
	}


    std::chrono::steady_clock::time_point lastTime;
    int frameCount = 0;
    float fps = 0.0f;

    lastTime = std::chrono::steady_clock::now();
    

    //Shader Initialization and main code loop
    Shader shader(VERT_PATH, FRAG_PATH);
    shader.use();
	shader.setInt("u_screenTexture", 0);
    GUI::initGUI(window);
    Scene::initialize(shader, screenWidth, screenHeight, screenTexture);

    int accumulatedPasses = 0;
    bool directOutputPass;
	while (!glfwWindowShouldClose(window))
	{
		glClearColor(0.0f, 0.0f, 0.0f, 1.0f);
		glClear(GL_COLOR_BUFFER_BIT);

        auto currentTime = std::chrono::steady_clock::now();
        float elapsedTime = std::chrono::duration_cast<std::chrono::duration<float>>(currentTime - lastTime).count();
        deltaTime = elapsedTime;
        lastTime = currentTime;
        frameCount++;
        fps = frameCount / elapsedTime;
        frameCount = 0;

        Scene::refresh = Scene::mouseCameraMovement(window, screenWidth, screenHeight);

		glBindVertexArray(VAO);
        //glDrawArrays(GL_TRIANGLES, 0, 6);

        glBindFramebuffer(GL_FRAMEBUFFER, FBO);
		//glDrawArrays(GL_TRIANGLES, 0, 6);
		accumulatedPasses += 1;
		glBindFramebuffer(GL_FRAMEBUFFER, 0);
		glDrawArrays(GL_TRIANGLES, 0, 6);

        directOutputPass = accumulatedPasses <= 1;
        if (accumulatedPasses > 100) accumulatedPasses = 100;
        if (Scene::refresh) accumulatedPasses = 0;

        GUI::renderGUI();

        Scene::update(shader, window, screenWidth, screenHeight, directOutputPass, accumulatedPasses, deltaTime);

		glfwSwapBuffers(window);
		glfwPollEvents();
	}

    GUI::deactivateGUI();

    //Deletion of vertex data and deactivation of shader program
	glDeleteVertexArrays(1, &VAO);
	glDeleteBuffers(1, &VBO);
    glDeleteFramebuffers(1, &FBO);
	glDeleteTextures(1, &screenTexture);

    shader.deactivate();

	glfwDestroyWindow(window);
	glfwTerminate();

	return 0;
}
