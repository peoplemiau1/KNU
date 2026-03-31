pub fn get_path<'a>(envp: *const *const u8) -> Option<&'a str> {
    if envp.is_null() { return None; }
    let mut e = 0;
    unsafe {
        while !(*envp.offset(e)).is_null() {
            let ptr = *envp.offset(e);
            let mut len = 0;
            while *ptr.offset(len) != 0 { len += 1; }
            let slice = core::slice::from_raw_parts(ptr, len as usize);
            if let Ok(s) = core::str::from_utf8(slice) {
                if let Some(path_val) = s.strip_prefix("PATH=") {
                    return Some(path_val);
                }
            }
            e += 1;
        }
    }
    None
}

pub fn parse_args<'a>(argc: isize, argv: *const *const u8) -> [&'a str; 32] {
    let mut res = [""; 32];
    if argv.is_null() { return res; }
    let mut i = 0;
    while i < argc && i < 32 {
        unsafe {
            let ptr = *argv.offset(i);
            if !ptr.is_null() {
                let mut len = 0;
                while *ptr.offset(len) != 0 { len += 1; }
                let slice = core::slice::from_raw_parts(ptr, len as usize);
                if let Ok(s) = core::str::from_utf8(slice) {
                    res[i as usize] = s;
                }
            }
        }
        i += 1;
    }
    res
}
