#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayPolicyArgs {
    /// The account identifier to target for the resource.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    #[builder(into)]
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    #[builder(into)]
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    #[builder(into, default)]
    pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
    /// Indicator of rule enablement.
    #[builder(into, default)]
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    #[builder(into, default)]
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    #[builder(into, default)]
    pub identity: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the teams rule.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    #[builder(into)]
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    #[builder(into, default)]
    pub rule_settings: pulumi_wasm_rust::Output<
        Option<super::types::ZeroTrustGatewayPolicyRuleSettings>,
    >,
    /// The wirefilter expression to be used for traffic matching.
    #[builder(into, default)]
    pub traffic: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct ZeroTrustGatewayPolicyResult {
    /// The account identifier to target for the resource.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
    pub action: pulumi_wasm_rust::Output<String>,
    /// The description of the teams rule.
    pub description: pulumi_wasm_rust::Output<String>,
    /// The wirefilter expression to be used for device_posture check matching.
    pub device_posture: pulumi_wasm_rust::Output<String>,
    /// Indicator of rule enablement.
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    /// The protocol or layer to evaluate the traffic and identity expressions.
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    /// The wirefilter expression to be used for identity matching.
    pub identity: pulumi_wasm_rust::Output<String>,
    /// The name of the teams rule.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The evaluation precedence of the teams rule.
    pub precedence: pulumi_wasm_rust::Output<i32>,
    /// Additional rule settings.
    pub rule_settings: pulumi_wasm_rust::Output<
        super::types::ZeroTrustGatewayPolicyRuleSettings,
    >,
    /// The wirefilter expression to be used for traffic matching.
    pub traffic: pulumi_wasm_rust::Output<String>,
    pub version: pulumi_wasm_rust::Output<i32>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(
    name: &str,
    args: ZeroTrustGatewayPolicyArgs,
) -> ZeroTrustGatewayPolicyResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let action_binding = args.action.get_inner();
    let description_binding = args.description.get_inner();
    let device_posture_binding = args.device_posture.get_inner();
    let enabled_binding = args.enabled.get_inner();
    let filters_binding = args.filters.get_inner();
    let identity_binding = args.identity.get_inner();
    let name_binding = args.name.get_inner();
    let precedence_binding = args.precedence.get_inner();
    let rule_settings_binding = args.rule_settings.get_inner();
    let traffic_binding = args.traffic.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/zeroTrustGatewayPolicy:ZeroTrustGatewayPolicy".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "action".into(),
                value: &action_binding,
            },
            register_interface::ObjectField {
                name: "description".into(),
                value: &description_binding,
            },
            register_interface::ObjectField {
                name: "devicePosture".into(),
                value: &device_posture_binding,
            },
            register_interface::ObjectField {
                name: "enabled".into(),
                value: &enabled_binding,
            },
            register_interface::ObjectField {
                name: "filters".into(),
                value: &filters_binding,
            },
            register_interface::ObjectField {
                name: "identity".into(),
                value: &identity_binding,
            },
            register_interface::ObjectField {
                name: "name".into(),
                value: &name_binding,
            },
            register_interface::ObjectField {
                name: "precedence".into(),
                value: &precedence_binding,
            },
            register_interface::ObjectField {
                name: "ruleSettings".into(),
                value: &rule_settings_binding,
            },
            register_interface::ObjectField {
                name: "traffic".into(),
                value: &traffic_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "action".into() },
            register_interface::ResultField { name : "description".into() },
            register_interface::ResultField { name : "devicePosture".into() },
            register_interface::ResultField { name : "enabled".into() },
            register_interface::ResultField { name : "filters".into() },
            register_interface::ResultField { name : "identity".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "precedence".into() },
            register_interface::ResultField { name : "ruleSettings".into() },
            register_interface::ResultField { name : "traffic".into() },
            register_interface::ResultField { name : "version".into() },
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
    ZeroTrustGatewayPolicyResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        action: into_domain(hashmap.remove("action").unwrap()),
        description: into_domain(hashmap.remove("description").unwrap()),
        device_posture: into_domain(hashmap.remove("devicePosture").unwrap()),
        enabled: into_domain(hashmap.remove("enabled").unwrap()),
        filters: into_domain(hashmap.remove("filters").unwrap()),
        identity: into_domain(hashmap.remove("identity").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        precedence: into_domain(hashmap.remove("precedence").unwrap()),
        rule_settings: into_domain(hashmap.remove("ruleSettings").unwrap()),
        traffic: into_domain(hashmap.remove("traffic").unwrap()),
        version: into_domain(hashmap.remove("version").unwrap()),
    }
}