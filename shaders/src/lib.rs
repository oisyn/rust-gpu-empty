#![cfg_attr(
    target_arch = "spirv",
    no_std
)]
#![allow(unused_variables)]

use spirv_std::{spirv, ray_tracing::AccelerationStructure};
use spirv_std::glam::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PushConstants {
    pub acceleration_structure_address: u64,
}


#[spirv(fragment)]
pub fn main_fs(
    #[spirv(push_constant)] push_constants: &PushConstants,
    output: &mut Vec4,
) {
    let acceleration_structure =
    unsafe { AccelerationStructure::from_u64(push_constants.acceleration_structure_address) };
}
