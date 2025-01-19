pub mod get_serverless_security_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessSecurityConfigArgs {
        /// The unique identifier of the security configuration.
        #[builder(into)]
        pub id: pulumi_wasm_rust::Output<String>,
        /// SAML options for the security configuration.
        #[builder(into, default)]
        pub saml_options: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::opensearch::GetServerlessSecurityConfigSamlOptions,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServerlessSecurityConfigResult {
        /// The version of the security configuration.
        pub config_version: pulumi_wasm_rust::Output<String>,
        /// The date the configuration was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// The description of the security configuration.
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date the configuration was last modified.
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        /// SAML options for the security configuration.
        pub saml_options: pulumi_wasm_rust::Output<
            Option<
                super::super::super::types::opensearch::GetServerlessSecurityConfigSamlOptions,
            >,
        >,
        /// The type of security configuration.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetServerlessSecurityConfigArgs,
    ) -> GetServerlessSecurityConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let saml_options_binding = args.saml_options.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:opensearch/getServerlessSecurityConfig:getServerlessSecurityConfig"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "samlOptions".into(),
                    value: &saml_options_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "configVersion".into(),
                },
                register_interface::ResultField {
                    name: "createdDate".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedDate".into(),
                },
                register_interface::ResultField {
                    name: "samlOptions".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetServerlessSecurityConfigResult {
            config_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configVersion").unwrap(),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdDate").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedDate").unwrap(),
            ),
            saml_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("samlOptions").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
