pub mod get_platform_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlatformImageArgs {
        /// Specifies the Location to pull information about this Platform Image from.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the Offer associated with the Platform Image.
        #[builder(into)]
        pub offer: pulumi_wasm_rust::Output<String>,
        /// Specifies the Publisher associated with the Platform Image.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::Output<String>,
        /// Specifies the SKU of the Platform Image.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::Output<String>,
        /// The version of the Platform Image.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPlatformImageResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub offer: pulumi_wasm_rust::Output<String>,
        pub publisher: pulumi_wasm_rust::Output<String>,
        pub sku: pulumi_wasm_rust::Output<String>,
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPlatformImageArgs) -> GetPlatformImageResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_inner();
        let offer_binding = args.offer.get_inner();
        let publisher_binding = args.publisher.get_inner();
        let sku_binding = args.sku.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:compute/getPlatformImage:getPlatformImage".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "offer".into(),
                    value: &offer_binding,
                },
                register_interface::ObjectField {
                    name: "publisher".into(),
                    value: &publisher_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
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
                    name: "offer".into(),
                },
                register_interface::ResultField {
                    name: "publisher".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPlatformImageResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            offer: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("offer").unwrap(),
            ),
            publisher: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publisher").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
