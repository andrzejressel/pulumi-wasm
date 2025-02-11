#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_response_headers_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResponseHeadersPolicyArgs {
        /// Identifier for the response headers policy.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name to identify the response headers policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetResponseHeadersPolicyResult {
        /// Comment to describe the response headers policy. The comment cannot be longer than 128 characters.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// Configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
        pub cors_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyCorsConfig,
            >,
        >,
        /// Object that contains an attribute `items` that contains a list of Custom Headers. See Custom Header for more information.
        pub custom_headers_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyCustomHeadersConfig,
            >,
        >,
        /// Current version of the response headers policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Object that contains an attribute `items` that contains a list of Remove Headers. See Remove Header for more information.
        pub remove_headers_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyRemoveHeadersConfig,
            >,
        >,
        /// A configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
        pub security_headers_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicySecurityHeadersConfig,
            >,
        >,
        /// (Optional) Configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
        pub server_timing_headers_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetResponseHeadersPolicyServerTimingHeadersConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResponseHeadersPolicyArgs,
    ) -> GetResponseHeadersPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getResponseHeadersPolicy:getResponseHeadersPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: &id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResponseHeadersPolicyResult {
            comment: o.get_field("comment"),
            cors_configs: o.get_field("corsConfigs"),
            custom_headers_configs: o.get_field("customHeadersConfigs"),
            etag: o.get_field("etag"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            remove_headers_configs: o.get_field("removeHeadersConfigs"),
            security_headers_configs: o.get_field("securityHeadersConfigs"),
            server_timing_headers_configs: o.get_field("serverTimingHeadersConfigs"),
        }
    }
}
