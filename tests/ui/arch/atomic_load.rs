// build-pass

use spirv_std::{memory::{Scope, Semantics}};

#[spirv(fragment)]
pub fn main(output: &mut Vec4) {
    unsafe {
        let output = spirv_std::arch::atomic_load::<
            _,
            { Scope::CrossDevice as u32 },
            { Semantics::IMAGE_MEMORY.bits() as u32 }
        >(&*output);
    }
}
