//! Access Keys Configuration defines the rotation policy for the keys
//! that access will use to sign data.

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustKeyAccessKeyConfigurationArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Number of days to trigger a rotation of the keys.
    #[builder(into, default)]
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<Option<i32>>,
}

pub struct ZeroTrustKeyAccessKeyConfigurationResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// Number of days to trigger a rotation of the keys.
    pub key_rotation_interval_days: pulumi_wasm_rust::Output<i32>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustKeyAccessKeyConfigurationArgs) -> ZeroTrustKeyAccessKeyConfigurationResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_key_access_key_configuration::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_key_access_key_configuration::Args {
        account_id: &args.account_id.get_inner(),
        key_rotation_interval_days: &args.key_rotation_interval_days.get_inner(),
    });

    ZeroTrustKeyAccessKeyConfigurationResult {
        account_id: crate::into_domain(result.account_id),
        key_rotation_interval_days: crate::into_domain(result.key_rotation_interval_days),
    }
}
