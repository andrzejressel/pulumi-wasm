pub mod get_client {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClientArgs {
        /// The name of the brand.
        #[builder(into)]
        pub brand: pulumi_wasm_rust::InputOrOutput<String>,
        /// The client_id of the brand.
        #[builder(into)]
        pub client_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetClientResult {
        pub brand: pulumi_wasm_rust::Output<String>,
        pub client_id: pulumi_wasm_rust::Output<String>,
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub secret: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClientArgs,
    ) -> GetClientResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let brand_binding = args.brand.get_output(context).get_inner();
        let client_id_binding = args.client_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:iap/getClient:getClient".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "brand".into(),
                    value: &brand_binding,
                },
                register_interface::ObjectField {
                    name: "clientId".into(),
                    value: &client_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClientResult {
            brand: pulumi_wasm_rust::__private::into_domain(o.extract_field("brand")),
            client_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientId"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            secret: pulumi_wasm_rust::__private::into_domain(o.extract_field("secret")),
        }
    }
}
