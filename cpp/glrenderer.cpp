#include "glrenderer.h"
#include <iostream>
extern "C"
{
    GLFWwindow *_glfwCreateWindow(char *title, int width, int height, GLFWframebuffersizefun callback)
    {
        // Initialize GLFW
        glfwInit();

        // Set MSAA samples for antialiasing
        glfwWindowHint(GLFW_SAMPLES, 4);

        // Tell GLFW what version of OpenGL we are using
        // In this case we are using OpenGL 3.3 to be compatible with Mac
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);

        // Tell GLFW we are using the CORE profile
        // So that means we only have the modern functions
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

#ifdef __APPLE__
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
#endif

        GLFWwindow *window = glfwCreateWindow(width, height, title, NULL, NULL);
        if (window == nullptr)
        {
            std::cout << "Failed to create GLFW window" << std::endl;
            glfwTerminate();
        }
        glfwMakeContextCurrent(window);
        glfwSetFramebufferSizeCallback(window, callback);

        // glad: load all OpenGL function pointers
        // ---------------------------------------
        gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);
        if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress))
        {
            std::cout << "Failed to initialize GLAD" << std::endl;
            return nullptr;
        }

        // Enable MSAA (glEnable must come AFTER context is current and GLAD is loaded)
        glEnable(GL_MULTISAMPLE);
        
        glViewport(0, 0, width, height);
        return window;
    }

    void _glfwWindowHint(int hint, int value)
    {
        glfwWindowHint(hint, value);
    }

    void _glfwSetWindowUserPointer(GLFWwindow *window, void *pointer)
    {
        glfwSetWindowUserPointer(window, pointer);
    }

    void *_glfwGetWindowUserPointer(GLFWwindow *window)
    {
        return glfwGetWindowUserPointer(window);
    }

    bool _glfwWindowShouldClose(GLFWwindow *window)
    {
        return glfwWindowShouldClose(window);
    }

    void _glfwTerminate()
    {
        glfwTerminate();
    }

    void _glfwSwapBuffers(GLFWwindow *window)
    {
        // Swap the back buffer with the front buffer
        glfwSwapBuffers(window);
    }

    void _glfwPollEvents()
    {
        // Take care of all GLFW events
        glfwPollEvents();
    }

    double _glfwGetTime()
    {
        return glfwGetTime();
    }

    void _glfwSetScrollCallback(GLFWwindow *window, GLFWscrollfun callback)
    {
        glfwSetScrollCallback(window, callback);
    }

    void _glfwSetCursorPosCallback(GLFWwindow *window, GLFWcursorposfun callback)
    {
        glfwSetCursorPosCallback(window, callback);
    }

    void _glfwGetWindowSize(GLFWwindow *window, int *width, int *height)
    {
        glfwGetWindowSize(window, width, height);
    }

    void _glClearColor(GLfloat x, GLfloat y, GLfloat z, GLfloat a)
    {
        glClearColor(x, y, z, a);
        // Clean the back buffer and assign the new color to it
        glClear(GL_COLOR_BUFFER_BIT);
    }

    void _glViewPort(GLint x, GLint y, GLsizei width, GLsizei height)
    {
        glViewport(x, y, width, height);
    }

    void _glGetIntegerv(GLenum pname, GLint *data)
    {
        glGetIntegerv(pname, data);
    }

    GLuint _glGenBuffer()
    {
        unsigned int VBO;
        glGenBuffers(1, &VBO);
        return VBO;
    }

    void _glGenBuffers(GLsizei n, GLuint *buffers)
    {
        glGenBuffers(n, buffers);
    }

    void _glDeleteBuffer(GLuint buffer)
    {
        glDeleteBuffers(1, &buffer);
    }

    void _glBindBuffer(GLenum target, GLuint buffer)
    {
        glBindBuffer(target, buffer);
    }

    void _glBufferData(GLenum mode, GLsizeiptr size, const GLvoid *data, GLenum usage)
    {
        glBufferData(mode, size, data, usage);
    }

    void _glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const GLvoid *data)
    {
        glBufferSubData(target, offset, size, data);
    }

    GLuint _glGenVertexArray()
    {
        unsigned int VAO;
        glGenVertexArrays(1, &VAO);
        return VAO;
    }

    void _glBindVertexArray(GLuint array)
    {
        glBindVertexArray(array);
    }

    void _glVertexAttribPointer(GLuint index, GLint size, GLenum type, GLboolean normalized, GLsizei stride, GLsizei offset)
    {
        glVertexAttribPointer(index, size, type, normalized, stride, (void *)offset);
    }

    void _glEnableVertexAttribArray(GLuint index)
    {
        glEnableVertexAttribArray(index);
    }

    GLint _glGenTexture()
    {
        unsigned int texture;
        glGenTextures(1, &texture);
        return texture;
    }

    void _glActiveTexture(GLenum unit)
    {
        glActiveTexture(unit);
    }

    void _glBindTexture(GLenum target, GLuint texture)
    {
        glBindTexture(target, texture);
    }

    void _glTexParameteri(GLenum target, GLenum pname, GLint param)
    {
        glTexParameteri(target, pname, param);
    }

    void _glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void *data)
    {
        glTexImage2D(target, level, internalformat, width, height, border, format, type, data);
        GLenum error = glGetError();
        if (error == GL_INVALID_OPERATION)
        {
            printf("OpenGL error: %d\n", error);
        }
        else
        {
            std::cout << "glTextImage2D called successfully" << std::endl;
        }
    }

    void _glGenerateMipmap(GLenum target)
    {
        glGenerateMipmap(target);
    }

    GLuint _glCreateShader(GLenum shaderType)
    {
        return glCreateShader(shaderType);
    }

    void _glShaderSource(GLuint shader, GLchar *source)
    {
        glShaderSource(shader, 1, &source, NULL);
    }

    void _glCompileShader(GLuint shader)
    {
        glCompileShader(shader);
#ifndef NDEBUG
        int success;
        char infoLog[512];
        glGetShaderiv(shader, GL_COMPILE_STATUS, &success);
        if (!success)
        {
            glGetShaderInfoLog(shader, 512, NULL, infoLog);
            std::cout << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n"
                      << infoLog << std::endl;
        }
        else
        {
            std::cout << "shader " << shader << " compiled successfully" << std::endl;
        }
#endif
    }

    GLuint _glCreateProgram()
    {
        return glCreateProgram();
    }

    void _glAttachShader(GLuint program, GLuint shader)
    {
        glAttachShader(program, shader);
    }

    void _glLinkProgram(GLuint program)
    {
        glLinkProgram(program);
    }

    void _glUseProgram(GLuint program)
    {
        glUseProgram(program);
    }

    void _glDrawArrays(GLenum mode, GLint first, GLsizei count)
    {
        glDrawArrays(mode, first, count);
    }

    void _glDrawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instancecount)
    {
        glDrawArraysInstanced(mode, first, count, instancecount);
    }

    void _glVertexAttribDivisor(GLuint index, GLuint divisor)
    {
        glVertexAttribDivisor(index, divisor);
    }

    void _glDrawElements(GLenum mode, GLsizei count, GLenum type, GLuint offset)
    {
        glDrawElements(mode, count, type, (void *)(offset));
    }

    GLint _glGetUniformLocation(GLuint program, GLchar *name)
    {
        return glGetUniformLocation(program, name);
    }

    void _glUniform1f(GLint location, GLfloat v0)
    {
        glUniform1f(location, v0);
    }

    void _glUniform2f(GLint location, GLfloat v0, GLfloat v1)
    {
        glUniform2f(location, v0, v1);
    }

    void _glUniform3f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2)
    {
        glUniform3f(location, v0, v1, v2);
    }

    void _glUniform4f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3)
    {
        glUniform4f(location, v0, v1, v2, v3);
    }

    void _glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value)
    {
        glUniformMatrix4fv(location, count, transpose, value);
    }

    void _glPointSize(GLfloat size)
    {
        glPointSize(size);
    }

    void _glEnable(GLenum cap)
    {
        glEnable(cap);
    }

    void _glBlendFunc(GLenum sfactor, GLenum dfactor)
    {
        glBlendFunc(sfactor, dfactor);
    }

    // Keep FT_Library internal here (simple singleton for the app)
    namespace {
        FT_Library g_ft_lib = nullptr;
    }


    FT_Error _ftInitFreeType() {
        if (g_ft_lib) return 0; // already inited
        return FT_Init_FreeType(&g_ft_lib);
    }

    FT_Error _ftDoneFreeType() {
        if (!g_ft_lib) return 0;
        FT_Error err = FT_Done_FreeType(g_ft_lib);
        g_ft_lib = nullptr;
        return err;
    }

    FT_Error _ftNewFace(const char *filePath, FT_Long faceIndex, FT_Face *aface_) {
        if (!g_ft_lib) return 1; // not inited; choose a non-zero error code
        return FT_New_Face(g_ft_lib, filePath, faceIndex, aface_);
    }

    FT_Error _ftDoneFace(FT_Face aface) {
        if (!aface) return 0;
        return FT_Done_Face(aface);
    }

    FT_Error _ftLoadChar(FT_Face face, FT_ULong char_code, FT_Int32 load_flags)
    {
        // Ensure we render immediately (LearnOpenGL expects a rasterized A8 bitmap)
        FT_Error err = FT_Load_Char(face, char_code, load_flags | FT_LOAD_RENDER);
        if (err != 0) {
            std::cerr << "FT_Load_Char failed (" << err << ") for codepoint " << char_code << "\n";
            return err;
        }

        FT_GlyphSlot slot = face->glyph;
        const FT_Bitmap& bmp = slot->bitmap;

        // Logs (guard family_name)
        const char* fam = face->family_name ? face->family_name : "(unknown)";
        std::cout << "face family: " << fam << "\n";
        std::cout << "glyph bitmap: " << bmp.width << "x" << bmp.rows
                  << " pitch=" << bmp.pitch
                  << " left=" << slot->bitmap_left
                  << " top=" << slot->bitmap_top
                  << " adv=" << (slot->advance.x / 64.0f)
                  << " mode=" << (int)bmp.pixel_mode
                  << "\n";

        return 0;
    }

    FT_Error _ftSetPixelSizes(FT_Face face, FT_UInt pixel_width, FT_UInt pixel_height)
    {
        return FT_Set_Pixel_Sizes(face, pixel_width, pixel_height);
    };

    FT_Error _getGlyphDimensions(FT_Face face, GlyphDimensions *dimensions) {
        if (!face || !dimensions) return 1;
        FT_GlyphSlot slot = face->glyph;
        if (!slot) return 1;

        const FT_Bitmap &bmp = slot->bitmap;
        dimensions->width     = static_cast<int>(bmp.width);
        dimensions->height    = static_cast<int>(bmp.rows);
        dimensions->left      = static_cast<int>(slot->bitmap_left);
        dimensions->top       = static_cast<int>(slot->bitmap_top);
        dimensions->advance_x = static_cast<int>(slot->advance.x); // 26.6 fixed
        return 0;
    }

    FT_Error _getBitmap(FT_Face face, FT_Bitmap **bitmap) {
        if (!face || !bitmap) return 1;
        FT_GlyphSlot slot = face->glyph;
        if (!slot) return 1;
        *bitmap = &slot->bitmap; // Borrowed; invalid after next _ftLoadChar
        return 0;
    }
}
