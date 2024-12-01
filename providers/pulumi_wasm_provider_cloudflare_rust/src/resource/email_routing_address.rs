//! The [Email Routing Address](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#destination-addresses) resource allows you to manage Cloudflare Email Routing Destination Addresses.
//! 
//! ## Example Usage
//! 
//! <!--Start PulumiCodeChooser -->
//! ```rust
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let example = email_routing_address::create(
//!         "example",
//!         EmailRoutingAddressArgs::builder()
//!             .account_id("f037e56e89293a057740de681ac9abbe")
//!             .email("user@example.com")
//!             .build_struct(),
//!     );
//! }
//! ```
//! <!--End PulumiCodeChooser -->
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/emailRoutingAddress:EmailRoutingAddress example <account_id>/<email_routing_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct EmailRoutingAddressArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    #[builder(into)]
    pub email: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingAddressResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been created.
    pub created: pulumi_wasm_rust::Output<String>,
    /// The contact email address of the user.
    pub email: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been modified.
    pub modified: pulumi_wasm_rust::Output<String>,
    /// Destination address identifier.
    pub tag: pulumi_wasm_rust::Output<String>,
    /// The date and time the destination address has been verified. Null means not verified yet.
    pub verified: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: EmailRoutingAddressArgs) -> EmailRoutingAddressResult {

    let result = crate::bindings::pulumi::cloudflare::email_routing_address::invoke(name, &crate::bindings::pulumi::cloudflare::email_routing_address::Args {
        account_id: &args.account_id.get_inner(),
        email: &args.email.get_inner(),
    });

    EmailRoutingAddressResult {
        account_id: crate::into_domain(result.account_id),
        created: crate::into_domain(result.created),
        email: crate::into_domain(result.email),
        modified: crate::into_domain(result.modified),
        tag: crate::into_domain(result.tag),
        verified: crate::into_domain(result.verified),
    }
}
