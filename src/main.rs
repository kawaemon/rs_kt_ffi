#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod libkt {
    include!("../binding.rs");
}

use std::ffi::{CStr, CString};

fn main() {
    unsafe {
        let text = CString::new("hoge").unwrap();

        let libkt = *libkt::libkt_symbols();
        let ret = libkt.kotlin.root.kotlin.onMessage.unwrap()(text.as_ptr());

        println!("libkt says: {}", CStr::from_ptr(ret).to_string_lossy());

        libkt.DisposeString.unwrap()(ret);
    }
}
