use super::ffi;

pub fn powerctl_hibernate() {
    unsafe {
        ffi::powerctl_hibernate();
    }
}
