// build-pass

use spirv_std::{memory::{Scope, Semantics}, Image};

#[spirv(compute(threads(8, 8, 1)))]
pub fn main(
    #[spirv(descriptor_set = 0, binding = 0, storage_buffer)] buffer: &mut [f32]
) {
    unsafe {
        let old = spirv_std::arch::atomic_exchange::<
            _,
            { Scope::Device as u8 },
            { Semantics::NONE.bits() as u8 }
        >(&mut buffer[0], 5.0);
    }
}
