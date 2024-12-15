//! Use this data source to lookup a single [Access Application](https://developers.cloudflare.com/cloudflare-one/applications/)

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetZeroTrustAccessApplicationArgs {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub domain: pulumi_wasm_rust::Output<Option<String>>,
    /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub name: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct GetZeroTrustAccessApplicationResult {
    /// The account identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Application Audience (AUD) Tag of the application.
    pub aud: pulumi_wasm_rust::Output<String>,
    /// The primary hostname and path that Access will secure. Must provide only one of `name`, `domain`.
    pub domain: pulumi_wasm_rust::Output<String>,
    /// The provider-assigned unique ID for this managed resource.
    pub id: pulumi_wasm_rust::Output<String>,
    /// Friendly name of the Access Application. Must provide only one of `name`, `domain`.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Must provide only one of `zone_id`, `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn invoke(
    args: GetZeroTrustAccessApplicationArgs
) -> GetZeroTrustAccessApplicationResult {

    let result = crate::bindings::pulumi::cloudflare::get_zero_trust_access_application::invoke(
        &crate::bindings::pulumi::cloudflare::get_zero_trust_access_application::Args {
                account_id: &args.account_id.get_inner(),
                domain: &args.domain.get_inner(),
                name: &args.name.get_inner(),
                zone_id: &args.zone_id.get_inner(),
        }
    );

    GetZeroTrustAccessApplicationResult {
        account_id: crate::into_domain(result.account_id),
        aud: crate::into_domain(result.aud),
        domain: crate::into_domain(result.domain),
        id: crate::into_domain(result.id),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
