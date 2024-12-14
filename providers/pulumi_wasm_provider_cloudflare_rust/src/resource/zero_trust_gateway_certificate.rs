//! Provides a Cloudflare Teams Gateway Certificate resource. A Teams Certificate can
//! be specified for Gateway TLS interception and block pages.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayCertificateArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub activate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub custom: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub gateway_managed: pulumi_wasm_rust::Output<Option<bool>>,
    /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub validity_period_days: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct ZeroTrustGatewayCertificateResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether or not to activate a certificate. A certificate must be activated to use in Gateway certificate settings. Defaults to `false`.
    pub activate: pulumi_wasm_rust::Output<Option<bool>>,
    /// The deployment status of the certificate on the edge Available values: `IP`, `SERIAL`, `URL`, `DOMAIN`, `EMAIL`.
    pub binding_status: pulumi_wasm_rust::Output<String>,
    pub created_at: pulumi_wasm_rust::Output<String>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    pub custom: pulumi_wasm_rust::Output<Option<bool>>,
    pub expires_on: pulumi_wasm_rust::Output<String>,
    /// The type of certificate (custom or Gateway-managed). Must provide only one of `custom`, `gateway_managed`.
    pub gateway_managed: pulumi_wasm_rust::Output<Option<bool>>,
    /// Whether the certificate is in use by Gateway for TLS interception and the block page.
    pub in_use: pulumi_wasm_rust::Output<bool>,
    pub qs_pack_id: pulumi_wasm_rust::Output<String>,
    pub uploaded_on: pulumi_wasm_rust::Output<String>,
    /// Number of days the generated certificate will be valid, minimum 1 day and maximum 30 years. Defaults to 5 years. Defaults to `1826`. Required when using `gateway_managed`. Conflicts with `custom`. **Modifying this attribute will force creation of a new resource.**
    pub validity_period_days: pulumi_wasm_rust::Output<Option<i32>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustGatewayCertificateArgs) -> ZeroTrustGatewayCertificateResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_gateway_certificate::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_gateway_certificate::Args {
        account_id: &args.account_id.get_inner(),
        activate: &args.activate.get_inner(),
        custom: &args.custom.get_inner(),
        gateway_managed: &args.gateway_managed.get_inner(),
        validity_period_days: &args.validity_period_days.get_inner(),
    });

    ZeroTrustGatewayCertificateResult {
        account_id: crate::into_domain(result.account_id),
        activate: crate::into_domain(result.activate),
        binding_status: crate::into_domain(result.binding_status),
        created_at: crate::into_domain(result.created_at),
        custom: crate::into_domain(result.custom),
        expires_on: crate::into_domain(result.expires_on),
        gateway_managed: crate::into_domain(result.gateway_managed),
        in_use: crate::into_domain(result.in_use),
        qs_pack_id: crate::into_domain(result.qs_pack_id),
        uploaded_on: crate::into_domain(result.uploaded_on),
        validity_period_days: crate::into_domain(result.validity_period_days),
    }
}
