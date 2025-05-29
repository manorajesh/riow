// Input buffer
struct DataBuf {
    data: array<f32>,
}

@group(0)
@binding(0)
var<storage, read> input_data: DataBuf;

// Storage texture for writing output
@group(0) 
@binding(1)
var output_texture: texture_storage_2d<rgba8unorm, write>;

// Workgroup size of 16x16 is common for texture operations
@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Make sure we don't write outside the texture dimensions
    let dimensions = textureDimensions(output_texture);
    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) {
        return;
    }
    
    // Example: Generate a simple pattern based on pixel coordinates
    // You can use the input_data buffer if needed for your actual implementation
    
    // Simple example: create a gradient pattern
    let normalized_x = f32(global_id.x) / f32(dimensions.x);
    let normalized_y = f32(global_id.y) / f32(dimensions.y);
    
    // Use input data to influence the color if available and within range
    var value = 0.5;
    if (global_id.x < arrayLength(&input_data.data)) {
        value = input_data.data[global_id.x] / 100.0; // Scale appropriately
    }
    
    // Create a color from position and input data
    let color = vec4<f32>(
        normalized_x,
        normalized_y,
        value,
        1.0
    );
    
    // Write the color to the storage texture
    textureStore(output_texture, vec2<i32>(global_id.xy), color);
}