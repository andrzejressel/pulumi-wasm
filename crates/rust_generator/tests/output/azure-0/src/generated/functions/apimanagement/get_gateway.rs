pub mod get_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayArgs {
        /// The ID of the API Management Service in which the Gateway exists.
        #[builder(into)]
        pub api_management_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the API Management Gateway.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayResult {
        pub api_management_id: pulumi_wasm_rust::Output<String>,
        /// The description of the API Management Gateway.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `location_data` block as documented below.
        pub location_datas: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::apimanagement::GetGatewayLocationData>,
        >,
        /// A canonical name for the geographic or physical location.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGatewayArgs,
    ) -> GetGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_management_id_binding = args
            .api_management_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:apimanagement/getGateway:getGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiManagementId".into(),
                    value: &api_management_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGatewayResult {
            api_management_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("apiManagementId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            location_datas: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationDatas"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
        }
    }
}
