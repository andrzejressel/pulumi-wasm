/// Manages a Management Group Policy Exemption.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleGroup:
///     type: azure:management:Group
///     name: example
///     properties:
///       displayName: Example MgmtGroup
///   exampleGroupPolicyAssignment:
///     type: azure:management:GroupPolicyAssignment
///     name: example
///     properties:
///       name: assignment1
///       managementGroupId: ${exampleGroup.id}
///       policyDefinitionId: ${example.id}
///       location: westus
///       identity:
///         type: SystemAssigned
///   exampleGroupPolicyExemption:
///     type: azure:management:GroupPolicyExemption
///     name: example
///     properties:
///       name: exemption1
///       managementGroupId: ${exampleGroup.id}
///       policyAssignmentId: ${exampleGroupPolicyAssignment.id}
///       exemptionCategory: Mitigated
/// variables:
///   example:
///     fn::invoke:
///       function: azure:policy:getPolicySetDefinition
///       arguments:
///         displayName: Audit machines with insecure password security settings
/// ```
///
/// ## Import
///
/// Policy Exemptions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/groupPolicyExemption:GroupPolicyExemption exemption1 /providers/Microsoft.Management/managementGroups/group1/providers/Microsoft.Authorization/policyExemptions/exemption1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policy_exemption {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyExemptionArgs {
        /// A description to use for this Policy Exemption.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A friendly display name to use for this Policy Exemption.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The category of this policy exemption. Possible values are `Waiver` and `Mitigated`.
        #[builder(into)]
        pub exemption_category: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The expiration date and time in UTC ISO 8601 format of this policy exemption.
        #[builder(into, default)]
        pub expires_on: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Management Group ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The metadata for this policy exemption. This is a JSON string representing additional metadata that should be stored with the policy exemption.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Policy Exemption. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Assignment to be exempted at the specified Scope.
        #[builder(into)]
        pub policy_assignment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition.
        #[builder(into, default)]
        pub policy_definition_reference_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyExemptionResult {
        /// A description to use for this Policy Exemption.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A friendly display name to use for this Policy Exemption.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The category of this policy exemption. Possible values are `Waiver` and `Mitigated`.
        pub exemption_category: pulumi_gestalt_rust::Output<String>,
        /// The expiration date and time in UTC ISO 8601 format of this policy exemption.
        pub expires_on: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Management Group ID where the Policy Exemption should be applied. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// The metadata for this policy exemption. This is a JSON string representing additional metadata that should be stored with the policy exemption.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name of the Policy Exemption. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Policy Assignment to be exempted at the specified Scope.
        pub policy_assignment_id: pulumi_gestalt_rust::Output<String>,
        /// The policy definition reference ID list when the associated policy assignment is an assignment of a policy set definition.
        pub policy_definition_reference_ids: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupPolicyExemptionArgs,
    ) -> GroupPolicyExemptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let exemption_category_binding = args.exemption_category.get_output(context);
        let expires_on_binding = args.expires_on.get_output(context);
        let management_group_id_binding = args.management_group_id.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_assignment_id_binding = args.policy_assignment_id.get_output(context);
        let policy_definition_reference_ids_binding = args
            .policy_definition_reference_ids
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:management/groupPolicyExemption:GroupPolicyExemption".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exemptionCategory".into(),
                    value: exemption_category_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expiresOn".into(),
                    value: expires_on_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupId".into(),
                    value: management_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: metadata_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyAssignmentId".into(),
                    value: policy_assignment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDefinitionReferenceIds".into(),
                    value: policy_definition_reference_ids_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupPolicyExemptionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            exemption_category: o.get_field("exemptionCategory"),
            expires_on: o.get_field("expiresOn"),
            management_group_id: o.get_field("managementGroupId"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            policy_assignment_id: o.get_field("policyAssignmentId"),
            policy_definition_reference_ids: o.get_field("policyDefinitionReferenceIds"),
        }
    }
}
