pub mod get_static_web_app {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStaticWebAppArgs {
        /// The name of this Static Web App.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Static Web App exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetStaticWebAppArgs) -> GetStaticWebAppResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiKey".into(),
                },
                register_interface::ResultField {
                    name: "appSettings".into(),
                },
                register_interface::ResultField {
                    name: "basicAuths".into(),
                },
                register_interface::ResultField {
                    name: "configurationFileChangesEnabled".into(),
                },
                register_interface::ResultField {
                    name: "defaultHostName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "previewEnvironmentsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "skuSize".into(),
                },
                register_interface::ResultField {
                    name: "skuTier".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStaticWebAppResult {
            api_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiKey").unwrap(),
            ),
            app_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("appSettings").unwrap(),
            ),
            basic_auths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("basicAuths").unwrap(),
            ),
            configuration_file_changes_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationFileChangesEnabled").unwrap(),
            ),
            default_host_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultHostName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            preview_environments_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("previewEnvironmentsEnabled").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            sku_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuSize").unwrap(),
            ),
            sku_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuTier").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
