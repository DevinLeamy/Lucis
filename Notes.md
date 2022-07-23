# Sources
- [Ray Tracing Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

# Coordinate System
- Right hand coordinate system, `-z` runs into the screen
- `+x`: to the right
- `+y`: up
- `+z`: out of the screen (towards you) 

# Basic Steps
1. Calculate the ray from the eye to the pixel.
2. Determine which objects the ray intersects.
3. Compute the color for that intersection point.

# Definitions
- `focus length`: Distance between the projection plane and the projection point <br>

# Creating Gradients with Interpolation
![Gradient](images/gradient.png)

# Vector Operations

### Subtracting Vectors
- let `P = Vec3(A, B, C)`
- let `C = Vec3(D, E, F)`
- Therefore, `P - C` is the vector from point `C` to point `P`

# Ray-Sphere Intersection
![Ray-Sphere Intersection](images/ray-sphere%20intersection.png)
![Ray-Sphere Roots](images/ray-sphere%20roots.png)
