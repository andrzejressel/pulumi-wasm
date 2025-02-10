#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod role_assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RoleAssignmentArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub condition_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub delegated_managed_identity_resource_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub principal_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub role_definition_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub role_definition_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub skip_service_principal_aad_check: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
    }
    #[allow(dead_code)]
    pub struct RoleAssignmentResult {
        pub condition: pulumi_gestalt_rust::Output<Option<String>>,
        pub condition_version: pulumi_gestalt_rust::Output<Option<String>>,
        pub delegated_managed_identity_resource_id: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub principal_id: pulumi_gestalt_rust::Output<String>,
        pub principal_type: pulumi_gestalt_rust::Output<String>,
        pub role_definition_id: pulumi_gestalt_rust::Output<Option<String>>,
        pub role_definition_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub skip_service_principal_aad_check: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RoleAssignmentArgs,
    ) -> RoleAssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let condition_version_binding = args.condition_version.get_output(context);
        let delegated_managed_identity_resource_id_binding = args
            .delegated_managed_identity_resource_id
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let principal_id_binding = args.principal_id.get_output(context);
        let role_definition_id_binding = args.role_definition_id.get_output(context);
        let role_definition_name_binding = args.role_definition_name.get_output(context);
        let skip_service_principal_aad_check_binding = args
            .skip_service_principal_aad_check
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:marketplace/roleAssignment:RoleAssignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: condition_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "conditionVersion".into(),
                    value: condition_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedManagedIdentityResourceId".into(),
                    value: delegated_managed_identity_resource_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "principalId".into(),
                    value: principal_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionId".into(),
                    value: role_definition_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleDefinitionName".into(),
                    value: role_definition_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipServicePrincipalAadCheck".into(),
                    value: skip_service_principal_aad_check_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RoleAssignmentResult {
            condition: o.get_field("condition"),
            condition_version: o.get_field("conditionVersion"),
            delegated_managed_identity_resource_id: o
                .get_field("delegatedManagedIdentityResourceId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            principal_id: o.get_field("principalId"),
            principal_type: o.get_field("principalType"),
            role_definition_id: o.get_field("roleDefinitionId"),
            role_definition_name: o.get_field("roleDefinitionName"),
            skip_service_principal_aad_check: o.get_field("skipServicePrincipalAadCheck"),
        }
    }
}
