pub mod impl_ {
    pub mod let_ {
        pub mod loop_ {
            include!("resources/impl_/let_/loop_/type.rs");
        }
    }
}
pub mod functions {
    pub mod impl_ {
        pub mod let_ {
            pub mod loop_ {
                include!("functions/impl_/let_/loop_/type.rs");
            }
        }
    }
}
pub mod types {
    pub mod impl_ {
        pub mod let_ {
            pub mod loop_ {
                include!("types/impl_/let_/loop_/type.rs");
            }
        }
    }
}
#[doc(hidden)]
pub mod constants {}
#[link_section = "pulumi_gestalt_provider::example"]
#[no_mangle]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_EXAMPLE: [u8; 44] = *b"{\"version\":\"1.0.0\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "1.0.0".to_string()
}
