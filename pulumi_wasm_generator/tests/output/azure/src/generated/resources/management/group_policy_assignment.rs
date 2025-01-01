/// Manages a Policy Assignment to a Management Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod group_policy_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupPolicyAssignmentArgs {
        /// A description which should be used for this Policy Assignment.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Display Name for this Policy Assignment.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        #[builder(into, default)]
        pub enforce: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::management::GroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Management Group. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Policy Assignment. Possible values must be between 3 and 24 characters in length. Changing this forces a new Policy Assignment to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `non_compliance_message` blocks as defined below.
        #[builder(into, default)]
        pub non_compliance_messages: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        #[builder(into, default)]
        pub not_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        #[builder(into, default)]
        pub overrides: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::management::GroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        #[builder(into)]
        pub policy_definition_id: pulumi_wasm_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        #[builder(into, default)]
        pub resource_selectors: pulumi_wasm_rust::Output<
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
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Display Name for this Policy Assignment.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies if this Policy should be enforced or not? Defaults to `true`.
        pub enforce: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        ///
        /// > **Note:** The `location` field must also be specified when `identity` is specified.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::management::GroupPolicyAssignmentIdentity>,
        >,
        /// The Azure Region where the Policy Assignment should exist. Changing this forces a new Policy Assignment to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Management Group. Changing this forces a new Policy Assignment to be created.
        pub management_group_id: pulumi_wasm_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Policy.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Policy Assignment. Possible values must be between 3 and 24 characters in length. Changing this forces a new Policy Assignment to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `non_compliance_message` blocks as defined below.
        pub non_compliance_messages: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::management::GroupPolicyAssignmentNonComplianceMessage,
                >,
            >,
        >,
        /// Specifies a list of Resource Scopes (for example a Subscription, or a Resource Group) within this Management Group which are excluded from this Policy.
        pub not_scopes: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more `overrides` blocks as defined below. More detail about `overrides` and `resource_selectors` see [policy assignment structure](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/assignment-structure#resource-selectors-preview)
        pub overrides: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::management::GroupPolicyAssignmentOverride>>,
        >,
        /// A JSON mapping of any Parameters for this Policy.
        pub parameters: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Policy Definition or Policy Definition Set. Changing this forces a new Policy Assignment to be created.
        pub policy_definition_id: pulumi_wasm_rust::Output<String>,
        /// One or more `resource_selectors` blocks as defined below to filter polices by resource properties.
        pub resource_selectors: pulumi_wasm_rust::Output<
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
        name: &str,
        args: GroupPolicyAssignmentArgs,
    ) -> GroupPolicyAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let enforce_binding = args.enforce.get_inner();
        let identity_binding = args.identity.get_inner();
        let location_binding = args.location.get_inner();
        let management_group_id_binding = args.management_group_id.get_inner();
        let metadata_binding = args.metadata.get_inner();
        let name_binding = args.name.get_inner();
        let non_compliance_messages_binding = args.non_compliance_messages.get_inner();
        let not_scopes_binding = args.not_scopes.get_inner();
        let overrides_binding = args.overrides.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let policy_definition_id_binding = args.policy_definition_id.get_inner();
        let resource_selectors_binding = args.resource_selectors.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:management/groupPolicyAssignment:GroupPolicyAssignment".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "enforce".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managementGroupId".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "nonComplianceMessages".into(),
                },
                register_interface::ResultField {
                    name: "notScopes".into(),
                },
                register_interface::ResultField {
                    name: "overrides".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitionId".into(),
                },
                register_interface::ResultField {
                    name: "resourceSelectors".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GroupPolicyAssignmentResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            enforce: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enforce").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupId").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            non_compliance_messages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nonComplianceMessages").unwrap(),
            ),
            not_scopes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notScopes").unwrap(),
            ),
            overrides: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("overrides").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            policy_definition_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionId").unwrap(),
            ),
            resource_selectors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceSelectors").unwrap(),
            ),
        }
    }
}
