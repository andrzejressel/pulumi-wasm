/// Manages a policy rule definition on a management group or your provider subscription.
///
/// Policy definitions do not take effect until they are assigned to a scope using a Policy Assignment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefinitionArgs {
        /// The description of the policy definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the policy definition.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Management Group where this policy should be defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The metadata for the policy definition. This is a JSON string representing additional metadata that should be stored with the policy definition.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy resource manager mode that allows you to specify which resource types will be evaluated. Possible values are `All`, `Indexed`, `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`.
        ///
        /// > **Note:** Other resource provider modes only support built-in policy definitions but may later become available in custom definitions, these include; `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`. [See here](https://docs.microsoft.com/en-us/azure/governance/policy/concepts/definition-structure#resource-provider-modes) for more details.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the policy definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters for the policy definition. This field is a JSON string that allows you to parameterize your policy definition.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy rule for the policy definition. This is a JSON string representing the rule that contains an if and a then block.
        #[builder(into, default)]
        pub policy_rule: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefinitionResult {
        /// The description of the policy definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the policy definition.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The id of the Management Group where this policy should be defined. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The metadata for the policy definition. This is a JSON string representing additional metadata that should be stored with the policy definition.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The policy resource manager mode that allows you to specify which resource types will be evaluated. Possible values are `All`, `Indexed`, `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`.
        ///
        /// > **Note:** Other resource provider modes only support built-in policy definitions but may later become available in custom definitions, these include; `Microsoft.ContainerService.Data`, `Microsoft.CustomerLockbox.Data`, `Microsoft.DataCatalog.Data`, `Microsoft.KeyVault.Data`, `Microsoft.Kubernetes.Data`, `Microsoft.MachineLearningServices.Data`, `Microsoft.Network.Data` and `Microsoft.Synapse.Data`. [See here](https://docs.microsoft.com/en-us/azure/governance/policy/concepts/definition-structure#resource-provider-modes) for more details.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Parameters for the policy definition. This field is a JSON string that allows you to parameterize your policy definition.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The policy rule for the policy definition. This is a JSON string representing the rule that contains an if and a then block.
        pub policy_rule: pulumi_gestalt_rust::Output<Option<String>>,
        /// The policy type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        pub policy_type: pulumi_gestalt_rust::Output<String>,
        /// A list of role definition id extracted from `policy_rule` required for remediation.
        pub role_definition_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefinitionArgs,
    ) -> DefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let management_group_id_binding = args.management_group_id.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let policy_rule_binding = args.policy_rule.get_output(context);
        let policy_type_binding = args.policy_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:policy/definition:Definition".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupId".into(),
                    value: &management_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: &mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyRule".into(),
                    value: &policy_rule_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyType".into(),
                    value: &policy_type_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DefinitionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            management_group_id: o.get_field("managementGroupId"),
            metadata: o.get_field("metadata"),
            mode: o.get_field("mode"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            policy_rule: o.get_field("policyRule"),
            policy_type: o.get_field("policyType"),
            role_definition_ids: o.get_field("roleDefinitionIds"),
        }
    }
}
