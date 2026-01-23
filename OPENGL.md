# OpenGL 2D Rendering Learning Path

## Core 2D Foundation
- Orthographic projection
- Sprite rendering (textured quads)
- Batching (minimize draw calls)
- Texture atlases / sprite sheets
- Instancing for repeated shapes

## Blending & Compositing
- Alpha blending modes (src/dst factors)
- Pre-multiplied alpha
- Order-independent transparency
- Blend equations for effects (additive, multiply, screen)

## Text Rendering
- Bitmap fonts (texture atlas approach)
- Signed Distance Field (SDF) fonts - smooth at any scale
- Subpixel rendering

## Anti-Aliasing
- MSAA
- FXAA / SMAA (post-process)
- Analytical AA for lines/edges in shaders

## Framebuffers & Post-Processing
- Render-to-texture
- Multi-pass rendering
- Blur (Gaussian, box)
- Glow / bloom
- Color grading

## Performance at Scale
- Geometry batching (dynamic VBOs)
- Texture binding reduction
- Shader switching minimization
- Frustum/viewport culling
- Spatial partitioning (quadtree)

## Advanced 2D
- Stencil buffer (masking, clipping regions)
- Signed Distance Fields for shapes
- Bezier curves in shaders
- GPU-driven particle systems
- Compute shaders for physics/simulation

## Next Steps for sky-renderer
1. Batching renderer - group similar shapes, one draw call
2. Text rendering - SDF fonts
3. Framebuffers - enables post-processing & layers

## Resources
- [learnopengl.com](https://learnopengl.com)
- OpenGL SuperBible (book)
- Real-Time Rendering (book)
