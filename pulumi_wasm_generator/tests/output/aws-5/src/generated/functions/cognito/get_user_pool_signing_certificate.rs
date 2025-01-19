pub mod get_user_pool_signing_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetUserPoolSigningCertificateArgs {
        /// Cognito user pool ID.
        #[builder(into)]
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetUserPoolSigningCertificateResult {
        /// Certificate string
        pub certificate: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub user_pool_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetUserPoolSigningCertificateArgs,
    ) -> GetUserPoolSigningCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let user_pool_id_binding = args.user_pool_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cognito/getUserPoolSigningCertificate:getUserPoolSigningCertificate"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "userPoolId".into(),
                    value: &user_pool_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "certificate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "userPoolId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetUserPoolSigningCertificateResult {
            certificate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certificate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            user_pool_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userPoolId").unwrap(),
            ),
        }
    }
}
