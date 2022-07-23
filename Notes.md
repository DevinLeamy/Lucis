# Sources
- [Ray Tracing Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

# Conventions

### Coordinate System
- Right hand coordinate system, `-z` runs into the screen
- `+x`: to the right
- `+y`: up
- `+z`: out of the screen (towards you) 

### Surface Normals
- Always point in the opposite direction of the incident ray
- Allows us to determine the side of the surface at the time of geometry intersection (rather
  than at the time of coloring)

# Basic Steps
1. Calculate the ray from the eye to the pixel.
2. Determine which objects the ray intersects.
3. Compute the color for that intersection point.

# Definitions
- `focus length`: Distance between the projection plane and the projection point 
- `antialiasing`: Sending rays through multiple points inside a pixel to get an average color
- `gamma correction`: transforming a value in the range [0, 1] before storing in (as bytes)
- `shadow acne`: undesirable featured caused by floating point approximations
  - eg: rays intersecting at `t = -0.0000001` or `t = 0.0000001` rather than exactly `t = 0` 

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

### Determining the Location of the Ray Based on a Surface Normal
- If the `ray` and the `normal` point in the same direction, the ray
  is coming from within the object
- Otherwise, the ray is coming from outside of the object
- This can be determined by taking the dot product of the two vectors
