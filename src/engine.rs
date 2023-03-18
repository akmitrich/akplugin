use std::{collections::HashMap, ptr::NonNull};

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
    let _config = uni::mrcp_engine_config_get(engine);
    log(&format!("Open Engine: {:p}", engine));
    (*engine).obj = Box::into_raw(Box::new(AkEngine::new())) as *mut _;
    helper_engine_open_respond(engine, uni::TRUE)
}

unsafe extern "C" fn engine_close(engine: *mut uni::mrcp_engine_t) -> uni::apt_bool_t {
    log(&format!("Close engine. {:p}", engine));
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
    log(&format!("Create channel. {:p}", channel));
    (*channel_ptr).lock().unwrap().channel = NonNull::new(channel).unwrap();
    channel
}

#[repr(C)]
pub struct AkEngine {
    pub yandex_iam_token: String,
}

impl AkEngine {
    pub fn new() -> Self {
        Self {
            yandex_iam_token: Self::get_iam_token(),
        }
    }

    fn get_iam_token() -> String {
        const IAM_TOKEN_KEY: &str = "iamToken";
        let client = reqwest::blocking::Client::new();
        let req = client
            .post("https://iam.api.cloud.yandex.net/iam/v1/tokens")
            .query(&[("yandexPassportOauthToken", crate::secret::YANDEX_KEY)]);
        let res = req.send().expect("need IAM-token but network fails");
        let json: HashMap<String, String> = res
            .json()
            .expect("need IAM-token but server responds without JSON");
        String::from(&json[IAM_TOKEN_KEY])
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
