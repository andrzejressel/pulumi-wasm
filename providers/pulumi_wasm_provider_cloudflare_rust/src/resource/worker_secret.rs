//! Provides a Cloudflare Worker secret resource.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let mySecret = worker_secret::create(
//!         "mySecret",
//!         WorkerSecretArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("MY_EXAMPLE_SECRET_TEXT")
//!             .script_name("script_1")
//!             .secret_text("my_secret_value")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workerSecret:WorkerSecret example <account_id>/<script_name>/<secret_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkerSecretArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerSecretResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub name: pulumi_wasm_rust::Output<String>,
    /// The name of the Worker script to associate the secret with. **Modifying this attribute will force creation of a new resource.**
    pub script_name: pulumi_wasm_rust::Output<String>,
    /// The text of the Worker secret. **Modifying this attribute will force creation of a new resource.**
    pub secret_text: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkerSecretArgs) -> WorkerSecretResult {

    let result = crate::bindings::pulumi::cloudflare::worker_secret::invoke(name, &crate::bindings::pulumi::cloudflare::worker_secret::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
        script_name: &args.script_name.get_inner(),
        secret_text: &args.secret_text.get_inner(),
    });

    WorkerSecretResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
        script_name: crate::into_domain(result.script_name),
        secret_text: crate::into_domain(result.secret_text),
    }
}
