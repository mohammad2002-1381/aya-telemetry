use std::ffi::CStr;

pub fn parse_string(bytes: &[u8]) -> String {
    let c_str = CStr::from_bytes_until_nul(bytes)
        .unwrap_or_else(|_| CStr::from_bytes_until_nul(b"\0").unwrap());
    c_str.to_string_lossy().into_owned()
}