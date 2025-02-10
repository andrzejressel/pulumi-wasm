#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_static_web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStaticWebAppArgs {
        /// The name of this Static Web App.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Static Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetStaticWebAppResult {
        pub api_key: pulumi_gestalt_rust::Output<String>,
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub basic_auths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetStaticWebAppBasicAuth>,
        >,
        pub configuration_file_changes_enabled: pulumi_gestalt_rust::Output<bool>,
        pub default_host_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetStaticWebAppIdentity>,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub preview_environments_enabled: pulumi_gestalt_rust::Output<bool>,
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub sku_size: pulumi_gestalt_rust::Output<String>,
        pub sku_tier: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetStaticWebAppArgs,
    ) -> GetStaticWebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getStaticWebApp:getStaticWebApp".into(),
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
        GetStaticWebAppResult {
            api_key: o.get_field("apiKey"),
            app_settings: o.get_field("appSettings"),
            basic_auths: o.get_field("basicAuths"),
            configuration_file_changes_enabled: o
                .get_field("configurationFileChangesEnabled"),
            default_host_name: o.get_field("defaultHostName"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            preview_environments_enabled: o.get_field("previewEnvironmentsEnabled"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_size: o.get_field("skuSize"),
            sku_tier: o.get_field("skuTier"),
            tags: o.get_field("tags"),
        }
    }
}
