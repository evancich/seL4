#![no_std]

use core::hint::spin_loop;
use core::ptr;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { spin_loop() }
}

/// void memclr(void *dst, size_t n)  (non-standard, kernel-local)
#[no_mangle]
pub unsafe extern "C" fn memclr(dst: *mut core::ffi::c_void, n: usize) {
    if n == 0 { return; }
    ptr::write_bytes(dst as *mut u8, 0xAAu8, n);
}
