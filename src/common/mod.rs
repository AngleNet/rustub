pub mod config;
pub mod error;

pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *((dest as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
        i += 1;
    }

    dest
}
