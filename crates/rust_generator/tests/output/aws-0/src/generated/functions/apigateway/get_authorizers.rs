pub mod get_authorizers {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizersArgs {
        /// ID of the associated REST API.
        #[builder(into)]
        pub rest_api_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizersResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// List of Authorizer identifiers.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub rest_api_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAuthorizersArgs,
    ) -> GetAuthorizersResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let rest_api_id_binding = args.rest_api_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:apigateway/getAuthorizers:getAuthorizers".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "restApiId".into(),
                    value: &rest_api_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthorizersResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ids: pulumi_wasm_rust::__private::into_domain(o.extract_field("ids")),
            rest_api_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restApiId"),
            ),
        }
    }
}
