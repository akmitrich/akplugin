use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

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

#[repr(C)]
pub struct AkChannel {
    pub channel: NonNull<uni::mrcp_engine_channel_t>,
    pub detector: Option<NonNull<uni::mpf_activity_detector_t>>,
}

impl AkChannel {
    pub fn new(pool: *mut uni::apr_pool_t) -> Arc<Mutex<Self>> {
        let uni_detector = unsafe { uni::mpf_activity_detector_create(pool) };
        let channel = Self {
            channel: NonNull::dangling(),
            detector: NonNull::new(uni_detector),
        };
        Arc::new(Mutex::new(channel))
    }
}
