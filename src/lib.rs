#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub use po_file_read_v3 as po_file_read;
pub use po_file_write_v2 as po_file_write;
pub use po_message_check_format_v2 as po_message_check_format;

#[cfg(test)]
mod tests {
    use std::{ffi::CString, os, ptr};

    use super::*;

    #[test]
    fn test_hello() {
        unsafe extern "C" fn my_xerror_handler(
            _severity: os::raw::c_int,
            _message: po_message_t,
            _filename: *const os::raw::c_char,
            _lineno: usize,
            _column: usize,
            _multiline_p: os::raw::c_int,
            message_text: *const os::raw::c_char,
        ) {
            let message_text = unsafe { CString::from_raw(message_text as *mut _) };
            println!("xerror_handler: {}", message_text.to_str().unwrap());
        }

        unsafe extern "C" fn my_error_handler(
            _severity: os::raw::c_int,
            _message1: po_message_t,
            _filename1: *const os::raw::c_char,
            _lineno1: usize,
            _column1: usize,
            _multiline_p1: os::raw::c_int,
            message_text1: *const os::raw::c_char,
            _message2: po_message_t,
            _filename2: *const os::raw::c_char,
            _lineno2: usize,
            _column2: usize,
            _multiline_p2: os::raw::c_int,
            _message_text2: *const os::raw::c_char,
        ) {
            let message_text = unsafe { CString::from_raw(message_text1 as *mut _) };
            println!("error_handler: {}", message_text.to_str().unwrap());
        }

        let _xerror_handler = po_xerror_handler {
            xerror: Some(my_xerror_handler),
            xerror2: Some(my_error_handler),
        };

        unsafe {
            let file = po_file_create();
            let iter = po_message_iterator(file, ptr::null());
            let msg = po_message_create();

            let msgid = CString::new("").unwrap();
            po_message_set_msgid(msg, msgid.as_ptr());

            let msgstr = CString::new("Hello, world!").unwrap();
            po_message_set_msgstr(msg, msgstr.as_ptr());

            po_message_insert(iter, msg);

            // let filename = CString::new("sample.po").unwrap();
            // po_file_write_v2(file, filename.as_ptr(), &xerror_handler);
        }
    }
}
