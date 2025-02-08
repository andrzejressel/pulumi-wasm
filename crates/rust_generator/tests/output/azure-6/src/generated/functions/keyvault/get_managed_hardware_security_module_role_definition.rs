#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedHardwareSecurityModuleRoleDefinitionArgs,
    ) -> GetManagedHardwareSecurityModuleRoleDefinitionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let managed_hsm_id_binding = args.managed_hsm_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:keyvault/getManagedHardwareSecurityModuleRoleDefinition:getManagedHardwareSecurityModuleRoleDefinition"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "managedHsmId".into(),
                    value: &managed_hsm_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedHardwareSecurityModuleRoleDefinitionResult {
            assignable_scopes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("assignableScopes"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            managed_hsm_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managedHsmId"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            resource_manager_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceManagerId"),
            ),
            role_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleName"),
            ),
            role_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("roleType"),
            ),
        }
    }
}
