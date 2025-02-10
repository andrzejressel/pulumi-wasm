/// ## Example Usage
///
/// The following example below creates a CloudFront origin request policy.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = origin_request_policy::create(
///         "example",
///         OriginRequestPolicyArgs::builder()
///             .comment("example comment")
///             .cookies_config(
///                 OriginRequestPolicyCookiesConfig::builder()
///                     .cookieBehavior("whitelist")
///                     .cookies(
///                         OriginRequestPolicyCookiesConfigCookies::builder()
///                             .items(vec!["example",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .headers_config(
///                 OriginRequestPolicyHeadersConfig::builder()
///                     .headerBehavior("whitelist")
///                     .headers(
///                         OriginRequestPolicyHeadersConfigHeaders::builder()
///                             .items(vec!["example",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example-policy")
///             .query_strings_config(
///                 OriginRequestPolicyQueryStringsConfig::builder()
///                     .queryStringBehavior("whitelist")
///                     .queryStrings(
///                         OriginRequestPolicyQueryStringsConfigQueryStrings::builder()
///                             .items(vec!["example",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Origin Request Policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/originRequestPolicy:OriginRequestPolicy policy ccca32ef-dce3-4df3-80df-1bd3000bc4d3
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod origin_request_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginRequestPolicyArgs {
        /// Comment to describe the origin request policy.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        #[builder(into)]
        pub cookies_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::OriginRequestPolicyCookiesConfig,
        >,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        #[builder(into)]
        pub headers_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::OriginRequestPolicyHeadersConfig,
        >,
        /// Unique name to identify the origin request policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        #[builder(into)]
        pub query_strings_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::OriginRequestPolicyQueryStringsConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct OriginRequestPolicyResult {
        /// Comment to describe the origin request policy.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        pub cookies_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyCookiesConfig,
        >,
        /// The current version of the origin request policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        pub headers_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyHeadersConfig,
        >,
        /// Unique name to identify the origin request policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        pub query_strings_config: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyQueryStringsConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OriginRequestPolicyArgs,
    ) -> OriginRequestPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let comment_binding = args.comment.get_output(context);
        let cookies_config_binding = args.cookies_config.get_output(context);
        let headers_config_binding = args.headers_config.get_output(context);
        let name_binding = args.name.get_output(context);
        let query_strings_config_binding = args.query_strings_config.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/originRequestPolicy:OriginRequestPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: comment_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cookiesConfig".into(),
                    value: cookies_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "headersConfig".into(),
                    value: headers_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "queryStringsConfig".into(),
                    value: query_strings_config_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        OriginRequestPolicyResult {
            comment: o.get_field("comment"),
            cookies_config: o.get_field("cookiesConfig"),
            etag: o.get_field("etag"),
            headers_config: o.get_field("headersConfig"),
            name: o.get_field("name"),
            query_strings_config: o.get_field("queryStringsConfig"),
        }
    }
}
