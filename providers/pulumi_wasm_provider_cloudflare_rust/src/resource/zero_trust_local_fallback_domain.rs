//! Provides a Cloudflare Fallback Domain resource. Fallback domains are
//! used to ignore DNS requests to a given list of domains. These DNS
//! requests will be passed back to other DNS servers configured on
//! existing network interfaces on the device.

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustLocalFallbackDomainArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    #[builder(into)]
    pub domains: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustLocalFallbackDomainDomain>>,
    /// The settings policy for which to configure this fallback domain policy.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustLocalFallbackDomainResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub domains: pulumi_wasm_rust::Output<Vec<crate::types::ZeroTrustLocalFallbackDomainDomain>>,
    /// The settings policy for which to configure this fallback domain policy.
    pub policy_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustLocalFallbackDomainArgs) -> ZeroTrustLocalFallbackDomainResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_local_fallback_domain::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_local_fallback_domain::Args {
        account_id: &args.account_id.get_inner(),
        domains: &args.domains.get_inner(),
        policy_id: &args.policy_id.get_inner(),
    });

    ZeroTrustLocalFallbackDomainResult {
        account_id: crate::into_domain(result.account_id),
        domains: crate::into_domain(result.domains),
        policy_id: crate::into_domain(result.policy_id),
    }
}