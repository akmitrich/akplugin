use crate::uni;

pub static STREAM_VTABLE: uni::mpf_audio_stream_vtable_t = uni::mpf_audio_stream_vtable_t {
    destroy: Some(stream_destroy),
    open_rx: None,
    close_rx: None,
    read_frame: None,
    open_tx: Some(stream_open),
    close_tx: Some(stream_close),
    write_frame: Some(stream_write),
    trace: Some(trace),
};

pub unsafe extern "C" fn stream_destroy(_stream: *mut uni::mpf_audio_stream_t) -> uni::apt_bool_t {
    uni::TRUE
}

pub unsafe extern "C" fn stream_open(
    _stream: *mut uni::mpf_audio_stream_t,
    _codec: *mut uni::mpf_codec_t,
) -> uni::apt_bool_t {
    uni::TRUE
}

pub unsafe extern "C" fn stream_close(_stream: *mut uni::mpf_audio_stream_t) -> uni::apt_bool_t {
    uni::TRUE
}

pub unsafe extern "C" fn stream_write(
    _stream: *mut uni::mpf_audio_stream_t,
    _frame: *const uni::mpf_frame_t,
) -> uni::apt_bool_t {
    uni::TRUE
}

pub unsafe extern "C" fn trace(
    _stream: *mut uni::mpf_audio_stream_t,
    _direction: uni::mpf_stream_direction_e,
    _output: *mut uni::apt_text_stream_t,
) {
}
