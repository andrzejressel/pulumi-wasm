pub struct ApiTokenArgs {
    pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
    pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub not_before: pulumi_wasm_rust::Output<Option<String>>,
    pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
}

pub struct ApiTokenResult {
    pub condition: pulumi_wasm_rust::Output<Option<crate::types::ApiTokenCondition>>,
    pub expires_on: pulumi_wasm_rust::Output<Option<String>>,
    pub issued_on: pulumi_wasm_rust::Output<String>,
    pub modified_on: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub not_before: pulumi_wasm_rust::Output<Option<String>>,
    pub policies: pulumi_wasm_rust::Output<Vec<crate::types::ApiTokenPolicy>>,
    pub status: pulumi_wasm_rust::Output<String>,
    pub value: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ApiTokenArgs) -> ApiTokenResult {
    let result = crate::bindings::pulumi::cloudflare::api_token::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::api_token::Args {
            condition: args.condition.get_inner(),
            expires_on: args.expires_on.get_inner(),
            name: args.name.get_inner(),
            not_before: args.not_before.get_inner(),
            policies: args.policies.get_inner(),
        },
    );

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
