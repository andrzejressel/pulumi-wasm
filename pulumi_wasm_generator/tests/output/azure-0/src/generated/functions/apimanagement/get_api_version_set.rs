pub mod get_api_version_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetApiVersionSetArgs {
        /// The name of the API Management Service where the API Version Set exists.
        #[builder(into)]
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The name of the API Version Set.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the parent API Management Service exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetApiVersionSetResult {
        pub api_management_name: pulumi_wasm_rust::Output<String>,
        /// The description of API Version Set.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of this API Version Set.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Header which should be read from Inbound Requests which defines the API Version.
        pub version_header_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Query String which should be read from Inbound Requests which defines the API Version.
        pub version_query_name: pulumi_wasm_rust::Output<String>,
        pub versioning_scheme: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetApiVersionSetArgs) -> GetApiVersionSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_name_binding = args.api_management_name.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getApiVersionSet:getApiVersionSet".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementName".into(),
                    value: &api_management_name_binding,
                },
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
                    name: "apiManagementName".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "versionHeaderName".into(),
                },
                register_interface::ResultField {
                    name: "versionQueryName".into(),
                },
                register_interface::ResultField {
                    name: "versioningScheme".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetApiVersionSetResult {
            api_management_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiManagementName").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            version_header_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionHeaderName").unwrap(),
            ),
            version_query_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionQueryName").unwrap(),
            ),
            versioning_scheme: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versioningScheme").unwrap(),
            ),
        }
    }
}
