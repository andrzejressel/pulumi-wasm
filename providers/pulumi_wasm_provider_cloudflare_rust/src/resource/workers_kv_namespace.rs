//! Provides the ability to manage Cloudflare Workers KV Namespace features.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = workers_kv_namespace::create(
//!         "example",
//!         WorkersKvNamespaceArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .title("test-namespace")
//!             .build_struct(),
//!     );
//! }
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersKvNamespace:WorkersKvNamespace example <account_id>/<namespace_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersKvNamespaceArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    #[builder(into)]
    pub title: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvNamespaceResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Title value of the Worker KV Namespace.
    pub title: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersKvNamespaceArgs) -> WorkersKvNamespaceResult {

    let result = crate::bindings::pulumi::cloudflare::workers_kv_namespace::invoke(name, &crate::bindings::pulumi::cloudflare::workers_kv_namespace::Args {
        account_id: &args.account_id.get_inner(),
        title: &args.title.get_inner(),
    });

    WorkersKvNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        title: crate::into_domain(result.title),
    }
}
