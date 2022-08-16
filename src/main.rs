const SHADER: &[u8] = include_bytes!(env!("shaders.spv"));

fn main() {
    println!("Shader module: {} bytes", SHADER.len());
}
