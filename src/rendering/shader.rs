// Emulates the functionality of a shader in GLSL
// We can request triangle information and other things by enum flags
// If it can be provided by current info it's passed into your shader

/// Denotes how this shader should be used by the renderer
/// # Notes
///     [ShaderType::Vertex] can be used without a corresponding [ShaderType::Fragment] unit in a [ShaderProgram] but there are exceptions, reference [ShaderProgram] for more info!
#[repr(C)]
pub enum ShaderType {
    /// Provides per-vertex shading that is interpolated over the triangle
    Vertex,

    /// Provides per-pixel shading
    Fragment,
}

pub trait Shader {
    fn request_info();

    fn shade();
}