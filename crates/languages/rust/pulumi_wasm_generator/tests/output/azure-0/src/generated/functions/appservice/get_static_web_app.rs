pub mod get_static_web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStaticWebAppArgs {
        /// The name of this Static Web App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Static Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetStaticWebAppResult {
        pub api_key: pulumi_wasm_rust::Output<String>,
        pub app_settings: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub basic_auths: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetStaticWebAppBasicAuth>,
        >,
        pub configuration_file_changes_enabled: pulumi_wasm_rust::Output<bool>,
        pub default_host_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appservice::GetStaticWebAppIdentity>,
        >,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub preview_environments_enabled: pulumi_wasm_rust::Output<bool>,
        pub public_network_access_enabled: pulumi_wasm_rust::Output<bool>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub sku_size: pulumi_wasm_rust::Output<String>,
        pub sku_tier: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetStaticWebAppArgs,
    ) -> GetStaticWebAppResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:appservice/getStaticWebApp:getStaticWebApp".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetStaticWebAppResult {
            api_key: pulumi_wasm_rust::__private::into_domain(o.extract_field("apiKey")),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appSettings"),
            ),
            basic_auths: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("basicAuths"),
            ),
            configuration_file_changes_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationFileChangesEnabled"),
            ),
            default_host_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("defaultHostName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            preview_environments_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("previewEnvironmentsEnabled"),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku_size: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuSize"),
            ),
            sku_tier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
