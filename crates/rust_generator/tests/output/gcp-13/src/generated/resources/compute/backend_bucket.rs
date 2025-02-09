/// Backend buckets allow you to use Google Cloud Storage buckets with HTTP(S)
/// load balancing.
///
/// An HTTP(S) load balancer can direct traffic to specified URLs to a
/// backend bucket rather than a backend service. It can send requests for
/// static content to a Cloud Storage bucket and requests for dynamic content
/// to a virtual machine instance.
///
///
/// To get more information about BackendBucket, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/backendBuckets)
/// * How-to Guides
///     * [Using a Cloud Storage bucket as a load balancer backend](https://cloud.google.com/compute/docs/load-balancing/http/backend-bucket)
///
/// ## Example Usage
///
/// ### Backend Bucket Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let imageBackend = backend_bucket::create(
///         "imageBackend",
///         BackendBucketArgs::builder()
///             .bucket_name("${imageBucket.name}")
///             .description("Contains beautiful images")
///             .enable_cdn(true)
///             .name("image-backend-bucket")
///             .build_struct(),
///     );
///     let imageBucket = bucket::create(
///         "imageBucket",
///         BucketArgs::builder().location("EU").name("image-store-bucket").build_struct(),
///     );
/// }
/// ```
/// ### Backend Bucket Security Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let imageBackend = backend_bucket::create(
///         "imageBackend",
///         BackendBucketArgs::builder()
///             .bucket_name("${imageBackendBucket.name}")
///             .description("Contains beautiful images")
///             .edge_security_policy("${policy.id}")
///             .enable_cdn(true)
///             .name("image-backend-bucket")
///             .build_struct(),
///     );
///     let imageBackendBucket = bucket::create(
///         "imageBackendBucket",
///         BucketArgs::builder().location("EU").name("image-store-bucket").build_struct(),
///     );
///     let policy = security_policy::create(
///         "policy",
///         SecurityPolicyArgs::builder()
///             .description("basic security policy")
///             .name("image-store-bucket")
///             .type_("CLOUD_ARMOR_EDGE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Backend Bucket Query String Whitelist
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let imageBackend = backend_bucket::create(
///         "imageBackend",
///         BackendBucketArgs::builder()
///             .bucket_name("${imageBucket.name}")
///             .cdn_policy(
///                 BackendBucketCdnPolicy::builder()
///                     .cacheKeyPolicy(
///                         BackendBucketCdnPolicyCacheKeyPolicy::builder()
///                             .queryStringWhitelists(vec!["image-version",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("Contains beautiful images")
///             .enable_cdn(true)
///             .name("image-backend-bucket")
///             .build_struct(),
///     );
///     let imageBucket = bucket::create(
///         "imageBucket",
///         BucketArgs::builder().location("EU").name("image-backend-bucket").build_struct(),
///     );
/// }
/// ```
/// ### Backend Bucket Include Http Headers
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let imageBackend = backend_bucket::create(
///         "imageBackend",
///         BackendBucketArgs::builder()
///             .bucket_name("${imageBucket.name}")
///             .cdn_policy(
///                 BackendBucketCdnPolicy::builder()
///                     .cacheKeyPolicy(
///                         BackendBucketCdnPolicyCacheKeyPolicy::builder()
///                             .includeHttpHeaders(vec!["X-My-Header-Field",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("Contains beautiful images")
///             .enable_cdn(true)
///             .name("image-backend-bucket")
///             .build_struct(),
///     );
///     let imageBucket = bucket::create(
///         "imageBucket",
///         BucketArgs::builder().location("EU").name("image-backend-bucket").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// BackendBucket can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/backendBuckets/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, BackendBucket can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/backendBucket:BackendBucket default projects/{{project}}/global/backendBuckets/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/backendBucket:BackendBucket default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/backendBucket:BackendBucket default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backend_bucket {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackendBucketArgs {
        /// Cloud Storage bucket name.
        #[builder(into)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Cloud CDN configuration for this Backend Bucket.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cdn_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::BackendBucketCdnPolicy>,
        >,
        /// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
        /// Possible values are: `AUTOMATIC`, `DISABLED`.
        #[builder(into, default)]
        pub compression_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Headers that the HTTP/S load balancer should add to proxied responses.
        #[builder(into, default)]
        pub custom_response_headers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An optional textual description of the resource; provided by the
        /// client when the resource is created.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security policy associated with this backend bucket.
        #[builder(into, default)]
        pub edge_security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If true, enable Cloud CDN for this BackendBucket.
        #[builder(into, default)]
        pub enable_cdn: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackendBucketResult {
        /// Cloud Storage bucket name.
        pub bucket_name: pulumi_gestalt_rust::Output<String>,
        /// Cloud CDN configuration for this Backend Bucket.
        /// Structure is documented below.
        pub cdn_policy: pulumi_gestalt_rust::Output<
            super::super::types::compute::BackendBucketCdnPolicy,
        >,
        /// Compress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header.
        /// Possible values are: `AUTOMATIC`, `DISABLED`.
        pub compression_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Headers that the HTTP/S load balancer should add to proxied responses.
        pub custom_response_headers: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An optional textual description of the resource; provided by the
        /// client when the resource is created.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The security policy associated with this backend bucket.
        pub edge_security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// If true, enable Cloud CDN for this BackendBucket.
        pub enable_cdn: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035.  Specifically, the name must be 1-63 characters long and
        /// match the regular expression `a-z?` which means
        /// the first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the
        /// last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BackendBucketArgs,
    ) -> BackendBucketResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bucket_name_binding_1 = args.bucket_name.get_output(context);
        let bucket_name_binding = bucket_name_binding_1.get_inner();
        let cdn_policy_binding_1 = args.cdn_policy.get_output(context);
        let cdn_policy_binding = cdn_policy_binding_1.get_inner();
        let compression_mode_binding_1 = args.compression_mode.get_output(context);
        let compression_mode_binding = compression_mode_binding_1.get_inner();
        let custom_response_headers_binding_1 = args
            .custom_response_headers
            .get_output(context);
        let custom_response_headers_binding = custom_response_headers_binding_1
            .get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let edge_security_policy_binding_1 = args
            .edge_security_policy
            .get_output(context);
        let edge_security_policy_binding = edge_security_policy_binding_1.get_inner();
        let enable_cdn_binding_1 = args.enable_cdn.get_output(context);
        let enable_cdn_binding = enable_cdn_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/backendBucket:BackendBucket".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding,
                },
                register_interface::ObjectField {
                    name: "cdnPolicy".into(),
                    value: &cdn_policy_binding,
                },
                register_interface::ObjectField {
                    name: "compressionMode".into(),
                    value: &compression_mode_binding,
                },
                register_interface::ObjectField {
                    name: "customResponseHeaders".into(),
                    value: &custom_response_headers_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "edgeSecurityPolicy".into(),
                    value: &edge_security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "enableCdn".into(),
                    value: &enable_cdn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BackendBucketResult {
            bucket_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bucketName"),
            ),
            cdn_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cdnPolicy"),
            ),
            compression_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("compressionMode"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            custom_response_headers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customResponseHeaders"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            edge_security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSecurityPolicy"),
            ),
            enable_cdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCdn"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
        }
    }
}
