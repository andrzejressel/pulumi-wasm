/// Manages a policy set definition.
///
/// > **NOTE:**  Policy set definitions (also known as policy initiatives) do not take effect until they are assigned to a scope using a Policy Set Assignment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = policy_set_definition::create(
///         "example",
///         PolicySetDefinitionArgs::builder()
///             .display_name("Test Policy Set")
///             .name("testPolicySet")
///             .parameters(
///                 "    {\n        \"allowedLocations\": {\n            \"type\": \"Array\",\n            \"metadata\": {\n                \"description\": \"The list of allowed locations for resources.\",\n                \"displayName\": \"Allowed locations\",\n                \"strongType\": \"location\"\n            }\n        }\n    }\n",
///             )
///             .policy_definition_references(
///                 vec![
///                     PolicySetDefinitionPolicyDefinitionReference::builder()
///                     .parameterValues("    {\n      \"listOfAllowedLocations\": {\"value\": \"[parameters('allowedLocations')]\"}\n    }")
///                     .policyDefinitionId("/providers/Microsoft.Authorization/policyDefinitions/e765b5de-1225-4ba3-bd56-1ac6695af988")
///                     .build_struct(),
///                 ],
///             )
///             .policy_type("Custom")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Policy Set Definitions can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:policy/policySetDefinition:PolicySetDefinition example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Authorization/policySetDefinitions/testPolicySet
/// ```
///
/// or
///
/// ```sh
/// $ pulumi import azure:policy/policySetDefinition:PolicySetDefinition example /providers/Microsoft.Management/managementGroups/my-mgmt-group-id/providers/Microsoft.Authorization/policySetDefinitions/testPolicySet
/// ```
///
pub mod policy_set_definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicySetDefinitionArgs {
        /// The description of the policy set definition.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The display name of the policy set definition.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the Management Group where this policy set definition should be defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The metadata for the policy set definition. This is a JSON object representing additional metadata that should be stored with the policy definition.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the policy set definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Parameters for the policy set definition. This field is a JSON object that allows you to parameterize your policy definition.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `policy_definition_group` blocks as defined below.
        #[builder(into, default)]
        pub policy_definition_groups: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::policy::PolicySetDefinitionPolicyDefinitionGroup,
                >,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        #[builder(into)]
        pub policy_definition_references: pulumi_wasm_rust::InputOrOutput<
            Vec<
                super::super::types::policy::PolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy set type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicySetDefinitionResult {
        /// The description of the policy set definition.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the policy set definition.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The id of the Management Group where this policy set definition should be defined. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The metadata for the policy set definition. This is a JSON object representing additional metadata that should be stored with the policy definition.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The name of the policy set definition. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Parameters for the policy set definition. This field is a JSON object that allows you to parameterize your policy definition.
        pub parameters: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `policy_definition_group` blocks as defined below.
        pub policy_definition_groups: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::policy::PolicySetDefinitionPolicyDefinitionGroup,
                >,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        pub policy_definition_references: pulumi_wasm_rust::Output<
            Vec<
                super::super::types::policy::PolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy set type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        pub policy_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: PolicySetDefinitionArgs,
    ) -> PolicySetDefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let policy_definition_groups_binding = args
            .policy_definition_groups
            .get_output(context)
            .get_inner();
        let policy_definition_references_binding = args
            .policy_definition_references
            .get_output(context)
            .get_inner();
        let policy_type_binding = args.policy_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:policy/policySetDefinition:PolicySetDefinition".into(),
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
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionGroups".into(),
                    value: &policy_definition_groups_binding,
                },
                register_interface::ObjectField {
                    name: "policyDefinitionReferences".into(),
                    value: &policy_definition_references_binding,
                },
                register_interface::ObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding,
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
                    name: "managementGroupId".into(),
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
                    name: "policyType".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        PolicySetDefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
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
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            policy_definition_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionGroups").unwrap(),
            ),
            policy_definition_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyDefinitionReferences").unwrap(),
            ),
            policy_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("policyType").unwrap(),
            ),
        }
    }
}
