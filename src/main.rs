#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    let msg = "Hello, world!\n";
    let ptr = msg.as_bytes() as *const [u8] as *const u8;
    unsafe {
        output(1, ptr, 14)
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(name="syscall", kind="static")]
extern "C" {
    fn output(fd: i64, ptr: *const u8, len: usize);
}
