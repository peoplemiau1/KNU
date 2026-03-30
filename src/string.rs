#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    let mut i = 0;
    while *s.offset(i) != 0 {
        i += 1;
    }
    i as usize
}
