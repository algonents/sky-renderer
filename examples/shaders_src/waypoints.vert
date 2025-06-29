#version 330 core
    // Input attributes (from the vertex buffer)
    layout(location = 0) in vec2 wgs84_coords; // WGS84 coordinates (longitude, latitude)
    
    uniform mat4 transform; 
    uniform vec4 map_bounds;
    
    // Constants
    const float EARTH_RADIUS = 6378137.0; // Earth radius in meters
    const float PI = 3.141592653589793;  // Value of π

    // Function to convert WGS84 to Mercator
    vec2 wgs84_to_mercator(vec2 wgs84) {
        float lon_rad = radians(wgs84.x); // Convert longitude to radians
        float lat_rad = radians(wgs84.y); // Convert latitude to radians
    
        float mercator_x = lon_rad * EARTH_RADIUS;
        float mercator_y = log(tan(PI / 4.0 + lat_rad / 2.0)) * EARTH_RADIUS;

        return vec2(mercator_x, mercator_y);
    }

    void main() {
        // WGS84 to Mercator transformation for the vertex
        vec2 mercator_coords = wgs84_to_mercator(wgs84_coords);

        // WGS84 to Mercator transformation for the bounds
        vec2 lon_min_lat_min = wgs84_to_mercator(vec2(map_bounds.x, map_bounds.y));
        vec2 lon_max_lat_max = wgs84_to_mercator(vec2(map_bounds.z, map_bounds.w));

        // Normalize Mercator coordinates to device coordinates (-1 to 1)
        float device_x = 2.0 * (mercator_coords.x - lon_min_lat_min.x) / (lon_max_lat_max.x - lon_min_lat_min.x) - 1.0;
        float device_y = 2.0 * (mercator_coords.y - lon_min_lat_min.y) / (lon_max_lat_max.y - lon_min_lat_min.y) - 1.0;

        // Set the final vertex position in clip space
        gl_Position = transform * vec4(device_x, device_y, 0.0, 1.0);
    }