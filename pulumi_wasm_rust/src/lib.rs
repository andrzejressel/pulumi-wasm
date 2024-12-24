pub use pulumi_wasm_rust_macro::pulumi_main;
mod output;
pub use output::Output;

pub mod runner;

pub fn add_export<T>(name: &str, output: &Output<T>) {
    output.add_to_export(name);
}

#[macro_export]
macro_rules! include_provider {
    ($file:expr) => {
        include!(concat!(env!("OUT_DIR"), "/pulumi/", $file, "/main.rs"));
    };
}
