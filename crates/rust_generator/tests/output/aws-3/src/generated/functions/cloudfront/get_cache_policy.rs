pub mod get_cache_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCachePolicyArgs {
        /// Identifier for the cache policy.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name to identify the cache policy.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetCachePolicyResult {
        /// Comment to describe the cache policy.
        pub comment: pulumi_gestalt_rust::Output<String>,
        /// Default amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub default_ttl: pulumi_gestalt_rust::Output<i32>,
        /// Current version of the cache policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub max_ttl: pulumi_gestalt_rust::Output<i32>,
        /// Minimum amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
        pub min_ttl: pulumi_gestalt_rust::Output<i32>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The HTTP headers, cookies, and URL query strings to include in the cache key. See Parameters In Cache Key And Forwarded To Origin for more information.
        pub parameters_in_cache_key_and_forwarded_to_origins: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOrigin,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCachePolicyArgs,
    ) -> GetCachePolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getCachePolicy:getCachePolicy".into(),
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
        GetCachePolicyResult {
            comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            default_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultTtl"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            max_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxTtl"),
            ),
            min_ttl: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minTtl"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            parameters_in_cache_key_and_forwarded_to_origins: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parametersInCacheKeyAndForwardedToOrigins"),
            ),
        }
    }
}
