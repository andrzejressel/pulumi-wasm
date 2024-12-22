//! Provides a resource to manage a Cloudflare Workers KV Pair.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = workers_kv::create(
//!         "example",
//!         WorkersKvArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .key("test-key")
//!             .namespace_id("${exampleNs.id}")
//!             .value("test value")
//!             .build_struct(),
//!     );
//!     let exampleNs = workers_kv_namespace::create(
//!         "exampleNs",
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
//! $ pulumi import cloudflare:index/workersKv:WorkersKv example <account_id>/<namespace_id>/<key_name>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkersKvArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    #[builder(into)]
    pub value: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersKvResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Name of the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub key: pulumi_wasm_rust::Output<String>,
    /// The ID of the Workers KV namespace in which you want to create the KV pair. **Modifying this attribute will force creation of a new resource.**
    pub namespace_id: pulumi_wasm_rust::Output<String>,
    /// Value of the KV pair.
    pub value: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(
    name: &str,
    args: WorkersKvArgs
) -> WorkersKvResult {

    let result = crate::bindings::pulumi::cloudflare::workers_kv::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::workers_kv::Args {
                account_id: &args.account_id.get_inner(),
                key: &args.key.get_inner(),
                namespace_id: &args.namespace_id.get_inner(),
                value: &args.value.get_inner(),
        }
    );

    WorkersKvResult {
        account_id: crate::into_domain(result.account_id),
        key: crate::into_domain(result.key),
        namespace_id: crate::into_domain(result.namespace_id),
        value: crate::into_domain(result.value),
    }
}
