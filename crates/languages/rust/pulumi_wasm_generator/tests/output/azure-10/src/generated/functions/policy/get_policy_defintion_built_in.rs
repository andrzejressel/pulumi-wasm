pub mod get_policy_defintion_built_in {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicyDefintionBuiltInArgs {
        /// Specifies the display name of the Policy Definition. Conflicts with `name`.
        ///
        /// > **NOTE** As `display_name` is not unique errors may occur when there are multiple policy definitions with same display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Only retrieve Policy Definitions from this Management Group.
        #[builder(into, default)]
        pub management_group_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Policy Definition. Conflicts with `display_name`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicyDefintionBuiltInResult {
        /// The Description of the Policy.
        pub description: pulumi_wasm_rust::Output<String>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub management_group_name: pulumi_wasm_rust::Output<Option<String>>,
        /// Any Metadata defined in the Policy.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The Mode of the Policy.
        pub mode: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Any Parameters defined in the Policy.
        pub parameters: pulumi_wasm_rust::Output<String>,
        /// The Rule as defined (in JSON) in the Policy.
        pub policy_rule: pulumi_wasm_rust::Output<String>,
        /// The Type of the Policy. Possible values are `BuiltIn`, `Custom` and `NotSpecified`.
        pub policy_type: pulumi_wasm_rust::Output<String>,
        /// A list of role definition id extracted from `policy_rule` required for remediation.
        pub role_definition_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// The Type of Policy.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPolicyDefintionBuiltInArgs,
    ) -> GetPolicyDefintionBuiltInResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let management_group_name_binding = args
            .management_group_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:policy/getPolicyDefintionBuiltIn:getPolicyDefintionBuiltIn"
                .into(),
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
        GetPolicyDefintionBuiltInResult {
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
            mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("mode")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameters: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameters"),
            ),
            policy_rule: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyRule"),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyType"),
            ),
            role_definition_ids: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("roleDefinitionIds"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
