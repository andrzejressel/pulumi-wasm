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
/// $ pulumi import cloudflare:index/zeroTrustAccessPolicy:ZeroTrustAccessPolicy example account/<account_id>/<application_id>/<policy_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zero_trust_access_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZeroTrustAccessPolicyArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub approval_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessPolicyApprovalGroup>>,
        >,
        #[builder(into, default)]
        pub approval_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
        #[builder(into, default)]
        pub connection_rules: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ZeroTrustAccessPolicyConnectionRules>,
        >,
        /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
        #[builder(into)]
        pub decision: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A series of access conditions, see Access Groups.
        #[builder(into, default)]
        pub excludes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessPolicyExclude>>,
        >,
        /// A series of access conditions, see Access Groups.
        #[builder(into)]
        pub includes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::ZeroTrustAccessPolicyInclude>,
        >,
        /// Require this application to be served in an isolated browser for users matching this policy.
        #[builder(into, default)]
        pub isolation_required: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Friendly name of the Access Policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The unique precedence for policies on a single application. Required when using `application_id`.
        #[builder(into, default)]
        pub precedence: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
        #[builder(into, default)]
        pub purpose_justification_prompt: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Whether to prompt the user for a justification for accessing the resource.
        #[builder(into, default)]
        pub purpose_justification_required: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// A series of access conditions, see Access Groups.
        #[builder(into, default)]
        pub requires: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ZeroTrustAccessPolicyRequire>>,
        >,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        #[builder(into, default)]
        pub session_duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ZeroTrustAccessPolicyResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the application the policy is associated with. Required when using `precedence`. **Modifying this attribute will force creation of a new resource.**
        pub application_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub approval_groups: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessPolicyApprovalGroup>>,
        >,
        pub approval_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The rules that define how users may connect to the targets secured by your application. Only applicable to Infrastructure Applications, in which case this field is required.
        pub connection_rules: pulumi_gestalt_rust::Output<
            Option<super::types::ZeroTrustAccessPolicyConnectionRules>,
        >,
        /// Defines the action Access will take if the policy matches the user. Available values: `allow`, `deny`, `non_identity`, `bypass`.
        pub decision: pulumi_gestalt_rust::Output<String>,
        /// A series of access conditions, see Access Groups.
        pub excludes: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessPolicyExclude>>,
        >,
        /// A series of access conditions, see Access Groups.
        pub includes: pulumi_gestalt_rust::Output<
            Vec<super::types::ZeroTrustAccessPolicyInclude>,
        >,
        /// Require this application to be served in an isolated browser for users matching this policy.
        pub isolation_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Friendly name of the Access Policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The unique precedence for policies on a single application. Required when using `application_id`.
        pub precedence: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The prompt to display to the user for a justification for accessing the resource. Required when using `purpose_justification_required`.
        pub purpose_justification_prompt: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether to prompt the user for a justification for accessing the resource.
        pub purpose_justification_required: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A series of access conditions, see Access Groups.
        pub requires: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::ZeroTrustAccessPolicyRequire>>,
        >,
        /// How often a user will be forced to re-authorise. Must be in the format `48h` or `2h45m`.
        pub session_duration: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZeroTrustAccessPolicyArgs,
    ) -> ZeroTrustAccessPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let application_id_binding = args.application_id.get_output(context);
        let approval_groups_binding = args.approval_groups.get_output(context);
        let approval_required_binding = args.approval_required.get_output(context);
        let connection_rules_binding = args.connection_rules.get_output(context);
        let decision_binding = args.decision.get_output(context);
        let excludes_binding = args.excludes.get_output(context);
        let includes_binding = args.includes.get_output(context);
        let isolation_required_binding = args.isolation_required.get_output(context);
        let name_binding = args.name.get_output(context);
        let precedence_binding = args.precedence.get_output(context);
        let purpose_justification_prompt_binding = args
            .purpose_justification_prompt
            .get_output(context);
        let purpose_justification_required_binding = args
            .purpose_justification_required
            .get_output(context);
        let requires_binding = args.requires.get_output(context);
        let session_duration_binding = args.session_duration.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zeroTrustAccessPolicy:ZeroTrustAccessPolicy".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalGroups".into(),
                    value: approval_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "approvalRequired".into(),
                    value: approval_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionRules".into(),
                    value: connection_rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "decision".into(),
                    value: decision_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludes".into(),
                    value: excludes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includes".into(),
                    value: includes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isolationRequired".into(),
                    value: isolation_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "precedence".into(),
                    value: precedence_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purposeJustificationPrompt".into(),
                    value: purpose_justification_prompt_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "purposeJustificationRequired".into(),
                    value: purpose_justification_required_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requires".into(),
                    value: requires_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sessionDuration".into(),
                    value: session_duration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZeroTrustAccessPolicyResult {
            account_id: o.get_field("accountId"),
            application_id: o.get_field("applicationId"),
            approval_groups: o.get_field("approvalGroups"),
            approval_required: o.get_field("approvalRequired"),
            connection_rules: o.get_field("connectionRules"),
            decision: o.get_field("decision"),
            excludes: o.get_field("excludes"),
            includes: o.get_field("includes"),
            isolation_required: o.get_field("isolationRequired"),
            name: o.get_field("name"),
            precedence: o.get_field("precedence"),
            purpose_justification_prompt: o.get_field("purposeJustificationPrompt"),
            purpose_justification_required: o.get_field("purposeJustificationRequired"),
            requires: o.get_field("requires"),
            session_duration: o.get_field("sessionDuration"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
