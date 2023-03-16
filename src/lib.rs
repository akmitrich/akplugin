pub mod audio_stream;
pub mod engine;
pub mod resource_channel;

pub mod uni {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(clippy::all)]
    #![allow(rustdoc::broken_intra_doc_links)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub const FALSE: apt_bool_t = 0;
    pub const TRUE: apt_bool_t = 1;
}

#[no_mangle]
pub static mut mrcp_plugin_version: uni::mrcp_plugin_version_t = uni::mrcp_plugin_version_t {
    major: uni::PLUGIN_MAJOR_VERSION as i32,
    minor: uni::PLUGIN_MINOR_VERSION as i32,
    patch: uni::PLUGIN_PATCH_VERSION as i32,
    is_dev: 0,
};

#[no_mangle]
pub extern "C" fn mrcp_plugin_create(pool: *mut uni::apr_pool_t) -> *mut uni::mrcp_engine_t {
    eprintln!("[AK-Plugin] Start the engine.");
    unsafe {
        // Engines's object pointer set
        // to null. It will be initialized in `engine_open`.
        uni::mrcp_engine_create(
            uni::MRCP_SYNTHESIZER_RESOURCE as _,
            std::ptr::null_mut(),
            &engine::ENGINE_VTABLE as *const _,
            pool,
        )
    }
}

pub(crate) fn log(s: &str) {
    eprintln!("[AK-Plugin] {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, uni::FALSE);
        assert_eq!(1, uni::TRUE);
    }

    #[test]
    fn test_version() {
        unsafe {
            println!(
                "MAJOR {}\nMINOR {}\nPATCH {}\n",
                mrcp_plugin_version.major, mrcp_plugin_version.minor, mrcp_plugin_version.patch
            );
        }
    }
}
