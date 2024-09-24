pub mod header;
use wasm_bindgen::prelude::*;

static HEADERS : [(&str, &str); 2] = [
    ("x-foo", "bar"),
    ("x-the-first-traefik-plugin-in-rust", "true"),
];

#[wasm_bindgen]
pub fn handle_request() -> u64 {
    unsafe {
        // set_header_value(kind: HeaderKind, name_ptr: u32, name_size: u32, value_ptr: u32, value_len: u32)
        for (name, value) in HEADERS.iter() {
            header::set_header_value(header::HeaderKind::Response, name.as_ptr() as u32, name.len() as u32, value.as_ptr() as u32, value.len() as u32);
        }
    }
    return 1;
}

#[wasm_bindgen]
pub fn handle_response(req_ctx: u32, is_err: u32) {
}