/// Manages a policy rule definition on a management group or your provider subscription.
///
/// Policy definitions do not take effect until they are assigned to a scope using a Policy Assignment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = definition::create(
///         "policy",
///         DefinitionArgs::builder()
///             .display_name("acceptance test policy definition")
///             .metadata("    {\n    \"category\": \"General\"\n    }\n\n")
///             .mode("Indexed")
///             .name("accTestPolicy")
///             .parameters(
///                 " {\n    \"allowedLocations\": {\n      \"type\": \"Array\",\n      \"metadata\": {\n        \"description\": \"The list of allowed locations for resources.\",\n        \"displayName\": \"Allowed locations\",\n        \"strongType\": \"location\"\n      }\n    }\n  }",
///             )
///             .policy_rule(
///                 " {\n    \"if\": {\n      \"not\": {\n        \"field\": \"location\",\n        \"in\": \"[parameters('allowedLocations')]\"\n      }\n    },\n    \"then\": {\n      \"effect\": \"audit\"\n    }\n  }\n",
///             )
///             .policy_type("Custom")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Policy Definitions can be imported using the `policy name`, e.g.
///
/// ```sh
/// $ pulumi import azure:policy/definition:Definition examplePolicy /subscriptions/<SUBSCRIPTION_ID>/providers/Microsoft.Authorization/policyDefinitions/<POLICY_NAME>
/// ```
///
/// or
///
/// ```sh
/// $ pulumi import azure:policy/definition:Definition examplePolicy /providers/Microsoft.Management/managementgroups/<MANGAGEMENT_GROUP_ID>/providers/Microsoft.Authorization/policyDefinitions/<POLICY_NAME>
/// ```
///
pub mod definition {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// The description of the policy definition.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The display name of the policy definition.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The id of the Management Group where this policy should be defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_group_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The metadata for the policy definition. This is a JSON string representing additional metadata that should be stored with the policy definition.
        #[builder(into, default)]
        pub metadata: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The policy resource manager mode that allows you to specify which resource types will be evaluated. Possible values are `All`, `Indexed`, `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`.
        ///
        /// > **Note:** Other resource provider modes only support built-in policy definitions but may later become available in custom definitions, these include; `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`. [See here](https://docs.microsoft.com/en-us/azure/governance/policy/concepts/definition-structure#resource-provider-modes) for more details.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the policy definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Parameters for the policy definition. This field is a JSON string that allows you to parameterize your policy definition.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The policy rule for the policy definition. This is a JSON string representing the rule that contains an if and a then block.
        #[builder(into, default)]
        pub policy_rule: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The policy type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_type: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// The description of the policy definition.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The display name of the policy definition.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The id of the Management Group where this policy should be defined. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The metadata for the policy definition. This is a JSON string representing additional metadata that should be stored with the policy definition.
        pub metadata: pulumi_wasm_rust::Output<String>,
        /// The policy resource manager mode that allows you to specify which resource types will be evaluated. Possible values are `All`, `Indexed`, `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`.
        ///
        /// > **Note:** Other resource provider modes only support built-in policy definitions but may later become available in custom definitions, these include; `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`. [See here](https://docs.microsoft.com/en-us/azure/governance/policy/concepts/definition-structure#resource-provider-modes) for more details.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// The name of the policy definition. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Parameters for the policy definition. This field is a JSON string that allows you to parameterize your policy definition.
        pub parameters: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy rule for the policy definition. This is a JSON string representing the rule that contains an if and a then block.
        pub policy_rule: pulumi_wasm_rust::Output<Option<String>>,
        /// The policy type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        pub policy_type: pulumi_wasm_rust::Output<String>,
        /// A list of role definition id extracted from `policy_rule` required for remediation.
        pub role_definition_ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let display_name_binding = args.display_name.get_output(context).get_inner();
        let management_group_id_binding = args
            .management_group_id
            .get_output(context)
            .get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_binding = args.parameters.get_output(context).get_inner();
        let policy_rule_binding = args.policy_rule.get_output(context).get_inner();
        let policy_type_binding = args.policy_type.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:policy/definition:Definition".into(),
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
                    name: "mode".into(),
                    value: &mode_binding,
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
                    name: "policyRule".into(),
                    value: &policy_rule_binding,
                },
                register_interface::ObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DefinitionResult {
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            management_group_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("managementGroupId"),
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
        }
    }
}
