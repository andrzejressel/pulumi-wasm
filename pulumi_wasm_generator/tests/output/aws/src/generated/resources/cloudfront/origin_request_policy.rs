/// ## Example Usage
///
/// The following example below creates a CloudFront origin request policy.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod origin_request_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginRequestPolicyArgs {
        /// Comment to describe the origin request policy.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        #[builder(into)]
        pub cookies_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyCookiesConfig,
        >,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        #[builder(into)]
        pub headers_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyHeadersConfig,
        >,
        /// Unique name to identify the origin request policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        #[builder(into)]
        pub query_strings_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyQueryStringsConfig,
        >,
    }
    #[allow(dead_code)]
    pub struct OriginRequestPolicyResult {
        /// Comment to describe the origin request policy.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
        pub cookies_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyCookiesConfig,
        >,
        /// The current version of the origin request policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// Object that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
        pub headers_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyHeadersConfig,
        >,
        /// Unique name to identify the origin request policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
        pub query_strings_config: pulumi_wasm_rust::Output<
            super::super::types::cloudfront::OriginRequestPolicyQueryStringsConfig,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: OriginRequestPolicyArgs,
    ) -> OriginRequestPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_inner();
        let cookies_config_binding = args.cookies_config.get_inner();
        let headers_config_binding = args.headers_config.get_inner();
        let name_binding = args.name.get_inner();
        let query_strings_config_binding = args.query_strings_config.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/originRequestPolicy:OriginRequestPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "cookiesConfig".into(),
                    value: &cookies_config_binding,
                },
                register_interface::ObjectField {
                    name: "headersConfig".into(),
                    value: &headers_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "queryStringsConfig".into(),
                    value: &query_strings_config_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "comment".into(),
                },
                register_interface::ResultField {
                    name: "cookiesConfig".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "headersConfig".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queryStringsConfig".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OriginRequestPolicyResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("comment").unwrap(),
            ),
            cookies_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cookiesConfig").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            headers_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("headersConfig").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            query_strings_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queryStringsConfig").unwrap(),
            ),
        }
    }
}