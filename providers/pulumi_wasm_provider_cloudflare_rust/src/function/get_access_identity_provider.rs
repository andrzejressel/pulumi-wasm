//! Use this data source to lookup a single [Access Identity Provider](https://developers.cloudflare.com/cloudflare-one/identity/idp-integration) by name.
//! 
//! ## Example Usage
//! 
//! ```ignore
//! use pulumi_wasm_rust::Output;
//! use pulumi_wasm_rust::{add_export, pulumi_main};
//! #[pulumi_main]
//! fn test_main() -> Result<(), Error> {
//!     let exampleAccessApplication = access_application::create(
//!         "exampleAccessApplication",
//!         AccessApplicationArgs::builder()
//!             .allowed_idps(vec!["${example.id}",])
//!             .auto_redirect_to_identity(true)
//!             .domain("name.example.com")
//!             .name("name")
//!             .session_duration("24h")
//!             .type_("self_hosted")
//!             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
//!             .build_struct(),
//!     );
//! }
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct GetAccessIdentityProviderArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Access Identity Provider name to search for.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetAccessIdentityProviderResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider name to search for.
    pub name: pulumi_wasm_rust::Output<String>,
    /// Access Identity Provider Type.
    pub type_: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetAccessIdentityProviderArgs
) -> GetAccessIdentityProviderResult {

    let result = crate::bindings::pulumi::cloudflare::get_access_identity_provider::invoke(
        &crate::bindings::pulumi::cloudflare::get_access_identity_provider::Args {
                account_id: &args.account_id.get_inner(),
                name: &args.name.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetAccessIdentityProviderResult {
        account_id: crate::into_domain(result.account_id),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        type_: crate::into_domain(result.type_),
        zone_id: crate::into_domain(result.zone_id),
    }
}
