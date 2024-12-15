//! Provides a resource which manages Cloudflare API tokens.
//! 
//! Read more about permission groups and their applicable scopes in the
//! [developer documentation](https://developers.cloudflare.com/api/tokens/create/permissions).
//! 

#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiTokenArgs {
    /// Conditions under which the token should be considered valid.
    #[builder(into, default)]
    pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
    /// The expiration time on or after which the token MUST NOT be accepted for processing.
    #[builder(into, default)]
    pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
    /// Name of the API Token.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The time before which the token MUST NOT be accepted for processing.
    #[builder(into, default)]
    pub not_before: pulumi_wasm_rust::Output<Option<String>>,
    /// Permissions policy. Multiple policy blocks can be defined.
    #[builder(into)]
    pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
}

pub struct ApiTokenResult {
    /// Conditions under which the token should be considered valid.
    pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
    /// The expiration time on or after which the token MUST NOT be accepted for processing.
    pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
    /// Timestamp of when the token was issued.
    pub issued_on: pulumi_wasm_rust::Output<String>,
    /// Timestamp of when the token was last modified.
    pub modified_on: pulumi_wasm_rust::Output<String>,
    /// Name of the API Token.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The time before which the token MUST NOT be accepted for processing.
    pub not_before: pulumi_wasm_rust::Output<Option<String>>,
    /// Permissions policy. Multiple policy blocks can be defined.
    pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
    pub status: pulumi_wasm_rust::Output<String>,
    /// The value of the API Token.
    pub value: pulumi_wasm_rust::Output<String>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiTokenArgs) -> ApiTokenResult {

    let result = crate::bindings::pulumi::cloudflare::api_token::invoke(name, &crate::bindings::pulumi::cloudflare::api_token::Args {
        condition: &args.condition.get_inner(),
        expires_on: &args.expires_on.get_inner(),
        name: &args.name.get_inner(),
        not_before: &args.not_before.get_inner(),
        policies: &args.policies.get_inner(),
    });

    ApiTokenResult {
        condition: crate::into_domain(result.condition),
        expires_on: crate::into_domain(result.expires_on),
        issued_on: crate::into_domain(result.issued_on),
        modified_on: crate::into_domain(result.modified_on),
        name: crate::into_domain(result.name),
        not_before: crate::into_domain(result.not_before),
        policies: crate::into_domain(result.policies),
        status: crate::into_domain(result.status),
        value: crate::into_domain(result.value),
    }
}
