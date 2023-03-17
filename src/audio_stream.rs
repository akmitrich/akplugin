use std::sync::{Arc, Mutex};

use crate::{log, resource_channel::AkChannel, uni};

pub static VTABLE: uni::mpf_audio_stream_vtable_t = uni::mpf_audio_stream_vtable_t {
    destroy: Some(stream_destroy),
    open_rx: Some(stream_open),
    close_rx: Some(stream_close),
    read_frame: Some(stream_read),
    open_tx: None,
    close_tx: None,
    write_frame: None,
    trace: Some(trace),
};

pub unsafe extern "C" fn stream_destroy(_stream: *mut uni::mpf_audio_stream_t) -> uni::apt_bool_t {
    log(&format!("Destroy audio stream {:p}", _stream));
    uni::TRUE
}

pub unsafe extern "C" fn stream_open(
    _stream: *mut uni::mpf_audio_stream_t,
    _codec: *mut uni::mpf_codec_t,
) -> uni::apt_bool_t {
    log(&format!("Open audio stream: {:p}", _stream));
    uni::TRUE
}

pub unsafe extern "C" fn stream_close(_stream: *mut uni::mpf_audio_stream_t) -> uni::apt_bool_t {
    log(&format!("Close audio stream: {:p}", _stream));
    uni::TRUE
}

pub unsafe extern "C" fn stream_read(
    stream: *mut uni::mpf_audio_stream_t,
    _frame: *mut uni::mpf_frame_t,
) -> uni::apt_bool_t {
    log(&format!("Read audio stream {:p}", stream));
    let ak_channel = (*stream).obj as *mut Arc<Mutex<AkChannel>>;
    let mut channel_lock = (*ak_channel).lock().unwrap();
    if let Some(msg) = channel_lock.speak_msg {
        channel_lock.speak_msg = None;
        let pool = (*msg).pool;
        log(&format!("Speak msg is {:p}, pool is {:p}", msg, pool));
        let complete_msg = uni::mrcp_event_create(msg, uni::SYNTHESIZER_SPEAK_COMPLETE as _, pool);
        log(&format!("Complete msg is {:p}", complete_msg));
        if !complete_msg.is_null() {
            (*complete_msg).start_line.request_state = uni::MRCP_REQUEST_STATE_COMPLETE;
            log(&format!("ak_channel state {:?}", (*ak_channel).as_ref()));
            channel_lock.engine_channel_message_send(complete_msg);
            log(&format!("Complete msg successfully sent."));
        }
    }
    uni::TRUE
}

pub unsafe extern "C" fn trace(
    _stream: *mut uni::mpf_audio_stream_t,
    _direction: uni::mpf_stream_direction_e,
    _output: *mut uni::apt_text_stream_t,
) {
    log(&format!(
        "Trace audio stream {:p} in direction {}",
        _stream, _direction
    ))
}
