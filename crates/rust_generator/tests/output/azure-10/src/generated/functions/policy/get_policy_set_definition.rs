#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_policy_set_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPolicySetDefinitionArgs {
        /// Specifies the display name of the Policy Set Definition. Conflicts with `name`.
        ///
        /// **NOTE** As `display_name` is not unique errors may occur when there are multiple policy set definitions with same display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Only retrieve Policy Set Definitions from this Management Group.
        #[builder(into, default)]
        pub management_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Policy Set Definition. Conflicts with `display_name`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPolicySetDefinitionResult {
        /// The description of this policy definition group.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of this policy definition group.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub management_group_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Any Metadata defined in the Policy Set Definition.
        pub metadata: pulumi_gestalt_rust::Output<String>,
        /// The name of this policy definition group.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The mapping of the parameter values for the referenced policy rule. The keys are the parameter names.
        pub parameters: pulumi_gestalt_rust::Output<String>,
        /// One or more `policy_definition_group` blocks as defined below.
        pub policy_definition_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicySetDefinitionPolicyDefinitionGroup,
            >,
        >,
        /// One or more `policy_definition_reference` blocks as defined below.
        pub policy_definition_references: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::policy::GetPolicySetDefinitionPolicyDefinitionReference,
            >,
        >,
        /// The policy definitions contained within the policy set definition.
        pub policy_definitions: pulumi_gestalt_rust::Output<String>,
        /// The Type of the Policy Set Definition.
        pub policy_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPolicySetDefinitionArgs,
    ) -> GetPolicySetDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let display_name_binding = args.display_name.get_output(context);
        let management_group_name_binding = args
            .management_group_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:policy/getPolicySetDefinition:getPolicySetDefinition".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementGroupName".into(),
                    value: management_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPolicySetDefinitionResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            id: o.get_field("id"),
            management_group_name: o.get_field("managementGroupName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            parameters: o.get_field("parameters"),
            policy_definition_groups: o.get_field("policyDefinitionGroups"),
            policy_definition_references: o.get_field("policyDefinitionReferences"),
            policy_definitions: o.get_field("policyDefinitions"),
            policy_type: o.get_field("policyType"),
        }
    }
}
