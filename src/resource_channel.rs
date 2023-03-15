use crate::uni;

pub const VTABLE: uni::mrcp_engine_channel_method_vtable_t =
    uni::mrcp_engine_channel_method_vtable_t {
        destroy: Some(channel_destroy),
        open: Some(channel_open),
        close: Some(channel_close),
        process_request: Some(channel_process_request),
    };

pub unsafe extern "C" fn channel_destroy(
    _channel: *mut uni::mrcp_engine_channel_t,
) -> uni::apt_bool_t {
    uni::TRUE
}

pub unsafe extern "C" fn channel_open(
    _channel: *mut uni::mrcp_engine_channel_t,
) -> uni::apt_bool_t {
    uni::TRUE
}

unsafe extern "C" fn channel_close(_channel: *mut uni::mrcp_engine_channel_t) -> uni::apt_bool_t {
    uni::TRUE
}

unsafe extern "C" fn channel_process_request(
    _channel: *mut uni::mrcp_engine_channel_t,
    _request: *mut uni::mrcp_message_t,
) -> uni::apt_bool_t {
    uni::TRUE
}
