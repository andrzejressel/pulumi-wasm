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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetResponseHeadersPolicyArgs,
    ) -> GetResponseHeadersPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getResponseHeadersPolicy:getResponseHeadersPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetResponseHeadersPolicyResult {
            comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            cors_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("corsConfigs"),
            ),
            custom_headers_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customHeadersConfigs"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            remove_headers_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("removeHeadersConfigs"),
            ),
            security_headers_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityHeadersConfigs"),
            ),
            server_timing_headers_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverTimingHeadersConfigs"),
            ),
        }
    }
}
