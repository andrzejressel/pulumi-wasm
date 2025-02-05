pub mod get_authorization_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenArgs {
        /// AWS account ID of the ECR Repository. If not specified the default account is assumed.
        #[builder(into, default)]
        pub registry_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAuthorizationTokenResult {
        /// Temporary IAM authentication credentials to access the ECR repository encoded in base64 in the form of `user_name:password`.
        pub authorization_token: pulumi_wasm_rust::Output<String>,
        /// Time in UTC RFC3339 format when the authorization token expires.
        pub expires_at: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Password decoded from the authorization token.
        pub password: pulumi_wasm_rust::Output<String>,
        /// Registry URL to use in the docker login command.
        pub proxy_endpoint: pulumi_wasm_rust::Output<String>,
        pub registry_id: pulumi_wasm_rust::Output<Option<String>>,
        /// User name decoded from the authorization token.
        pub user_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetAuthorizationTokenArgs,
    ) -> GetAuthorizationTokenResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let registry_id_binding = args.registry_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ecr/getAuthorizationToken:getAuthorizationToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "registryId".into(),
                    value: &registry_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetAuthorizationTokenResult {
            authorization_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authorizationToken"),
            ),
            expires_at: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("expiresAt"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            proxy_endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("proxyEndpoint"),
            ),
            registry_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("registryId"),
            ),
            user_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("userName"),
            ),
        }
    }
}
