struct InstanceInput {
  @location(0) position: vec2<f32>,
  @location(1) color: vec3<f32>,
  @location(2) size: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
  @builtin(vertex_index) in_vertex_index: u32,
  instance: InstanceInput,
) -> VertexOutput {
  var out: VertexOutput;
  
  // A standard generic unit square (0,0 to 1,1)
  // We construct the vertices on the fly using the index
  var x = f32(in_vertex_index & 1u);
  var y = f32(in_vertex_index >> 1u); // 0, 1, 0, 1...

  // 1. Scale by size
  var scaled_pos = vec2<f32>(x, y) * instance.size;
  
  // 2. Move to position (Top-Left coordinate system)
  var world_pos = instance.position + scaled_pos;

  // 3. Convert to Clip Space (-1.0 to 1.0)
  // We need the screen size (resolution) to do this perfectly.
  // For simplicity, let's assume the user passes Normalized Coordinates (-1 to 1) 
  // or we pass a Uniform with screen size. 
  //
  // SIMPLIFICATION: Let's assume input position is already in -1.0 to 1.0 space for now.
  out.clip_position = vec4<f32>(world_pos.x, world_pos.y, 0.0, 1.0);
  out.color = instance.color;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}
