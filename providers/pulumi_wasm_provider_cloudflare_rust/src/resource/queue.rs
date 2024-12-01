//! Provides the ability to manage Cloudflare Workers Queue features.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = queue::create(
//!         "example",
//!         QueueArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .name("my-queue")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/queue:Queue example <account_id>/<queue_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct QueueArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the queue.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
}

pub struct QueueResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The name of the queue.
    pub name: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: QueueArgs) -> QueueResult {

    let result = crate::bindings::pulumi::cloudflare::queue::invoke(name, &crate::bindings::pulumi::cloudflare::queue::Args {
        account_id: &args.account_id.get_inner(),
        name: &args.name.get_inner(),
    });

    QueueResult {
        account_id: crate::into_domain(result.account_id),
        name: crate::into_domain(result.name),
    }
}
