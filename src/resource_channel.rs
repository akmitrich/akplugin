use std::{
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use crate::{log, uni};

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
    log(&format!("Destroy channel. {:p}", _channel));
    uni::TRUE
}

pub unsafe extern "C" fn channel_open(channel: *mut uni::mrcp_engine_channel_t) -> uni::apt_bool_t {
    log(&format!("Open channel. {:p}", channel));
    helper_engine_channel_open_respond(channel, uni::TRUE)
}

unsafe extern "C" fn channel_close(channel: *mut uni::mrcp_engine_channel_t) -> uni::apt_bool_t {
    log(&format!("Close channel. {:p}", channel));
    helper_engine_channel_close_respond(channel)
}

unsafe extern "C" fn channel_process_request(
    _channel: *mut uni::mrcp_engine_channel_t,
    request: *mut uni::mrcp_message_t,
) -> uni::apt_bool_t {
    let method_id = unsafe { (*request).start_line.method_id as u32 };
    let cmd = match method_id {
        uni::SYNTHESIZER_SET_PARAMS => "SYNTHESIZER_SET_PARAMS",
        uni::SYNTHESIZER_GET_PARAMS => "SYNTHESIZER_GET_PARAMS",
        uni::SYNTHESIZER_SPEAK => "SYNTHESIZER_SPEAK",
        uni::SYNTHESIZER_STOP => "SYNTHESIZER_STOP",
        uni::SYNTHESIZER_PAUSE => "SYNTHESIZER_PAUSE",
        uni::SYNTHESIZER_RESUME => "SYNTHESIZER_RESUME",
        uni::SYNTHESIZER_BARGE_IN_OCCURRED => "SYNTHESIZER_BARGE_IN_OCCURRED",
        uni::SYNTHESIZER_CONTROL => "SYNTHESIZER_CONTROL",
        uni::SYNTHESIZER_DEFINE_LEXICON => "SYNTHESIZER_DEFINE_LEXICON",
        _ => "Other",
    };
    log(&format!(
        "Request {cmd} processing. {:p} {:p}",
        _channel, request
    ));
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

unsafe fn helper_engine_channel_open_respond(
    channel: *mut uni::mrcp_engine_channel_t,
    status: uni::apt_bool_t,
) -> uni::apt_bool_t {
    (*(*channel).event_vtable).on_open.unwrap()(channel, status)
}

unsafe fn helper_engine_channel_close_respond(
    channel: *mut uni::mrcp_engine_channel_t,
) -> uni::apt_bool_t {
    (*(*channel).event_vtable).on_close.unwrap()(channel)
}
