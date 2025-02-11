#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_hardware_security_module_role_definition {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleRoleDefinitionArgs {
        #[builder(into)]
        pub managed_hsm_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name in UUID notation of this KeyVault Role Definition.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetManagedHardwareSecurityModuleRoleDefinitionResult {
        /// A list of assignable role scope. Possible values are `/` and `/keys`.
        pub assignable_scopes: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A text description of this role definition.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub managed_hsm_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `permission` block as defined below.
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::keyvault::GetManagedHardwareSecurityModuleRoleDefinitionPermission,
            >,
        >,
        /// The ID of the role definition resource without base url.
        pub resource_manager_id: pulumi_gestalt_rust::Output<String>,
        /// The role name of the role definition.
        pub role_name: pulumi_gestalt_rust::Output<String>,
        /// The type of the role definition. Possible values are `AKVBuiltInRole` and `CustomRole`.
        pub role_type: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedHardwareSecurityModuleRoleDefinitionArgs,
    ) -> GetManagedHardwareSecurityModuleRoleDefinitionResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleRoleDefinition:getManagedHardwareSecurityModuleRoleDefinition"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetManagedHardwareSecurityModuleRoleDefinitionResult {
            assignable_scopes: o.get_field("assignableScopes"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            managed_hsm_id: o.get_field("managedHsmId"),
            name: o.get_field("name"),
            permissions: o.get_field("permissions"),
            resource_manager_id: o.get_field("resourceManagerId"),
            role_name: o.get_field("roleName"),
            role_type: o.get_field("roleType"),
        }
    }
}
