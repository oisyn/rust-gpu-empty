use spirv_builder::{MetadataPrintout, SpirvBuilder, Capability};

fn main() {
    SpirvBuilder::new("shaders", "spirv-unknown-vulkan1.1")
    .print_metadata(MetadataPrintout::Full)
    //.capability(Capability::RuntimeDescriptorArray)
    .capability(Capability::Int64)
    //.capability(Capability::RayQueryKHR)
    //.extension("SPV_EXT_descriptor_indexing")
    //.extension("SPV_KHR_ray_query")
    .build()
    .unwrap(); 
}