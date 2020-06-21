use amuri::parse::uri::parse_uri;
use amuri::query::client::Client;
use amuri::traits::Retriever;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
/// Given a uri make a call to the underlying Amuri engine and attempt to get the
/// path or return a copy of the original path if the previous attempt fails
pub extern "C" fn get_path_from_uri(s: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    let client = if let Ok(client) = Client::from_env() {
        client
    } else {
        return CString::new(r_str).unwrap().into_raw();
    };

    match parse_uri(&r_str) {
        Ok(asset_model) => match client.get(&asset_model.to_owned()) {
            Ok(file) => CString::new(file).unwrap().into_raw(),
            Err(_) => CString::new(r_str).unwrap().into_raw(),
        },
        Err(_) => CString::new(r_str).unwrap().into_raw(),
    }
}

#[no_mangle]
/// Rust owns the string returned from get_path_from_uri. Rust
/// must free it ultimately.
pub extern "C" fn path_from_uri_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
