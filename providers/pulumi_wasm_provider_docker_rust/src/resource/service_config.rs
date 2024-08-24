//! 
//! 
//! ## Import
//! 
//! ### Example
//! 
//! Assuming you created a `config` as follows
//! 
//! #!/bin/bash
//! 
//! printf '{"a":"b"}' | docker config create foo -
//! 
//! prints the id 
//! 
//! 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
//! 
//! you provide the definition for the resource as follows
//! 
//! terraform
//! 
//! resource "docker_config" "foo" {
//! 
//!   name = "foo"
//! 
//!   data = base64encode("{\"a\": \"b\"}")
//! 
//! }
//! 
//! then the import command is as follows
//! 
//! #!/bin/bash
//! 
//! ```sh
//! $ pulumi import docker:index/serviceConfig:ServiceConfig foo 08c26c477474478d971139f750984775a7f019dbe8a2e7f09d66a187c009e66d
//! ```
//! 

pub struct ServiceConfigArgs {
    /// Base64-url-safe-encoded config data
    pub data: pulumi_wasm_rust::Output<String>,
    /// User-defined name of the config
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
        data: args.data.get_inner(),
        name: args.name.get_inner(),
    });

    ServiceConfigResult {
        data: crate::into_domain(result.data),
        name: crate::into_domain(result.name),
    }
}
