/// Resource for managing an AWS OpenSearch Serverless Security Config.
///
/// ## Example Usage
///
/// ## Import
///
/// Using `pulumi import`, import OpenSearchServerless Access Policy using the `name` argument prefixed with the string `saml/account_id/`. For example:
///
/// ```sh
/// $ pulumi import aws:opensearch/serverlessSecurityConfig:ServerlessSecurityConfig example saml/123456789012/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod serverless_security_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerlessSecurityConfigArgs {
        /// Description of the security configuration.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block for SAML options.
        #[builder(into, default)]
        pub saml_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::opensearch::ServerlessSecurityConfigSamlOptions>,
        >,
        /// Type of configuration. Must be `saml`.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerlessSecurityConfigResult {
        /// Version of the configuration.
        pub config_version: pulumi_gestalt_rust::Output<String>,
        /// Description of the security configuration.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for SAML options.
        pub saml_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::opensearch::ServerlessSecurityConfigSamlOptions>,
        >,
        /// Type of configuration. Must be `saml`.
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServerlessSecurityConfigArgs,
    ) -> ServerlessSecurityConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let saml_options_binding = args.saml_options.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:opensearch/serverlessSecurityConfig:ServerlessSecurityConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samlOptions".into(),
                    value: saml_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServerlessSecurityConfigResult {
            config_version: o.get_field("configVersion"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            saml_options: o.get_field("samlOptions"),
            type_: o.get_field("type"),
        }
    }
}
