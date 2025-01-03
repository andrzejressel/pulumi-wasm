pub mod get_azure_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAzureVersionsArgs {
        /// The location to list versions for.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the project to list available cluster versions for. Should match the project the cluster will be deployed to.
        /// Defaults to the project that the provider is authenticated with.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAzureVersionsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// A list of Azure regions that are available for use with this project and GCP location.
        pub supported_regions: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of versions available for use with this project and location.
        pub valid_versions: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAzureVersionsArgs) -> GetAzureVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getAzureVersions:getAzureVersions".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "supportedRegions".into(),
                },
                register_interface::ResultField {
                    name: "validVersions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAzureVersionsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            supported_regions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportedRegions").unwrap(),
            ),
            valid_versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validVersions").unwrap(),
            ),
        }
    }
}
