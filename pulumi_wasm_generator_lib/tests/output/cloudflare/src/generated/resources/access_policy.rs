#[derive(bon::Builder, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyArgs {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    #[builder(into, default)]
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    pub application_id: pulumi_wasm_rust::Output<Option<String>>,
    #[builder(into, default)]
    pub approval_groups: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyApprovalGroup>>,
    >,
    #[builder(into, default)]
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
    #[builder(into, default)]
    pub connection_rules: pulumi_wasm_rust::Output<
        Option<super::types::AccessPolicyConnectionRules>,
    >,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    #[builder(into)]
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default)]
    pub excludes: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyExclude>>,
    >,
    /// A series of access conditions, see Access Groups.
    #[builder(into)]
    pub includes: pulumi_wasm_rust::Output<Vec<super::types::AccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    #[builder(into, default)]
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application. Required when using `application_id`.
    #[builder(into, default)]
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    #[builder(into, default)]
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    #[builder(into, default)]
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    #[builder(into, default)]
    pub requires: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyRequire>>,
    >,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    #[builder(into, default)]
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    #[builder(into, default)]
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
pub struct AccessPolicyResult {
    /// The account identifier to target for the resource. Conflicts with `zone_id`.
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
    pub application_id: pulumi_wasm_rust::Output<Option<String>>,
    pub approval_groups: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyApprovalGroup>>,
    >,
    pub approval_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
    pub connection_rules: pulumi_wasm_rust::Output<
        Option<super::types::AccessPolicyConnectionRules>,
    >,
    /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
    pub decision: pulumi_wasm_rust::Output<String>,
    /// A series of access conditions, see Access Groups.
    pub excludes: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyExclude>>,
    >,
    /// A series of access conditions, see Access Groups.
    pub includes: pulumi_wasm_rust::Output<Vec<super::types::AccessPolicyInclude>>,
    /// Require this application to be served in an isolated browser for users matching this policy.
    pub isolation_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// Friendly name of the Access Policy.
    pub name: pulumi_wasm_rust::Output<String>,
    /// The unique precedence for policies on a single application. Required when using `application_id`.
    pub precedence: pulumi_wasm_rust::Output<Option<i32>>,
    /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
    pub purpose_justification_prompt: pulumi_wasm_rust::Output<Option<String>>,
    /// Whether to prompt the user for a justification for accessing the resource.
    pub purpose_justification_required: pulumi_wasm_rust::Output<Option<bool>>,
    /// A series of access conditions, see Access Groups.
    pub requires: pulumi_wasm_rust::Output<
        Option<Vec<super::types::AccessPolicyRequire>>,
    >,
    /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
    pub session_duration: pulumi_wasm_rust::Output<Option<String>>,
    /// The zone identifier to target for the resource. Conflicts with `account_id`.
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports)]
pub fn create(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
    use pulumi_wasm_wit::client_bindings::component::pulumi_wasm::output_interface::Output as WitOutput;
    use pulumi_wasm_rust::Output;
    use std::collections::HashMap;
    let account_id_binding = args.account_id.get_inner();
    let application_id_binding = args.application_id.get_inner();
    let approval_groups_binding = args.approval_groups.get_inner();
    let approval_required_binding = args.approval_required.get_inner();
    let connection_rules_binding = args.connection_rules.get_inner();
    let decision_binding = args.decision.get_inner();
    let excludes_binding = args.excludes.get_inner();
    let includes_binding = args.includes.get_inner();
    let isolation_required_binding = args.isolation_required.get_inner();
    let name_binding = args.name.get_inner();
    let precedence_binding = args.precedence.get_inner();
    let purpose_justification_prompt_binding = args
        .purpose_justification_prompt
        .get_inner();
    let purpose_justification_required_binding = args
        .purpose_justification_required
        .get_inner();
    let requires_binding = args.requires.get_inner();
    let session_duration_binding = args.session_duration.get_inner();
    let zone_id_binding = args.zone_id.get_inner();
    let request = register_interface::RegisterResourceRequest {
        type_: "cloudflare:index/accessPolicy:AccessPolicy".into(),
        name: name.to_string(),
        object: Vec::from([
            register_interface::ObjectField {
                name: "accountId".into(),
                value: &account_id_binding,
            },
            register_interface::ObjectField {
                name: "applicationId".into(),
                value: &application_id_binding,
            },
            register_interface::ObjectField {
                name: "approvalGroups".into(),
                value: &approval_groups_binding,
            },
            register_interface::ObjectField {
                name: "approvalRequired".into(),
                value: &approval_required_binding,
            },
            register_interface::ObjectField {
                name: "connectionRules".into(),
                value: &connection_rules_binding,
            },
            register_interface::ObjectField {
                name: "decision".into(),
                value: &decision_binding,
            },
            register_interface::ObjectField {
                name: "excludes".into(),
                value: &excludes_binding,
            },
            register_interface::ObjectField {
                name: "includes".into(),
                value: &includes_binding,
            },
            register_interface::ObjectField {
                name: "isolationRequired".into(),
                value: &isolation_required_binding,
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
                name: "purposeJustificationPrompt".into(),
                value: &purpose_justification_prompt_binding,
            },
            register_interface::ObjectField {
                name: "purposeJustificationRequired".into(),
                value: &purpose_justification_required_binding,
            },
            register_interface::ObjectField {
                name: "requires".into(),
                value: &requires_binding,
            },
            register_interface::ObjectField {
                name: "sessionDuration".into(),
                value: &session_duration_binding,
            },
            register_interface::ObjectField {
                name: "zoneId".into(),
                value: &zone_id_binding,
            },
        ]),
        results: vec![
            register_interface::ResultField { name : "accountId".into() },
            register_interface::ResultField { name : "applicationId".into() },
            register_interface::ResultField { name : "approvalGroups".into() },
            register_interface::ResultField { name : "approvalRequired".into() },
            register_interface::ResultField { name : "connectionRules".into() },
            register_interface::ResultField { name : "decision".into() },
            register_interface::ResultField { name : "excludes".into() },
            register_interface::ResultField { name : "includes".into() },
            register_interface::ResultField { name : "isolationRequired".into() },
            register_interface::ResultField { name : "name".into() },
            register_interface::ResultField { name : "precedence".into() },
            register_interface::ResultField { name : "purposeJustificationPrompt".into()
            }, register_interface::ResultField { name : "purposeJustificationRequired"
            .into() }, register_interface::ResultField { name : "requires".into() },
            register_interface::ResultField { name : "sessionDuration".into() },
            register_interface::ResultField { name : "zoneId".into() },
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
    AccessPolicyResult {
        account_id: into_domain(hashmap.remove("accountId").unwrap()),
        application_id: into_domain(hashmap.remove("applicationId").unwrap()),
        approval_groups: into_domain(hashmap.remove("approvalGroups").unwrap()),
        approval_required: into_domain(hashmap.remove("approvalRequired").unwrap()),
        connection_rules: into_domain(hashmap.remove("connectionRules").unwrap()),
        decision: into_domain(hashmap.remove("decision").unwrap()),
        excludes: into_domain(hashmap.remove("excludes").unwrap()),
        includes: into_domain(hashmap.remove("includes").unwrap()),
        isolation_required: into_domain(hashmap.remove("isolationRequired").unwrap()),
        name: into_domain(hashmap.remove("name").unwrap()),
        precedence: into_domain(hashmap.remove("precedence").unwrap()),
        purpose_justification_prompt: into_domain(
            hashmap.remove("purposeJustificationPrompt").unwrap(),
        ),
        purpose_justification_required: into_domain(
            hashmap.remove("purposeJustificationRequired").unwrap(),
        ),
        requires: into_domain(hashmap.remove("requires").unwrap()),
        session_duration: into_domain(hashmap.remove("sessionDuration").unwrap()),
        zone_id: into_domain(hashmap.remove("zoneId").unwrap()),
    }
}
