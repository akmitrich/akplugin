use crate::uni;

pub static ENGINE_VTABLE: uni::mrcp_engine_method_vtable_t = uni::mrcp_engine_method_vtable_t {
    destroy: Some(engine_destroy),
    open: Some(engine_open),
    close: Some(engine_close),
    create_channel: Some(engine_create_channel),
};
unsafe extern "C" fn engine_destroy(_engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    uni::TRUE
}

unsafe extern "C" fn engine_open(engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    let _config = uni::mrcp_engine_config_get(engine);
    //helper_engine_open_respond(engine, uni::TRUE)
    uni::TRUE
}

unsafe extern "C" fn engine_close(_engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    // helper_engine_close_respond(engine)
    uni::TRUE
}

unsafe extern "C" fn engine_create_channel(
    _engine: *mut uni::mrcp_engine_t,
    _pool: *mut uni::apr_pool_t,
) -> *mut uni::mrcp_engine_channel_t {
    //channel
    std::ptr::null_mut()
}
