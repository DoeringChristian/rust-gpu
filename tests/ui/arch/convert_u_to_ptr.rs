// build-pass
// compile-flags: -C target-feature=+PhysicalStorageBufferAddresses,+ext:SPV_KHR_physical_storage_buffer

use spirv_std::*;

unsafe fn convert_u_to_ptr<T>(handle: u64) -> *mut T {
    let result: *mut T;
    asm!(
        "{result} = OpConvertUToPtr typeof{result} {handle}",
        handle = in(reg) handle,
        result = out(reg) result,
    );
    result
}

#[spirv(fragment)]
pub fn main(out: &mut u32) {
    unsafe {
        let x: *mut RuntimeArray<u32> = convert_u_to_ptr(100);
        *out = *(*x).index(0);
    }
}