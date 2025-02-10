#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_origin_request_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginRequestPolicyArgs {
        /// Identifier for the origin request policy.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name to identify the origin request policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetOriginRequestPolicyResult {
        /// Comment to describe the origin request policy.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        pub cookies_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyCookiesConfig,
            >,
        >,
        /// Current version of the origin request policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        pub headers_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyHeadersConfig,
            >,
        >,
        pub id: pulumi_gestalt_rust::Output<Option<String>>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        pub query_strings_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetOriginRequestPolicyQueryStringsConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOriginRequestPolicyArgs,
    ) -> GetOriginRequestPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:cloudfront/getOriginRequestPolicy:getOriginRequestPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOriginRequestPolicyResult {
            comment: o.get_field("comment"),
            cookies_configs: o.get_field("cookiesConfigs"),
            etag: o.get_field("etag"),
            headers_configs: o.get_field("headersConfigs"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            query_strings_configs: o.get_field("queryStringsConfigs"),
        }
    }
}
