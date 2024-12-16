//! The [Risk Score Integration](https://developers.cloudflare.com/cloudflare-one/insights/risk-score/#send-risk-score-to-okta) resource allows you to transmit changes in User Risk Score to a specified vendor such as Okta.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustRiskScoreIntegrationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this integration is enabled. If disabled, no risk changes will be exported to the third-party.
    #[builder(into, default)]
    pub active: pulumi_wasm_rust::Output<Option<bool>>,
    /// The type of integration, e.g. 'Okta'. Full list of allowed values can be found here: https://developers.cloudflare.com/api/operations/dlp-zt-risk-score-integration-create#request-body
    #[builder(into)]
    pub integration_type: pulumi_wasm_rust::Output<String>,
    /// A reference id that can be supplied by the client. Currently this should be set to the Access-Okta IDP ID (a UUIDv4). If omitted, a random UUIDv4 is used. https://developers.cloudflare.com/api/operations/access-identity-providers-get-an-access-identity-provider
    #[builder(into, default)]
    pub reference_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The base url of the tenant, e.g. 'https://tenant.okta.com'. Must be your Okta Tenant URL and not your custom domain.
    #[builder(into)]
    pub tenant_url: pulumi_wasm_rust::Output<String>,
}

pub struct ZeroTrustRiskScoreIntegrationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Whether this integration is enabled. If disabled, no risk changes will be exported to the third-party.
    pub active: pulumi_wasm_rust::Output<bool>,
    /// The type of integration, e.g. 'Okta'. Full list of allowed values can be found here: https://developers.cloudflare.com/api/operations/dlp-zt-risk-score-integration-create#request-body
    pub integration_type: pulumi_wasm_rust::Output<String>,
    /// A reference id that can be supplied by the client. Currently this should be set to the Access-Okta IDP ID (a UUIDv4). If omitted, a random UUIDv4 is used. https://developers.cloudflare.com/api/operations/access-identity-providers-get-an-access-identity-provider
    pub reference_id: pulumi_wasm_rust::Output<String>,
    /// The base url of the tenant, e.g. 'https://tenant.okta.com'. Must be your Okta Tenant URL and not your custom domain.
    pub tenant_url: pulumi_wasm_rust::Output<String>,
    /// The URL for the Shared Signals Framework configuration, e.g. '/.well-known/sse-configuration/{integration*uuid}/'. https://openid.net/specs/openid-sse-framework-1*0.html#rfc.section.6.2.1
    pub well_known_url: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustRiskScoreIntegrationArgs) -> ZeroTrustRiskScoreIntegrationResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_risk_score_integration::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_risk_score_integration::Args {
        account_id: &args.account_id.get_inner(),
        active: &args.active.get_inner(),
        integration_type: &args.integration_type.get_inner(),
        reference_id: &args.reference_id.get_inner(),
        tenant_url: &args.tenant_url.get_inner(),
    });

    ZeroTrustRiskScoreIntegrationResult {
        account_id: crate::into_domain(result.account_id),
        active: crate::into_domain(result.active),
        integration_type: crate::into_domain(result.integration_type),
        reference_id: crate::into_domain(result.reference_id),
        tenant_url: crate::into_domain(result.tenant_url),
        well_known_url: crate::into_domain(result.well_known_url),
    }
}
