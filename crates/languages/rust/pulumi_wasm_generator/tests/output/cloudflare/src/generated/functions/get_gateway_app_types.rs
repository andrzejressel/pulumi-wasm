pub mod get_gateway_app_types {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayAppTypesArgs {
        /// The account ID to fetch Gateway App Types from.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayAppTypesResult {
        /// The account ID to fetch Gateway App Types from.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A list of Gateway App Types.
        pub app_types: pulumi_wasm_rust::Output<
            Vec<super::super::types::GetGatewayAppTypesAppType>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetGatewayAppTypesArgs,
    ) -> GetGatewayAppTypesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getGatewayAppTypes:getGatewayAppTypes".into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetGatewayAppTypesResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accountId"),
            ),
            app_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("appTypes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
