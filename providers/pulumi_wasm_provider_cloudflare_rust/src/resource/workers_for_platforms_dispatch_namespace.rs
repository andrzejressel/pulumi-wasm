//! The [Workers for Platforms](https://developers.cloudflare.com/cloudflare-for-platforms/workers-for-platforms/) resource allows you
//! to manage Cloudflare Workers for Platforms dispatch namespaces.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:WorkersForPlatformsDispatchNamespace
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: example-namespace
//!   customerWorker1:
//!     type: cloudflare:WorkersScript
//!     name: customer_worker_1
//!     properties:
//!       accountId: f037e56e89293a057740de681ac9abbe
//!       name: customer-worker-1
//!       content:
//!         fn::invoke:
//!           Function: std:file
//!           Arguments:
//!             input: script.js
//!           Return: result
//!       dispatchNamespace: ${example.name}
//!       tags:
//!         - free
//! ```
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/workersForPlatformsDispatchNamespace:WorkersForPlatformsDispatchNamespace example <account_id>/<namespace_name>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct WorkersForPlatformsDispatchNamespaceArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Workers for Platforms namespace.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct WorkersForPlatformsDispatchNamespaceResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the Workers for Platforms namespace.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: WorkersForPlatformsDispatchNamespaceArgs) -> WorkersForPlatformsDispatchNamespaceResult {

    let result = crate::bindings::pulumi::cloudflare::workers_for_platforms_dispatch_namespace::invoke(name, &crate::bindings::pulumi::cloudflare::workers_for_platforms_dispatch_namespace::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    WorkersForPlatformsDispatchNamespaceResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
