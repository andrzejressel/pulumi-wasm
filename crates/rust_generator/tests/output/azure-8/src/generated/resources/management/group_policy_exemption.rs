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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupPolicyExemptionArgs,
    ) -> GroupPolicyExemptionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let exemption_category_binding_1 = args.exemption_category.get_output(context);
        let exemption_category_binding = exemption_category_binding_1.get_inner();
        let expires_on_binding_1 = args.expires_on.get_output(context);
        let expires_on_binding = expires_on_binding_1.get_inner();
        let management_group_id_binding_1 = args.management_group_id.get_output(context);
        let management_group_id_binding = management_group_id_binding_1.get_inner();
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let policy_assignment_id_binding_1 = args
            .policy_assignment_id
            .get_output(context);
        let policy_assignment_id_binding = policy_assignment_id_binding_1.get_inner();
        let policy_definition_reference_ids_binding_1 = args
            .policy_definition_reference_ids
            .get_output(context);
        let policy_definition_reference_ids_binding = policy_definition_reference_ids_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupPolicyExemption:GroupPolicyExemption".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "exemptionCategory".into(),
                    value: &exemption_category_binding,
                },
                register_interface::ObjectField {
                    name: "expiresOn".into(),
                    value: &expires_on_binding,
                },
                register_interface::ObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "policyAssignmentId".into(),
                    value: &policy_assignment_id_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionReferenceIds".into(),
                    value: &policy_definition_reference_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupPolicyExemptionResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            exemption_category: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exemptionCategory"),
            ),
            expires_on: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expiresOn"),
            ),
            management_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            policy_assignment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyAssignmentId"),
            ),
            policy_definition_reference_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDefinitionReferenceIds"),
            ),
        }
    }
}
