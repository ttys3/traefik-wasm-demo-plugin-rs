
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderKind {
    /// Represents an operation on HTTP request headers.
    Request = 0,

    /// Represents an operation on HTTP response headers.
    Response = 1,

    /// Represents an operation on HTTP request trailers (trailing headers).
    /// This requires FeatureTrailers.
    ///
    /// To enable FeatureTrailers, call FuncEnableFeatures prior to FuncNext.
    /// Doing otherwise, may result in a panic.
    RequestTrailers = 2,

    /// Represents an operation on HTTP response trailers (trailing headers).
    /// This requires FeatureTrailers.
    ///
    /// To enable FeatureTrailers, call FuncEnableFeatures prior to FuncNext.
    /// Doing otherwise, may result in a panic.
    ResponseTrailers = 3,
}

#[link(wasm_import_module = "http_handler")]
extern "C" {
    #[link_name = "set_header_value"]
    fn set_header_value_impl(kind: u32, name_ptr: u32, name_size: u32, value_ptr: u32, value_len: u32);
}

#[no_mangle]
pub unsafe fn set_header_value(kind: HeaderKind, name_ptr: u32, name_size: u32, value_ptr: u32, value_len: u32) {
    set_header_value_impl(kind as u32, name_ptr, name_size, value_ptr, value_len);
}