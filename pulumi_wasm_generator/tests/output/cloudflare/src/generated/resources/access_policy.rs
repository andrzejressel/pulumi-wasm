/// Provides a Cloudflare Access Policy resource. Access Policies are
/// used in conjunction with Access Applications to restrict access to
/// a particular resource.
///
/// > It's required that an `account_id` or `zone_id` is provided and in most cases using either is fine.
///    However, if you're using a scoped access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com" zone needs to use the `zone_id` argument.
///    If 'application_id' is omitted, the policy created can be reused by multiple access applications.
///    Any cloudflare.AccessApplication resource can reference reusable policies through its `policies` argument.
///    To destroy a reusable policy and remove it from all applications' policies lists on the same apply, preemptively set the
///    lifecycle option `create_before_destroy` to true on the 'cloudflare_access_policy' resource.
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/accessPolicy:AccessPolicy example account/<account_id>/<application_id>/<policy_id>
/// ```
///
pub mod access_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AccessPolicyArgs) -> AccessPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "applicationId".into(),
                },
                register_interface::ResultField {
                    name: "approvalGroups".into(),
                },
                register_interface::ResultField {
                    name: "approvalRequired".into(),
                },
                register_interface::ResultField {
                    name: "connectionRules".into(),
                },
                register_interface::ResultField {
                    name: "decision".into(),
                },
                register_interface::ResultField {
                    name: "excludes".into(),
                },
                register_interface::ResultField {
                    name: "includes".into(),
                },
                register_interface::ResultField {
                    name: "isolationRequired".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "precedence".into(),
                },
                register_interface::ResultField {
                    name: "purposeJustificationPrompt".into(),
                },
                register_interface::ResultField {
                    name: "purposeJustificationRequired".into(),
                },
                register_interface::ResultField {
                    name: "requires".into(),
                },
                register_interface::ResultField {
                    name: "sessionDuration".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessPolicyResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            application_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applicationId").unwrap(),
            ),
            approval_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalGroups").unwrap(),
            ),
            approval_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("approvalRequired").unwrap(),
            ),
            connection_rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionRules").unwrap(),
            ),
            decision: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("decision").unwrap(),
            ),
            excludes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludes").unwrap(),
            ),
            includes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includes").unwrap(),
            ),
            isolation_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("isolationRequired").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            precedence: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("precedence").unwrap(),
            ),
            purpose_justification_prompt: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purposeJustificationPrompt").unwrap(),
            ),
            purpose_justification_required: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purposeJustificationRequired").unwrap(),
            ),
            requires: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requires").unwrap(),
            ),
            session_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionDuration").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
