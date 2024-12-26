#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct MagicFirewallRulesetArgs {
    /// The ID of the account where the ruleset is being created.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    #[builder(into, default)]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default)]
    pub rules: pulumi_wasm_rust::Output<
        Option<Vec<std::collections::HashMap<String, String>>>,
    >,
}
pub struct MagicFirewallRulesetResult {
    /// The ID of the account where the ruleset is being created.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    pub name: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<
        Option<Vec<std::collections::HashMap<String, String>>>,
    >,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: MagicFirewallRulesetArgs) -> MagicFirewallRulesetResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let description_binding = args.description.get_inner();
    let name_binding = args.name.get_inner();
    let rules_binding = args.rules.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "rules".into(),
                value: &rules_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "rules".into() },
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
    MagicFirewallRulesetResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        rules: into_domain(hashmap.remove("rules").unwrap()),
    }
}
