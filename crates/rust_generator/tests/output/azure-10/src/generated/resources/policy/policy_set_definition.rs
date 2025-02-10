/// Manages a policy set definition.
///
/// > **NOTE:**  Policy set definitions (also known as policy initiatives) do not take effect until they are assigned to a scope using a Policy Set Assignment.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_set_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicySetDefinitionArgs {
        /// The description of the policy set definition.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The display name of the policy set definition.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Management Group where this policy set definition should be defined. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub management_group_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The metadata for the policy set definition. This is a JSON object representing additional metadata that should be stored with the policy definition.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the policy set definition. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Parameters for the policy set definition. This field is a JSON object that allows you to parameterize your policy definition.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `policy_definition_group` blocks as defined below.
        #[builder(into, default)]
        pub policy_definition_groups: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::policy::PolicySetDefinitionPolicyDefinitionGroup,
                >,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        #[builder(into)]
        pub policy_definition_references: pulumi_gestalt_rust::InputOrOutput<
            Vec<
                super::super::types::policy::PolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy set type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub policy_type: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicySetDefinitionResult {
        /// The description of the policy set definition.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The display name of the policy set definition.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The id of the Management Group where this policy set definition should be defined. Changing this forces a new resource to be created.
        pub management_group_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The metadata for the policy set definition. This is a JSON object representing additional metadata that should be stored with the policy definition.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name of the policy set definition. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Parameters for the policy set definition. This field is a JSON object that allows you to parameterize your policy definition.
        pub parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `policy_definition_group` blocks as defined below.
        pub policy_definition_groups: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::policy::PolicySetDefinitionPolicyDefinitionGroup,
                >,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        pub policy_definition_references: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::policy::PolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy set type. Possible values are `BuiltIn`, `Custom`, `NotSpecified` and `Static`. Changing this forces a new resource to be created.
        pub policy_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicySetDefinitionArgs,
    ) -> PolicySetDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let management_group_id_binding = args.management_group_id.get_output(context);
        let metadata_binding = args.metadata.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let policy_definition_groups_binding = args
            .policy_definition_groups
            .get_output(context);
        let policy_definition_references_binding = args
            .policy_definition_references
            .get_output(context);
        let policy_type_binding = args.policy_type.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:policy/policySetDefinition:PolicySetDefinition".into(),
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
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDefinitionGroups".into(),
                    value: policy_definition_groups_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyDefinitionReferences".into(),
                    value: policy_definition_references_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyType".into(),
                    value: policy_type_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicySetDefinitionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            management_group_id: o.get_field("managementGroupId"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            policy_definition_groups: o.get_field("policyDefinitionGroups"),
            policy_definition_references: o.get_field("policyDefinitionReferences"),
            policy_type: o.get_field("policyType"),
        }
    }
}
