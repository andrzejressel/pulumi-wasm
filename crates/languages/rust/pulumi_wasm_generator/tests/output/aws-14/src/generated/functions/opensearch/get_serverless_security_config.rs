pub mod get_serverless_security_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessSecurityConfigArgs {
        /// The unique identifier of the security configuration.
        #[builder(into)]
        pub id: pulumi_wasm_rust::InputOrOutput<String>,
        /// SAML options for the security configuration.
        #[builder(into, default)]
        pub saml_options: pulumi_wasm_rust::InputOrOutput<
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
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServerlessSecurityConfigArgs,
    ) -> GetServerlessSecurityConfigResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let saml_options_binding = args.saml_options.get_output(context).get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerlessSecurityConfigResult {
            config_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configVersion"),
            ),
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedDate"),
            ),
            saml_options: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("samlOptions"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
