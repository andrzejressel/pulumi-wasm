#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_serverless_security_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessSecurityConfigArgs {
        /// The unique identifier of the security configuration.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// SAML options for the security configuration.
        #[builder(into, default)]
        pub saml_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::super::types::opensearch::GetServerlessSecurityConfigSamlOptions,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetServerlessSecurityConfigResult {
        /// The version of the security configuration.
        pub config_version: pulumi_gestalt_rust::Output<String>,
        /// The date the configuration was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// The description of the security configuration.
        pub description: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date the configuration was last modified.
        pub last_modified_date: pulumi_gestalt_rust::Output<String>,
        /// SAML options for the security configuration.
        pub saml_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::super::types::opensearch::GetServerlessSecurityConfigSamlOptions,
            >,
        >,
        /// The type of security configuration.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetServerlessSecurityConfigArgs,
    ) -> GetServerlessSecurityConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let saml_options_binding = args.saml_options.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:opensearch/getServerlessSecurityConfig:getServerlessSecurityConfig"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samlOptions".into(),
                    value: &saml_options_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetServerlessSecurityConfigResult {
            config_version: o.get_field("configVersion"),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            last_modified_date: o.get_field("lastModifiedDate"),
            saml_options: o.get_field("samlOptions"),
            type_: o.get_field("type"),
        }
    }
}
