//! Access Service Tokens are used for service-to-service communication
//! when an application is behind Cloudflare Access.
//! 
//! ## Import
//! 
//! If you are importing an Access Service Token you will not have the
//! 
//! client_secret available in the state for use. The client_secret is only
//! 
//! available once, at creation. In most cases, it is better to just create a new
//! 
//! resource should you need to reference it in other resources.
//! 
//! ```sh
//! $ pulumi import cloudflare:index/zeroTrustAccessServiceToken:ZeroTrustAccessServiceToken example <account_id>/<service_token_id>
//! ```
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustAccessServiceTokenArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    #[builder(into, default)]
    pub duration: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct ZeroTrustAccessServiceTokenResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// Client ID associated with the Service Token. **Modifying this attribute will force creation of a new resource.**
    pub client_id: pulumi_wasm_rust::Output<String>,
    /// A secret for interacting with Access protocols. **Modifying this attribute will force creation of a new resource.**
    pub client_secret: pulumi_wasm_rust::Output<String>,
    /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
    pub duration: pulumi_wasm_rust::Output<String>,
    /// Date when the token expires.
    pub expires_at: pulumi_wasm_rust::Output<String>,
    pub min_days_for_renewal: pulumi_wasm_rust::Output<Option<i32>>,
    /// Friendly name of the token's intent.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ZeroTrustAccessServiceTokenArgs) -> ZeroTrustAccessServiceTokenResult {

    let result = crate::bindings::pulumi::cloudflare::zero_trust_access_service_token::invoke(name, &crate::bindings::pulumi::cloudflare::zero_trust_access_service_token::Args {
        account_id: &args.account_id.get_inner(),
        duration: &args.duration.get_inner(),
        min_days_for_renewal: &args.min_days_for_renewal.get_inner(),
        name: &args.name.get_inner(),
        zone_id: &args.zone_id.get_inner(),
    });

    ZeroTrustAccessServiceTokenResult {
        account_id: crate::into_domain(result.account_id),
        client_id: crate::into_domain(result.client_id),
        client_secret: crate::into_domain(result.client_secret),
        duration: crate::into_domain(result.duration),
        expires_at: crate::into_domain(result.expires_at),
        min_days_for_renewal: crate::into_domain(result.min_days_for_renewal),
        name: crate::into_domain(result.name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
