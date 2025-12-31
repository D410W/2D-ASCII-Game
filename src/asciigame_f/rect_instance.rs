#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectInstance {
  pub position: [f32; 2], // X, Y coordinates (0.0 to 1.0)
  pub color: [f32; 3],  // R, G, B
  pub size: [f32; 2],   // Width, Height
}

impl RectInstance {
  // Describes how this data looks in memory so the GPU can read it
  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<RectInstance>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance, // We change data per RECTANGLE, not per vertex
      attributes: &[
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 0, shader_location: 0 }, // pos
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x3, offset: 8, shader_location: 1 }, // color
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x2, offset: 20, shader_location: 2 }, // size
      ],
    }
  }
}
