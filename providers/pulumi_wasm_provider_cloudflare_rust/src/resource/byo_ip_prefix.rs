//! Provides the ability to manage Bring-Your-Own-IP prefixes (BYOIP)
//! which are used with or without Magic Transit.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = byo_ip_prefix::create(
//!         "example",
//!         ByoIpPrefixArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .advertisement("on")
//!             .description("Example IP Prefix")
//!             .prefix_id("d41d8cd98f00b204e9800998ecf8427e")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/byoIpPrefix:ByoIpPrefix example <account_id>/<prefix_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ByoIpPrefixArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub advertisement: pulumi_wasm_rust::Output<Option<String>>,
    /// Description of the BYO IP prefix.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    #[builder(into)]
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

pub struct ByoIpPrefixResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not the prefix shall be announced. A prefix can be activated or deactivated once every 15 minutes (attempting more regular updates will trigger rate limiting). Available values: `on`, `off`.
    pub advertisement: pulumi_wasm_rust::Output<String>,
    /// Description of the BYO IP prefix.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The assigned Bring-Your-Own-IP prefix ID. **Modifying this attribute will force creation of a new resource.**
    pub prefix_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ByoIpPrefixArgs) -> ByoIpPrefixResult {

    let result = crate::bindings::pulumi::cloudflare::byo_ip_prefix::invoke(name, &crate::bindings::pulumi::cloudflare::byo_ip_prefix::Args {
        account_id: &args.account_id.get_inner(),
        advertisement: &args.advertisement.get_inner(),
        description: &args.description.get_inner(),
        prefix_id: &args.prefix_id.get_inner(),
    });

    ByoIpPrefixResult {
        account_id: crate::into_domain(result.account_id),
        advertisement: crate::into_domain(result.advertisement),
        description: crate::into_domain(result.description),
        prefix_id: crate::into_domain(result.prefix_id),
    }
}
