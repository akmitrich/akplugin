use std::ptr::NonNull;

use crate::{log, resource_channel::AkChannel, uni};

pub static ENGINE_VTABLE: uni::mrcp_engine_method_vtable_t = uni::mrcp_engine_method_vtable_t {
    destroy: Some(engine_destroy),
    open: Some(engine_open),
    close: Some(engine_close),
    create_channel: Some(engine_create_channel),
};
unsafe extern "C" fn engine_destroy(_engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    log("Engine destroy.");
    uni::TRUE
}

unsafe extern "C" fn engine_open(engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    let config = uni::mrcp_engine_config_get(engine);
    log(&format!("Open Engine. Get config: {:p}", (*config).params));
    (*engine).obj = Box::into_raw(Box::new(AkEngine::new())) as *mut _;
    helper_engine_open_respond(engine, uni::TRUE)
}

unsafe extern "C" fn engine_close(engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    helper_engine_close_respond(engine)
}

unsafe extern "C" fn engine_create_channel(
    engine: *mut uni::mrcp_engine_t,
    pool: *mut uni::apr_pool_t,
) -> *mut uni::mrcp_engine_channel_t {
    let ak_channel = AkChannel::new(pool);
    let channel_ptr = Box::into_raw(Box::new(ak_channel));
    let capabilities = uni::mpf_stream_capabilities_create(uni::STREAM_DIRECTION_RECEIVE, pool);
    uni::mpf_codec_default_capabilities_add(&mut (*capabilities).codecs as *mut _);
    let termination = uni::mrcp_engine_audio_termination_create(
        channel_ptr as _,
        &crate::audio_stream::VTABLE,
        capabilities,
        pool,
    );
    let channel = uni::mrcp_engine_channel_create(
        engine,
        &crate::resource_channel::VTABLE,
        channel_ptr as *mut _,
        termination,
        pool,
    );
    (*channel_ptr).lock().unwrap().channel = NonNull::new(channel).unwrap();
    channel
}

#[repr(C)]
pub struct AkEngine;

impl AkEngine {
    pub fn new() -> Self {
        Self
    }
}

unsafe fn helper_engine_open_respond(
    engine: *mut uni::mrcp_engine_t,
    status: uni::apt_bool_t,
) -> uni::apt_bool_t {
    ((*(*engine).event_vtable).on_open.unwrap())(engine, status)
}

unsafe fn helper_engine_close_respond(engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    ((*(*engine).event_vtable).on_close.unwrap())(engine)
}
