#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_registry_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryTokenArgs {
        /// The Name of the Container Registry where the token exists.
        #[builder(into)]
        pub container_registry_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Container Registry token.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where this Container Registry token exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryTokenResult {
        pub container_registry_name: pulumi_gestalt_rust::Output<String>,
        /// Whether this Token is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Scope Map ID used by the token.
        pub scope_map_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegistryTokenArgs,
    ) -> GetRegistryTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_name_binding = args
            .container_registry_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getRegistryToken:getRegistryToken".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryName".into(),
                    value: &container_registry_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegistryTokenResult {
            container_registry_name: o.get_field("containerRegistryName"),
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            scope_map_id: o.get_field("scopeMapId"),
        }
    }
}
