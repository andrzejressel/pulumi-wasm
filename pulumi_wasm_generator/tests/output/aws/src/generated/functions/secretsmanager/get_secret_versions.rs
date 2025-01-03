pub mod get_secret_versions {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecretVersionsArgs {
        /// If true, all deprecated secret versions are included in the response.
        /// If false, no deprecated secret versions are included in the response. If no value is specified, the default value is `false`.
        #[builder(into, default)]
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the secret containing the version that you want to retrieve. You can specify either the ARN or the friendly name of the secret.
        #[builder(into)]
        pub secret_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSecretVersionsResult {
        /// ARN of the secret.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_deprecated: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub secret_id: pulumi_wasm_rust::Output<String>,
        /// List of the versions of the secret. Attributes are specified below.
        pub versions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::secretsmanager::GetSecretVersionsVersion>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSecretVersionsArgs) -> GetSecretVersionsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let include_deprecated_binding = args.include_deprecated.get_inner();
        let secret_id_binding = args.secret_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:secretsmanager/getSecretVersions:getSecretVersions".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "includeDeprecated".into(),
                    value: &include_deprecated_binding,
                },
                register_interface::ObjectField {
                    name: "secretId".into(),
                    value: &secret_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includeDeprecated".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "secretId".into(),
                },
                register_interface::ResultField {
                    name: "versions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecretVersionsResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_deprecated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeDeprecated").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            secret_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretId").unwrap(),
            ),
            versions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versions").unwrap(),
            ),
        }
    }
}
