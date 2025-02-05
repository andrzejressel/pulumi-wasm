/// Provides a CloudFront response headers policy resource.
/// A response headers policy contains information about a set of HTTP response headers and their values.
/// After you create a response headers policy, you can use its ID to attach it to one or more cache behaviors in a CloudFront distribution.
/// When it’s attached to a cache behavior, CloudFront adds the headers in the policy to every response that it sends for requests that match the cache behavior.
///
/// ## Example Usage
///
/// The example below creates a CloudFront response headers policy.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = response_headers_policy::create(
///         "example",
///         ResponseHeadersPolicyArgs::builder()
///             .comment("test comment")
///             .cors_config(
///                 ResponseHeadersPolicyCorsConfig::builder()
///                     .accessControlAllowCredentials(true)
///                     .accessControlAllowHeaders(
///                         ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders::builder()
///                             .items(vec!["test",])
///                             .build_struct(),
///                     )
///                     .accessControlAllowMethods(
///                         ResponseHeadersPolicyCorsConfigAccessControlAllowMethods::builder()
///                             .items(vec!["GET",])
///                             .build_struct(),
///                     )
///                     .accessControlAllowOrigins(
///                         ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins::builder()
///                             .items(vec!["test.example.comtest",])
///                             .build_struct(),
///                     )
///                     .originOverride(true)
///                     .build_struct(),
///             )
///             .name("example-policy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// The example below creates a CloudFront response headers policy with a custom headers config.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = response_headers_policy::create(
///         "example",
///         ResponseHeadersPolicyArgs::builder()
///             .custom_headers_config(
///                 ResponseHeadersPolicyCustomHeadersConfig::builder()
///                     .items(
///                         vec![
///                             ResponseHeadersPolicyCustomHeadersConfigItem::builder()
///                             .header("X-Permitted-Cross-Domain-Policies").override(true)
///                             .value("none").build_struct(),
///                             ResponseHeadersPolicyCustomHeadersConfigItem::builder()
///                             .header("X-Test").override(true).value("none")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example-headers-policy")
///             .build_struct(),
///     );
/// }
/// ```
///
/// The example below creates a CloudFront response headers policy with a custom headers config, remove headers config and server timing headers config.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = response_headers_policy::create(
///         "example",
///         ResponseHeadersPolicyArgs::builder()
///             .custom_headers_config(
///                 ResponseHeadersPolicyCustomHeadersConfig::builder()
///                     .items(
///                         vec![
///                             ResponseHeadersPolicyCustomHeadersConfigItem::builder()
///                             .header("X-Permitted-Cross-Domain-Policies").override(true)
///                             .value("none").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example-headers-policy")
///             .remove_headers_config(
///                 ResponseHeadersPolicyRemoveHeadersConfig::builder()
///                     .items(
///                         vec![
///                             ResponseHeadersPolicyRemoveHeadersConfigItem::builder()
///                             .header("Set-Cookie").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .server_timing_headers_config(
///                 ResponseHeadersPolicyServerTimingHeadersConfig::builder()
///                     .enabled(true)
///                     .samplingRate(50)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudfront Response Headers Policies using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/responseHeadersPolicy:ResponseHeadersPolicy policy 658327ea-f89d-4fab-a63d-7e88639e58f9
/// ```
pub mod response_headers_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ResponseHeadersPolicyArgs {
        /// A comment to describe the response headers policy. The comment cannot be longer than 128 characters.
        #[builder(into, default)]
        pub comment: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
        #[builder(into, default)]
        pub cors_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfig>,
        >,
        /// Object that contains an attribute `items` that contains a list of custom headers. See Custom Header for more information.
        #[builder(into, default)]
        pub custom_headers_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyCustomHeadersConfig,
            >,
        >,
        /// The current version of the response headers policy.
        #[builder(into, default)]
        pub etag: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A unique name to identify the response headers policy.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A configuration for a set of HTTP headers to remove from the HTTP response. Object that contains an attribute `items` that contains a list of headers. See Remove Header for more information.
        #[builder(into, default)]
        pub remove_headers_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyRemoveHeadersConfig,
            >,
        >,
        /// A configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
        #[builder(into, default)]
        pub security_headers_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfig,
            >,
        >,
        /// A configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
        #[builder(into, default)]
        pub server_timing_headers_config: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyServerTimingHeadersConfig,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ResponseHeadersPolicyResult {
        /// A comment to describe the response headers policy. The comment cannot be longer than 128 characters.
        pub comment: pulumi_wasm_rust::Output<Option<String>>,
        /// A configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
        pub cors_config: pulumi_wasm_rust::Output<
            Option<super::super::types::cloudfront::ResponseHeadersPolicyCorsConfig>,
        >,
        /// Object that contains an attribute `items` that contains a list of custom headers. See Custom Header for more information.
        pub custom_headers_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyCustomHeadersConfig,
            >,
        >,
        /// The current version of the response headers policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// A unique name to identify the response headers policy.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A configuration for a set of HTTP headers to remove from the HTTP response. Object that contains an attribute `items` that contains a list of headers. See Remove Header for more information.
        pub remove_headers_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyRemoveHeadersConfig,
            >,
        >,
        /// A configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
        pub security_headers_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicySecurityHeadersConfig,
            >,
        >,
        /// A configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
        pub server_timing_headers_config: pulumi_wasm_rust::Output<
            Option<
                super::super::types::cloudfront::ResponseHeadersPolicyServerTimingHeadersConfig,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ResponseHeadersPolicyArgs,
    ) -> ResponseHeadersPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let comment_binding = args.comment.get_output(context).get_inner();
        let cors_config_binding = args.cors_config.get_output(context).get_inner();
        let custom_headers_config_binding = args
            .custom_headers_config
            .get_output(context)
            .get_inner();
        let etag_binding = args.etag.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let remove_headers_config_binding = args
            .remove_headers_config
            .get_output(context)
            .get_inner();
        let security_headers_config_binding = args
            .security_headers_config
            .get_output(context)
            .get_inner();
        let server_timing_headers_config_binding = args
            .server_timing_headers_config
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:cloudfront/responseHeadersPolicy:ResponseHeadersPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "comment".into(),
                    value: &comment_binding,
                },
                register_interface::ObjectField {
                    name: "corsConfig".into(),
                    value: &cors_config_binding,
                },
                register_interface::ObjectField {
                    name: "customHeadersConfig".into(),
                    value: &custom_headers_config_binding,
                },
                register_interface::ObjectField {
                    name: "etag".into(),
                    value: &etag_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "removeHeadersConfig".into(),
                    value: &remove_headers_config_binding,
                },
                register_interface::ObjectField {
                    name: "securityHeadersConfig".into(),
                    value: &security_headers_config_binding,
                },
                register_interface::ObjectField {
                    name: "serverTimingHeadersConfig".into(),
                    value: &server_timing_headers_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ResponseHeadersPolicyResult {
            comment: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("comment"),
            ),
            cors_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("corsConfig"),
            ),
            custom_headers_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customHeadersConfig"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            remove_headers_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("removeHeadersConfig"),
            ),
            security_headers_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("securityHeadersConfig"),
            ),
            server_timing_headers_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverTimingHeadersConfig"),
            ),
        }
    }
}
