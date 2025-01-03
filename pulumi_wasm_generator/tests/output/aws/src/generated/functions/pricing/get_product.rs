pub mod get_product {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProductArgs {
        /// List of filters. Passed directly to the API (see GetProducts API reference). These filters must describe a single product, this resource will fail if more than one product is returned by the API.
        #[builder(into)]
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pricing::GetProductFilter>,
        >,
        /// Code of the service. Available service codes can be fetched using the DescribeServices pricing API call.
        #[builder(into)]
        pub service_code: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetProductResult {
        pub filters: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::pricing::GetProductFilter>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set to the product returned from the API.
        pub result: pulumi_wasm_rust::Output<String>,
        pub service_code: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProductArgs) -> GetProductResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let service_code_binding = args.service_code.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:pricing/getProduct:getProduct".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "serviceCode".into(),
                    value: &service_code_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "result".into(),
                },
                register_interface::ResultField {
                    name: "serviceCode".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProductResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            result: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("result").unwrap(),
            ),
            service_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceCode").unwrap(),
            ),
        }
    }
}
