//! Provides a resource to customize the pages your end users will see
//! when trying to reach applications behind Cloudflare Access.

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessTagArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps associated with the tag.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub app_count: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the Access Tag.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessTagResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`. **Modifying this attribute will force creation of a new resource.**
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Number of apps associated with the tag.
    pub app_count: pulumi_wasm_rust::Output<i32>,
    /// Friendly name of the Access Tag.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`. **Modifying this attribute will force creation of a new resource.**
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustAccessTagArgs) -> ZeroTrustAccessTagResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_tag::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_access_tag::Args {
        account_id: &args.account_id.get_inner(),
        app_count: &args.app_count.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZeroTrustAccessTagResult {
        account_id: crate::into_domain(result.account_id),
        app_count: crate::into_domain(result.app_count),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}