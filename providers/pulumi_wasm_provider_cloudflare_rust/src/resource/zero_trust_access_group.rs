//! Provides a Cloudflare Access Group resource. Access Groups are used
//! in conjunction with Access Policies to restrict access to a
//! particular resource based on group membership.
//! 
//! > It's required that an `account_id` or `zone_id` is provided and in
//!    most cases using either is fine. However, if you're using a scoped
//!    access token, you must provide the argument that matches the token's
//!    scope. For example, an access token that is scoped to the "example.com"
//!    zone needs to use the `zone_id` argument.
//! 
//! ## Import
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustAccessGroup:ZeroTrustAccessGroup example <account_id>/<group_id>
//! ```
//! 

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessGroupArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessGroupExclude>>>,
    #[builder(into)]
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustAccessGroupInclude>>,
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessGroupRequire>>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessGroupResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub excludes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessGroupExclude>>>,
    pub includes: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustAccessGroupInclude>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub requires: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustAccessGroupRequire>>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustAccessGroupArgs) -> ZeroTrustAccessGroupResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_group::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_access_group::Args {
        account_id: &args.account_id.get_inner(),
        excludes: &args.excludes.get_inner(),
        includes: &args.includes.get_inner(),
        name: &args.name.get_inner(),
        requires: &args.requires.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZeroTrustAccessGroupResult {
        account_id: crate::into_domain(result.account_id),
        excludes: crate::into_domain(result.excludes),
        includes: crate::into_domain(result.includes),
        name: crate::into_domain(result.name),
        requires: crate::into_domain(result.requires),
        zone_id: crate::into_domain(result.zone_id),
    }
}

