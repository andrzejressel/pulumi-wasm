//! The [Risk Behavior](https://developers.cloudflare.com/cloudflare-one/insights/risk-score/) resource allows you to configure Cloudflare Risk Behaviors for an account.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustRiskBehaviorArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Zero Trust risk behaviors configured on this account
    #[builder(into, default)]
    pub behaviors: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustRiskBehaviorBehavior>>>,
}

pub struct ZeroTrustRiskBehaviorResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Zero Trust risk behaviors configured on this account
    pub behaviors: pulumi_wasm_rust::Output<Option<Vec<crate::types::ZeroTrustRiskBehaviorBehavior>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustRiskBehaviorArgs) -> ZeroTrustRiskBehaviorResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_risk_behavior::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_risk_behavior::Args {
        account_id: &args.account_id.get_inner(),
        behaviors: &args.behaviors.get_inner(),
    });

    ZeroTrustRiskBehaviorResult {
        account_id: crate::into_domain(result.account_id),
        behaviors: crate::into_domain(result.behaviors),
    }
}
