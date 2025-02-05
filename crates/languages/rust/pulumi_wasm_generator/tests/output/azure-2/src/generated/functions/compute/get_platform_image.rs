pub mod get_platform_image {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPlatformImageArgs {
        /// Specifies the Location to pull information about this Platform Image from.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Offer associated with the Platform Image.
        #[builder(into)]
        pub offer: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the Publisher associated with the Platform Image.
        #[builder(into)]
        pub publisher: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the SKU of the Platform Image.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<String>,
        /// The version of the Platform Image.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
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
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetPlatformImageArgs,
    ) -> GetPlatformImageResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let offer_binding = args.offer.get_output(context).get_inner();
        let publisher_binding = args.publisher.get_output(context).get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let version_binding = args.version.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetPlatformImageResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            offer: pulumi_wasm_rust::__private::into_domain(o.extract_field("offer")),
            publisher: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publisher"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            version: pulumi_wasm_rust::__private::into_domain(o.extract_field("version")),
        }
    }
}
