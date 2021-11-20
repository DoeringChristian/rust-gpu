// build-pass



use spirv_std::{memory::{Scope, Semantics}};

#[spirv(fragment)]
pub fn main(mut output: &mut f32) {
    unsafe {
        spirv_std::arch::atomic_store::<
            _,
            { Scope::CrossDevice as u32 },
            { Semantics::IMAGE_MEMORY.bits() as u32 }
        >(&mut *output, 5.0);
    }
}
