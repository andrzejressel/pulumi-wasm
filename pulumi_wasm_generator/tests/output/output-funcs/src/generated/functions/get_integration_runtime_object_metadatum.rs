pub mod get_integration_runtime_object_metadatum {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetIntegrationRuntimeObjectMetadatumArgs {
        /// The factory name.
        #[builder(into)]
        pub factory_name: pulumi_wasm_rust::Output<String>,
        /// The integration runtime name.
        #[builder(into)]
        pub integration_runtime_name: pulumi_wasm_rust::Output<String>,
        /// Metadata path.
        #[builder(into, default)]
        pub metadata_path: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource group name.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetIntegrationRuntimeObjectMetadatumResult {
        /// The link to the next page of results, if any remaining results exist.
        pub next_link: pulumi_wasm_rust::Output<Option<String>>,
        /// List of SSIS object metadata.
        pub value: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    pulumi_wasm_rust::OneOf4<
                        super::super::types::SsisEnvironmentResponse,
                        super::super::types::SsisFolderResponse,
                        super::super::types::SsisPackageResponse,
                        super::super::types::SsisProjectResponse,
                    >,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetIntegrationRuntimeObjectMetadatumArgs,
    ) -> GetIntegrationRuntimeObjectMetadatumResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let factory_name_binding = args.factory_name.get_inner();
        let integration_runtime_name_binding = args.integration_runtime_name.get_inner();
        let metadata_path_binding = args.metadata_path.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "mypkg::getIntegrationRuntimeObjectMetadatum".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "factoryName".into(),
                    value: &factory_name_binding,
                },
                register_interface::ObjectField {
                    name: "integrationRuntimeName".into(),
                    value: &integration_runtime_name_binding,
                },
                register_interface::ObjectField {
                    name: "metadataPath".into(),
                    value: &metadata_path_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "nextLink".into(),
                },
                register_interface::ResultField {
                    name: "value".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetIntegrationRuntimeObjectMetadatumResult {
            next_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nextLink").unwrap(),
            ),
            value: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("value").unwrap(),
            ),
        }
    }
}
