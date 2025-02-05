pub mod get_policy_set_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicySetDefinitionArgs {
        /// Specifies the display name of the Policy Set Definition. Conflicts with `name`.
        ///
        /// **NOTE** As `display_name` is not unique errors may occur when there are multiple policy set definitions with same display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Only retrieve Policy Set Definitions from this Management Group.
        #[builder(into, default)]
        pub management_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Policy Set Definition. Conflicts with `display_name`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicySetDefinitionArgs,
    ) -> GetPolicySetDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let management_group_name_binding = args
            .management_group_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:policy/getPolicySetDefinition:getPolicySetDefinition".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPolicySetDefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            management_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementGroupName"),
            ),
            metadata: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_definition_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyDefinitionGroups"),
            ),
            policy_definition_references: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyDefinitionReferences"),
            ),
            policy_definitions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyDefinitions"),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyType"),
            ),
        }
    }
}
