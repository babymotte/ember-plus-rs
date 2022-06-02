use libember_sys::{pcstr, size_t};
use std::{
    ffi::{c_void, CStr},
    net::SocketAddr,
    os::raw::c_int,
    thread,
    time::Duration,
};

pub fn connect(addr: SocketAddr) {
    log::debug!("Using socket address {:?}", addr);

    unsafe {
        libember_sys::ember_init(
            Some(throw_error),
            Some(fail_assertion),
            Some(alloc_memory),
            Some(free_memory),
        );
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

unsafe extern "C" fn throw_error(error: c_int, p_message: pcstr) {
    let msg = CStr::from_ptr(p_message).to_string_lossy();
    log::error!("ber error {}: {}", error, msg);
}

unsafe extern "C" fn fail_assertion(p_file_name: pcstr, line_number: c_int) {
    let file = CStr::from_ptr(p_file_name).to_string_lossy();
    log::error!("Debug assertion failed @ '{}' line {}", file, line_number);
}

unsafe extern "C" fn alloc_memory(_size: size_t) -> *mut c_void {
    todo!()
}

unsafe extern "C" fn free_memory(_p_memory: *mut c_void) {
    todo!()
}
