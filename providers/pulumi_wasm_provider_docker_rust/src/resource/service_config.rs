//! 
//! 
//! ## Import
//! 
//! ### Example
//! 
//! Assuming you created a `config` as follows
//! 
//! ```shell
//! printf '{"a":"b"}' | docker config create foo -
//! ```
//! 
//! prints the id 
//! 
//! ```text
//! 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
//! ```
//! 
//! you provide the definition for the resource as follows
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let foo = service_config::create(
//!         "foo",
//!         ServiceConfigArgs::builder()
//!             .data("base64encode(\"{\\\"a\\\": \\\"b\\\"}\")")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! then the import command is as follows
//! 
//! ```sh
//! $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ServiceConfigArgs {
    /// Base64-url-safe-encoded config data
    #[builder(into)]
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined name of the config
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ServiceConfigResult {
    /// Base64-url-safe-encoded config data
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined name of the config
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {

    let result = crate::bindings::pulumi::docker::service_config::invoke(name, &crate::bindings::pulumi::docker::service_config::Args {
        data: &args.data.get_inner(),
        name: &args.name.get_inner(),
    });

    ServiceConfigResult {
        data: crate::into_domain(result.data),
        name: crate::into_domain(result.name),
    }
}
