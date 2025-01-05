pub mod get_policy_set_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicySetDefinitionArgs {
        /// Specifies the display name of the Policy Set Definition. Conflicts with `name`.
        ///
        /// **NOTE** As `display_name` is not unique errors may occur when there are multiple policy set definitions with same display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Only retrieve Policy Set Definitions from this Management Group.
        #[builder(into, default)]
        pub management_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Policy Set Definition. Conflicts with `display_name`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicySetDefinitionResult {
        /// The description of this policy definition group.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of this policy definition group.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub management_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Any Metadata defined in the Policy Set Definition.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The name of this policy definition group.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The mapping of the parameter values for the referenced policy rule. The keys are the parameter names.
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// One or more `policy_definition_group` blocks as defined below.
        pub policy_definition_groups: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicySetDefinitionPolicyDefinitionGroup,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        pub policy_definition_references: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy definitions contained within the policy set definition.
        pub policy_definitions: pulumi_wasm_rust::Output<String>,
        /// The Type of the Policy Set Definition.
        pub policy_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPolicySetDefinitionArgs) -> GetPolicySetDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_inner();
        let management_group_name_binding = args.management_group_name.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:policy/getPolicySetDefinition:getPolicySetDefinition".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "managementGroupName".into(),
                    value: &management_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
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
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "managementGroupName".into(),
                },
                register_interface::ResultField {
                    name: "metadata".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitionGroups".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitionReferences".into(),
                },
                register_interface::ResultField {
                    name: "policyDefinitions".into(),
                },
                register_interface::ResultField {
                    name: "policyType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPolicySetDefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            management_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managementGroupName").unwrap(),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metadata").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            policy_definition_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionGroups").unwrap(),
            ),
            policy_definition_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionReferences").unwrap(),
            ),
            policy_definitions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitions").unwrap(),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyType").unwrap(),
            ),
        }
    }
}
