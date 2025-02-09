/// Manages a Policy Assignment to a Management Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = group::create(
///         "example",
///         GroupArgs::builder().display_name("Some Management Group").build_struct(),
///     );
///     let exampleDefinition = definition::create(
///         "exampleDefinition",
///         DefinitionArgs::builder()
///             .display_name("my-policy-definition")
///             .management_group_id("${example.id}")
///             .mode("All")
///             .name("only-deploy-in-westeurope")
///             .policy_rule(
///                 " {\n    \"if\": {\n      \"not\": {\n        \"field\": \"location\",\n        \"equals\": \"westeurope\"\n      }\n    },\n    \"then\": {\n      \"effect\": \"Deny\"\n    }\n  }\n",
///             )
///             .policy_type("Custom")
///             .build_struct(),
///     );
///     let exampleGroupPolicyAssignment = group_policy_assignment::create(
///         "exampleGroupPolicyAssignment",
///         GroupPolicyAssignmentArgs::builder()
///             .management_group_id("${example.id}")
///             .name("example-policy")
///             .policy_definition_id("${exampleDefinition.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Management Group Policy Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:management/groupPolicyAssignment:GroupPolicyAssignment example /providers/Microsoft.Management/managementGroups/group1/providers/Microsoft.Authorization/policyAssignments/assignment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group_policy_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAssignmentArgs {
        /// A description which should be used for this Policy Assignment.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Display Name for this Policy Assignment.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        #[builder(into, default)]
        pub enforce: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::management::GroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Management Group. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A JSON mapping of any Metadata for this Policy.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Policy Assignment. Possible values must be between 3 and 24 characters in length. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `non_compliance_message` blocks as defined below.
        #[builder(into, default)]
        pub non_compliance_messages: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        #[builder(into, default)]
        pub not_scopes: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        #[builder(into, default)]
        pub overrides: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::management::GroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub policy_definition_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        #[builder(into, default)]
        pub resource_selectors: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentResourceSelector,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupPolicyAssignmentResult {
        /// A description which should be used for this Policy Assignment.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Display Name for this Policy Assignment.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        pub enforce: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::management::GroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Management Group. Changing this forces a new Policy Assignment to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Policy Assignment. Possible values must be between 3 and 24 characters in length. Changing this forces a new Policy Assignment to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `non_compliance_message` blocks as defined below.
        pub non_compliance_messages: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        pub not_scopes: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        pub overrides: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::management::GroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        pub policy_definition_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        pub resource_selectors: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentResourceSelector,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: GroupPolicyAssignmentArgs,
    ) -> GroupPolicyAssignmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let display_name_binding_1 = args.display_name.get_output(context);
        let display_name_binding = display_name_binding_1.get_inner();
        let enforce_binding_1 = args.enforce.get_output(context);
        let enforce_binding = enforce_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let management_group_id_binding_1 = args.management_group_id.get_output(context);
        let management_group_id_binding = management_group_id_binding_1.get_inner();
        let metadata_binding_1 = args.metadata.get_output(context);
        let metadata_binding = metadata_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let non_compliance_messages_binding_1 = args
            .non_compliance_messages
            .get_output(context);
        let non_compliance_messages_binding = non_compliance_messages_binding_1
            .get_inner();
        let not_scopes_binding_1 = args.not_scopes.get_output(context);
        let not_scopes_binding = not_scopes_binding_1.get_inner();
        let overrides_binding_1 = args.overrides.get_output(context);
        let overrides_binding = overrides_binding_1.get_inner();
        let parameters_binding_1 = args.parameters.get_output(context);
        let parameters_binding = parameters_binding_1.get_inner();
        let policy_definition_id_binding_1 = args
            .policy_definition_id
            .get_output(context);
        let policy_definition_id_binding = policy_definition_id_binding_1.get_inner();
        let resource_selectors_binding_1 = args.resource_selectors.get_output(context);
        let resource_selectors_binding = resource_selectors_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupPolicyAssignment:GroupPolicyAssignment".into(),
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
                    name: "enforce".into(),
                    value: &enforce_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "nonComplianceMessages".into(),
                    value: &non_compliance_messages_binding,
                },
                register_interface::ObjectField {
                    name: "notScopes".into(),
                    value: &not_scopes_binding,
                },
                register_interface::ObjectField {
                    name: "overrides".into(),
                    value: &overrides_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionId".into(),
                    value: &policy_definition_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceSelectors".into(),
                    value: &resource_selectors_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupPolicyAssignmentResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            enforce: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enforce"),
            ),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            non_compliance_messages: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nonComplianceMessages"),
            ),
            not_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notScopes"),
            ),
            overrides: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("overrides"),
            ),
            parameters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_definition_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("policyDefinitionId"),
            ),
            resource_selectors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceSelectors"),
            ),
        }
    }
}
