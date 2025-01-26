pub mod get_credentials {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCredentialsArgs {
        #[builder(into)]
        pub registry_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCredentialsResult {
        pub authorization_token: pulumi_wasm_rust::Output<String>,
        pub expires_at: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub proxy_endpoint: pulumi_wasm_rust::Output<String>,
        pub registry_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetCredentialsArgs,
    ) -> GetCredentialsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let registry_id_binding = args.registry_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getCredentials:getCredentials".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCredentialsResult {
            authorization_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationToken"),
            ),
            expires_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiresAt"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            proxy_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proxyEndpoint"),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
        }
    }
}
