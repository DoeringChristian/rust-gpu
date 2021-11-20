// build-pass



use spirv_std::{memory::{Scope, Semantics}};

#[spirv(fragment)]
pub fn main(output: &mut u32) {
    unsafe {
        let old = spirv_std::arch::atomic_u_max::<
            _,
            { Scope::CrossDevice as u32 },
            { Semantics::IMAGE_MEMORY.bits() as u32 },
        >(&mut *output, 10);
    }
}
