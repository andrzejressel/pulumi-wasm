#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_registry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRegistryArgs {
        /// The name of the Container Registry.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Name of the Resource Group where this Container Registry exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetRegistryResult {
        /// Is the Administrator account enabled for this Container Registry.
        pub admin_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Password associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_password: pulumi_gestalt_rust::Output<String>,
        /// The Username associated with the Container Registry Admin account - if the admin account is enabled.
        pub admin_username: pulumi_gestalt_rust::Output<String>,
        /// Whether dedicated data endpoints for this Container Registry are enabled?
        pub data_endpoint_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region in which this Container Registry exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The URL that can be used to log into the container registry.
        pub login_server: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of this Container Registry, such as `Basic`.
        pub sku: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the Container Registry.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRegistryArgs,
    ) -> GetRegistryResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getRegistry:getRegistry".into(),
            version: super::super::super::get_version(),
            object: &[
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
        GetRegistryResult {
            admin_enabled: o.get_field("adminEnabled"),
            admin_password: o.get_field("adminPassword"),
            admin_username: o.get_field("adminUsername"),
            data_endpoint_enabled: o.get_field("dataEndpointEnabled"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            login_server: o.get_field("loginServer"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku: o.get_field("sku"),
            tags: o.get_field("tags"),
        }
    }
}
