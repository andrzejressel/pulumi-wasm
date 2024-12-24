#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ApiTokenArgs {
    /// Conditions under which the token should be considered valid.
    #[builder(into, default)]
    pub condition: pulumi_wasm_rust::Output<Option<super::types::ApiTokenCondition>>,
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
    pub policies: pulumi_wasm_rust::Output<Vec<super::types::ApiTokenPolicy>>,
}
pub struct ApiTokenResult {
    /// Conditions under which the token should be considered valid.
    pub condition: pulumi_wasm_rust::Output<Option<super::types::ApiTokenCondition>>,
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
    pub policies: pulumi_wasm_rust::Output<Vec<super::types::ApiTokenPolicy>>,
    pub status: pulumi_wasm_rust::Output<String>,
    /// The value of the API Token.
    pub value: pulumi_wasm_rust::Output<String>,
}
///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: ApiTokenArgs) -> ApiTokenResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let condition_binding = args.condition.get_inner();
    let expires_on_binding = args.expires_on.get_inner();
    let name_binding = args.name.get_inner();
    let not_before_binding = args.not_before.get_inner();
    let policies_binding = args.policies.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/apiToken:ApiToken".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "condition".into(),
                value: &condition_binding,
            },
            register_interface::ObjectField {
                name: "expiresOn".into(),
                value: &expires_on_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "notBefore".into(),
                value: &not_before_binding,
            },
            register_interface::ObjectField {
                name: "policies".into(),
                value: &policies_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "condition".into() },
            register_interface::ResultField { name : "expiresOn".into() },
            register_interface::ResultField { name : "issuedOn".into() },
            register_interface::ResultField { name : "modifiedOn".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "notBefore".into() },
            register_interface::ResultField { name : "policies".into() },
            register_interface::ResultField { name : "status".into() },
            register_interface::ResultField { name : "value".into() },
        ],
    };
    fn into_domain<F: serde::Serialize>(output: WitOutput) -> Output<F> {
        unsafe { Output::<F>::new_from_handle(output) }
    }
    let o = register_interface::register(&request);
    let mut hashmap: HashMap<String, _> = o
        .fields
        .into_iter()
        .map(|f| (f.name, f.output))
        .collect();
    ApiTokenResult {
        condition: into_domain(hashmap.remove("condition").unwrap()),
        expires_on: into_domain(hashmap.remove("expiresOn").unwrap()),
        issued_on: into_domain(hashmap.remove("issuedOn").unwrap()),
        modified_on: into_domain(hashmap.remove("modifiedOn").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        not_before: into_domain(hashmap.remove("notBefore").unwrap()),
        policies: into_domain(hashmap.remove("policies").unwrap()),
        status: into_domain(hashmap.remove("status").unwrap()),
        value: into_domain(hashmap.remove("value").unwrap()),
    }
}
