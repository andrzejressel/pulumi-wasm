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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOriginRequestPolicyArgs,
    ) -> GetOriginRequestPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let id_binding_1 = args.id.get_output(context);
        let id_binding = id_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:cloudfront/getOriginRequestPolicy:getOriginRequestPolicy".into(),
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
        GetOriginRequestPolicyResult {
            comment: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            cookies_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cookiesConfigs"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            headers_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("headersConfigs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            query_strings_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queryStringsConfigs"),
            ),
        }
    }
}
