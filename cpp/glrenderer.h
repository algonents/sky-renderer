#include <iostream>
#include <glad/glad.h>
#include <GLFW/glfw3.h>

extern "C"
{
    // GFLW
    GLFWwindow *_glfwCreateWindow(char *title, int width, int height, GLFWframebuffersizefun callback);
    void _glfwSetWindowUserPointer(GLFWwindow *window, void *pointer);
    void *_glfwGetWindowUserPointer(GLFWwindow *window);

    bool _glfwWindowShouldClose(GLFWwindow *window);
    void _glfwTerminate();

    void _glfwSwapBuffers(GLFWwindow *window);
    void _glfwPollEvents();

    double _glfwGetTime();

    void _glfwSetScrollCallback(GLFWwindow *window, GLFWscrollfun callback);
    void _glfwSetCursorPosCallback(GLFWwindow *window, GLFWcursorposfun callback);

    void _glfwGetWindowSize(GLFWwindow *window, int *width, int *height);

    // GL
    void _glClearColor(GLfloat x, GLfloat y, GLfloat z, GLfloat a);
    void _glViewPort(GLint x, GLint y, GLsizei width, GLsizei height);
    void _glGetIntegerv(GLenum pname, GLint *data);

    GLuint _glCreateShader(GLenum shaderType);
    void _glShaderSource(GLuint shader, GLchar *source);
    void _glCompileShader(GLuint shader);
    GLuint _glCreateProgram();
    void _glAttachShader(GLuint program, GLuint shader);
    void _glLinkProgram(GLuint program);
    void _glUseProgram(GLuint program);
    GLuint _glGenBuffer();
    void _glGenBuffers(GLsizei n, GLuint *buffers);
    void _glBindBuffer(GLenum target, GLuint buffer);
    void _glBufferData(GLenum mode, GLsizeiptr size, const GLvoid *data, GLenum usage);
    void _glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const GLvoid *data);
    void _glDeleteBuffer(GLuint buffer);

    void _glActiveTexture(GLenum unit);
    GLint _glGenTexture();
    void _glBindTexture(GLenum target, GLuint texture);
    void _glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void *data);
    void _glTexParameteri(GLenum target, GLenum pname, GLint param);
    void _glGenerateMipmap(GLenum target);

    GLuint _glGenVertexArray();
    void _glBindVertexArray(GLuint VAO);
    void _glVertexAttribPointer(GLuint index, GLint size, GLenum type, GLboolean normalized, GLsizei stride, GLsizei offset);
    void _glEnableVertexAttribArray(GLuint index);

    void _glDrawArrays(GLenum mode, GLint first, GLsizei count);
    void _glDrawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instancecount);
    void _glVertexAttribDivisor(GLuint index, GLuint divisor);

    void _glDrawElements(GLenum mode, GLsizei count, GLenum type, GLuint offset);
    GLint _glGetUniformLocation(GLuint program, GLchar *name);
    void _glUniform1f(GLint location, GLfloat v0);
    void _glUniform2f(GLint location, GLfloat v0, GLfloat v1);
    void _glUniform3f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2);
    void _glUniform4f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3);
    void _glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
    void _glPointSize(GLfloat size);
    void _glEnable(GLenum cap);
    void _glBlendFunc(GLenum sfactor, GLenum dfactor);
};