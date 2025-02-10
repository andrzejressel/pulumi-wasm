#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_registry_scope_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryScopeMapArgs {
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
    pub struct GetRegistryScopeMapResult {
        /// The actions for the Scope Map.
        pub actions: pulumi_gestalt_rust::Output<Vec<String>>,
        pub container_registry_name: pulumi_gestalt_rust::Output<String>,
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegistryScopeMapArgs,
    ) -> GetRegistryScopeMapResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_registry_name_binding = args
            .container_registry_name
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getRegistryScopeMap:getRegistryScopeMap"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerRegistryName".into(),
                    value: container_registry_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRegistryScopeMapResult {
            actions: o.get_field("actions"),
            container_registry_name: o.get_field("containerRegistryName"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
