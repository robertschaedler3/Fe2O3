use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn encrypt1_rs(text: String, key: i32) -> String {
    unsafe {
        let filecstr = std::ffi::CString::new(text).unwrap();
        let out = encrypt1(filecstr.into_raw(), key);
        CStr::from_ptr(out).to_str().unwrap().to_string()
    }
}

pub fn decrypt1_rs(text: String, key: i32) -> String {
    unsafe {
        let filecstr = std::ffi::CString::new(text).unwrap();
        let out = decrypt1(filecstr.into_raw(), key);
        CStr::from_ptr(out).to_str().unwrap().to_string()
    }
}

pub fn encrypt2_rs(text: String, key: String) -> String {
    unsafe {
        let filecstr = std::ffi::CString::new(text).unwrap();
        let keycstr = std::ffi::CString::new(key).unwrap();
        let out = encrypt2(filecstr.into_raw(), keycstr.into_raw());
        CStr::from_ptr(out).to_str().unwrap().to_string()
    }
}

pub fn decrypt2_rs(text: String, key: String) -> String {
    unsafe {
        let filecstr = std::ffi::CString::new(text).unwrap();
        let keycstr = std::ffi::CString::new(key).unwrap();
        let out = decrypt2(filecstr.into_raw(), keycstr.into_raw());
        CStr::from_ptr(out).to_str().unwrap().to_string()
    }
}