// build-pass
// compile-flags: -C target-feature=+PhysicalStorageBufferAddresses,+ext:SPV_KHR_physical_storage_buffer

use spirv_std::*;
use glam::{Mat4, Vec3};

#[repr(C)]
pub struct Struct {
    pub a: Mat4,
    pub b: Mat4,
    pub c: Vec3,
    pub d: f32,
    pub e: u32,
    pub f: u32,
    pub g: u32,
    pub h: bool,
}

#[spirv(fragment)]
pub fn main(out: &mut u32) {
    unsafe {
        let x: *mut RuntimeArray<u32> = arch::convert_u_to_ptr(100);
        let y: *mut Struct = arch::convert_u_to_ptr(200);

        let mat = (*y).a;

        *out = *(*x).index(0) + (*y).g;
    }
}
