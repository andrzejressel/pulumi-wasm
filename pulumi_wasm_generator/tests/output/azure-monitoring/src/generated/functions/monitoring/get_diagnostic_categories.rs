pub mod get_diagnostic_categories {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDiagnosticCategoriesArgs {
        /// The ID of an existing Resource which Monitor Diagnostics Categories should be retrieved for.
        #[builder(into)]
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetDiagnosticCategoriesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of the supported log category groups of this resource to send to the destination.
        pub log_category_groups: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of the supported log category types of this resource to send to the destination.
        pub log_category_types: pulumi_wasm_rust::Output<Vec<String>>,
        /// A list of the Metric Categories supported for this Resource.
        pub metrics: pulumi_wasm_rust::Output<Vec<String>>,
        pub resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDiagnosticCategoriesArgs) -> GetDiagnosticCategoriesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let resource_id_binding = args.resource_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:monitoring/getDiagnosticCategories:getDiagnosticCategories"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "resourceId".into(),
                    value: &resource_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "logCategoryGroups".into(),
                },
                register_interface::ResultField {
                    name: "logCategoryTypes".into(),
                },
                register_interface::ResultField {
                    name: "metrics".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDiagnosticCategoriesResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            log_category_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logCategoryGroups").unwrap(),
            ),
            log_category_types: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("logCategoryTypes").unwrap(),
            ),
            metrics: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metrics").unwrap(),
            ),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
        }
    }
}
