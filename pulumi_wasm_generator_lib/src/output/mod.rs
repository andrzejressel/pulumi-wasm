pub(crate) mod provider;
pub(crate) mod rust;
pub(crate) mod wit;

fn get_main_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
