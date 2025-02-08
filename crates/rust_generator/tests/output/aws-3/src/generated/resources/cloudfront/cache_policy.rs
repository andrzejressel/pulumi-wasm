/// ## Example Usage
///
/// Use the `aws.cloudfront.CachePolicy` resource to create a cache policy for CloudFront.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cache_policy::create(
///         "example",
///         CachePolicyArgs::builder()
///             .comment("test comment")
///             .default_ttl(50)
///             .max_ttl(100)
///             .min_ttl(1)
///             .name("example-policy")
///             .parameters_in_cache_key_and_forwarded_to_origin(
///                 CachePolicyParametersInCacheKeyAndForwardedToOrigin::builder()
///                     .cookiesConfig(
///                         CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig::builder()
///                             .cookieBehavior("whitelist")
///                             .cookies(
///                                 CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies::builder()
///                                     .items(vec!["example",])
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .headersConfig(
///                         CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig::builder()
///                             .headerBehavior("whitelist")
///                             .headers(
///                                 CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders::builder()
///                                     .items(vec!["example",])
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .queryStringsConfig(
///                         CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig::builder()
///                             .queryStringBehavior("whitelist")
///                             .queryStrings(
///                                 CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings::builder()
///                                     .items(vec!["example",])
///                                     .build_struct(),
///                             )
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
/// Using `pulumi import`, import CloudFront cache policies using the `id` of the cache policy. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/cachePolicy:CachePolicy policy 658327ea-f89d-4fab-a63d-7e88639e58f6
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod cache_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CachePolicyArgs {
        /// Description for the cache policy.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Amount of time, in seconds, that objects are allowed to remain in the CloudFront cache before CloudFront sends a new request to the origin server to check if the object has been updated.
        #[builder(into, default)]
        pub default_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        #[builder(into, default)]
        pub max_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Minimum amount of time, in seconds, that objects should remain in the CloudFront cache before a new request is sent to the origin to check for updates.
        #[builder(into, default)]
        pub min_ttl: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Unique name used to identify the cache policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration for including HTTP headers, cookies, and URL query strings in the cache key. For more information, refer to the Parameters In Cache Key And Forwarded To Origin section.
        #[builder(into)]
        pub parameters_in_cache_key_and_forwarded_to_origin: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOrigin,
        >,
    }
    #[allow(dead_code)]
    pub struct CachePolicyResult {
        /// Description for the cache policy.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amount of time, in seconds, that objects are allowed to remain in the CloudFront cache before CloudFront sends a new request to the origin server to check if the object has been updated.
        pub default_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Current version of the cache policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub max_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Minimum amount of time, in seconds, that objects should remain in the CloudFront cache before a new request is sent to the origin to check for updates.
        pub min_ttl: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Unique name used to identify the cache policy.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Configuration for including HTTP headers, cookies, and URL query strings in the cache key. For more information, refer to the Parameters In Cache Key And Forwarded To Origin section.
        pub parameters_in_cache_key_and_forwarded_to_origin: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOrigin,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CachePolicyArgs,
    ) -> CachePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_output(context).get_inner();
        let default_ttl_binding = args.default_ttl.get_output(context).get_inner();
        let max_ttl_binding = args.max_ttl.get_output(context).get_inner();
        let min_ttl_binding = args.min_ttl.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameters_in_cache_key_and_forwarded_to_origin_binding = args
            .parameters_in_cache_key_and_forwarded_to_origin
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/cachePolicy:CachePolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "defaultTtl".into(),
                    value: &default_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "maxTtl".into(),
                    value: &max_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "minTtl".into(),
                    value: &min_ttl_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parametersInCacheKeyAndForwardedToOrigin".into(),
                    value: &parameters_in_cache_key_and_forwarded_to_origin_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CachePolicyResult {
            comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            default_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultTtl"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            max_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxTtl"),
            ),
            min_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minTtl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters_in_cache_key_and_forwarded_to_origin: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parametersInCacheKeyAndForwardedToOrigin"),
            ),
        }
    }
}
