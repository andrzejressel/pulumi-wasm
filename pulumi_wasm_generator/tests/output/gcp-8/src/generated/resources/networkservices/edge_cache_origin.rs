/// EdgeCacheOrigin represents a HTTP-reachable backend for an EdgeCacheService.
///
///
/// To get more information about EdgeCacheOrigin, see:
///
/// * [API documentation](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins)
///
/// ## Example Usage
///
/// ### Network Services Edge Cache Origin Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = edge_cache_origin::create(
///         "default",
///         EdgeCacheOriginArgs::builder()
///             .description("The default bucket for media edge test")
///             .name("my-origin")
///             .origin_address("gs://media-edge-default")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Network Services Edge Cache Origin Advanced
///
///
/// ```yaml
/// resources:
///   fallback:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-fallback
///       originAddress: fallback.example.com
///       description: The default bucket for media edge test
///       maxAttempts: 3
///       protocol: HTTP
///       port: 80
///       retryConditions:
///         - CONNECT_FAILURE
///         - NOT_FOUND
///         - HTTP_5XX
///         - FORBIDDEN
///       timeout:
///         connectTimeout: 10s
///         maxAttemptsTimeout: 20s
///         responseTimeout: 60s
///         readTimeout: 5s
///       originOverrideAction:
///         urlRewrite:
///           hostRewrite: example.com
///         headerAction:
///           requestHeadersToAdds:
///             - headerName: x-header
///               headerValue: value
///               replace: true
///       originRedirect:
///         redirectConditions:
///           - MOVED_PERMANENTLY
///           - FOUND
///           - SEE_OTHER
///           - TEMPORARY_REDIRECT
///           - PERMANENT_REDIRECT
///   default:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-origin
///       originAddress: gs://media-edge-default
///       failoverOrigin: ${fallback.id}
///       description: The default bucket for media edge test
///       maxAttempts: 2
///       labels:
///         a: b
///       timeout:
///         connectTimeout: 10s
/// ```
/// ### Network Services Edge Cache Origin V4auth
///
///
/// ```yaml
/// resources:
///   secret-basic:
///     type: gcp:secretmanager:Secret
///     properties:
///       secretId: secret-name
///       replication:
///         auto: {}
///   secret-version-basic:
///     type: gcp:secretmanager:SecretVersion
///     properties:
///       secret: ${["secret-basic"].id}
///       secretData: secret-data
///   default:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-origin
///       originAddress: gs://media-edge-default
///       description: The default bucket for V4 authentication
///       awsV4Authentication:
///         accessKeyId: ACCESSKEYID
///         secretAccessKeyVersion: ${["secret-version-basic"].id}
///         originRegion: auto
/// ```
///
/// ## Import
///
/// EdgeCacheOrigin can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/edgeCacheOrigins/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EdgeCacheOrigin can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheOrigin:EdgeCacheOrigin default projects/{{project}}/locations/global/edgeCacheOrigins/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheOrigin:EdgeCacheOrigin default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheOrigin:EdgeCacheOrigin default {{name}}
/// ```
///
pub mod edge_cache_origin {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EdgeCacheOriginArgs {
        /// Enable AWS Signature Version 4 origin authentication.
        /// Structure is documented below.
        #[builder(into, default)]
        pub aws_v4_authentication: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginAwsV4Authentication,
            >,
        >,
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The Origin resource to try when the current origin cannot be reached.
        /// After maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.
        /// The value of timeout.maxAttemptsTimeout dictates the timeout across all origins.
        /// A reference to a Topic resource.
        #[builder(into, default)]
        pub failover_origin: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.
        /// Once maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,
        /// retryConditions and failoverOrigin to control its own cache fill failures.
        /// The total number of allowed attempts to cache fill across this and failover origins is limited to four.
        /// The total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.
        /// The last valid, non-retried response from all origins will be returned to the client.
        /// If no origin returns a valid response, an HTTP 502 will be returned to the client.
        /// Defaults to 1. Must be a value greater than 0 and less than 4.
        #[builder(into, default)]
        pub max_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.
        /// This address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname
        /// When providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.
        /// If a Cloud Storage bucket is provided, it must be in the canonical "gs://bucketname" format. Other forms, such as "storage.googleapis.com", will be rejected.
        #[builder(into)]
        pub origin_address: pulumi_wasm_rust::Output<String>,
        /// The override actions, including url rewrites and header
        /// additions, for requests that use this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub origin_override_action: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginOriginOverrideAction,
            >,
        >,
        /// Follow redirects from this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub origin_redirect: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginOriginRedirect>,
        >,
        /// The port to connect to the origin on.
        /// Defaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP.
        #[builder(into, default)]
        pub port: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.
        /// When using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server.
        /// Possible values are: `HTTP2`, `HTTPS`, `HTTP`.
        #[builder(into, default)]
        pub protocol: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies one or more retry conditions for the configured origin.
        /// If the failure mode during a connection attempt to the origin matches the configured retryCondition(s),
        /// the origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.
        /// The default retryCondition is "CONNECT_FAILURE".
        /// retryConditions apply to this origin, and not subsequent failoverOrigin(s),
        /// which may specify their own retryConditions and maxAttempts.
        /// Valid values are:
        /// - CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.
        /// - HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.
        /// - GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.
        /// - RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)
        /// - NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.
        /// - FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden).
        /// Each value may be one of: `CONNECT_FAILURE`, `HTTP_5XX`, `GATEWAY_ERROR`, `RETRIABLE_4XX`, `NOT_FOUND`, `FORBIDDEN`.
        #[builder(into, default)]
        pub retry_conditions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The connection and HTTP timeout configuration for this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub timeout: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginTimeout>,
        >,
    }
    #[allow(dead_code)]
    pub struct EdgeCacheOriginResult {
        /// Enable AWS Signature Version 4 origin authentication.
        /// Structure is documented below.
        pub aws_v4_authentication: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginAwsV4Authentication,
            >,
        >,
        /// A human-readable description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Origin resource to try when the current origin cannot be reached.
        /// After maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.
        /// The value of timeout.maxAttemptsTimeout dictates the timeout across all origins.
        /// A reference to a Topic resource.
        pub failover_origin: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.
        /// Once maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,
        /// retryConditions and failoverOrigin to control its own cache fill failures.
        /// The total number of allowed attempts to cache fill across this and failover origins is limited to four.
        /// The total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.
        /// The last valid, non-retried response from all origins will be returned to the client.
        /// If no origin returns a valid response, an HTTP 502 will be returned to the client.
        /// Defaults to 1. Must be a value greater than 0 and less than 4.
        pub max_attempts: pulumi_wasm_rust::Output<Option<i32>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.
        /// This address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname
        /// When providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.
        /// If a Cloud Storage bucket is provided, it must be in the canonical "gs://bucketname" format. Other forms, such as "storage.googleapis.com", will be rejected.
        pub origin_address: pulumi_wasm_rust::Output<String>,
        /// The override actions, including url rewrites and header
        /// additions, for requests that use this origin.
        /// Structure is documented below.
        pub origin_override_action: pulumi_wasm_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginOriginOverrideAction,
            >,
        >,
        /// Follow redirects from this origin.
        /// Structure is documented below.
        pub origin_redirect: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginOriginRedirect>,
        >,
        /// The port to connect to the origin on.
        /// Defaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP.
        pub port: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.
        /// When using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server.
        /// Possible values are: `HTTP2`, `HTTPS`, `HTTP`.
        pub protocol: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies one or more retry conditions for the configured origin.
        /// If the failure mode during a connection attempt to the origin matches the configured retryCondition(s),
        /// the origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.
        /// The default retryCondition is "CONNECT_FAILURE".
        /// retryConditions apply to this origin, and not subsequent failoverOrigin(s),
        /// which may specify their own retryConditions and maxAttempts.
        /// Valid values are:
        /// - CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.
        /// - HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.
        /// - GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.
        /// - RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)
        /// - NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.
        /// - FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden).
        /// Each value may be one of: `CONNECT_FAILURE`, `HTTP_5XX`, `GATEWAY_ERROR`, `RETRIABLE_4XX`, `NOT_FOUND`, `FORBIDDEN`.
        pub retry_conditions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The connection and HTTP timeout configuration for this origin.
        /// Structure is documented below.
        pub timeout: pulumi_wasm_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginTimeout>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EdgeCacheOriginArgs) -> EdgeCacheOriginResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let aws_v4_authentication_binding = args.aws_v4_authentication.get_inner();
        let description_binding = args.description.get_inner();
        let failover_origin_binding = args.failover_origin.get_inner();
        let labels_binding = args.labels.get_inner();
        let max_attempts_binding = args.max_attempts.get_inner();
        let name_binding = args.name.get_inner();
        let origin_address_binding = args.origin_address.get_inner();
        let origin_override_action_binding = args.origin_override_action.get_inner();
        let origin_redirect_binding = args.origin_redirect.get_inner();
        let port_binding = args.port.get_inner();
        let project_binding = args.project.get_inner();
        let protocol_binding = args.protocol.get_inner();
        let retry_conditions_binding = args.retry_conditions.get_inner();
        let timeout_binding = args.timeout.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/edgeCacheOrigin:EdgeCacheOrigin".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsV4Authentication".into(),
                    value: &aws_v4_authentication_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "failoverOrigin".into(),
                    value: &failover_origin_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "maxAttempts".into(),
                    value: &max_attempts_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "originAddress".into(),
                    value: &origin_address_binding,
                },
                register_interface::ObjectField {
                    name: "originOverrideAction".into(),
                    value: &origin_override_action_binding,
                },
                register_interface::ObjectField {
                    name: "originRedirect".into(),
                    value: &origin_redirect_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "protocol".into(),
                    value: &protocol_binding,
                },
                register_interface::ObjectField {
                    name: "retryConditions".into(),
                    value: &retry_conditions_binding,
                },
                register_interface::ObjectField {
                    name: "timeout".into(),
                    value: &timeout_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "awsV4Authentication".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "failoverOrigin".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "maxAttempts".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "originAddress".into(),
                },
                register_interface::ResultField {
                    name: "originOverrideAction".into(),
                },
                register_interface::ResultField {
                    name: "originRedirect".into(),
                },
                register_interface::ResultField {
                    name: "port".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "protocol".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "retryConditions".into(),
                },
                register_interface::ResultField {
                    name: "timeout".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EdgeCacheOriginResult {
            aws_v4_authentication: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsV4Authentication").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            failover_origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failoverOrigin").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            max_attempts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxAttempts").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            origin_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originAddress").unwrap(),
            ),
            origin_override_action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originOverrideAction").unwrap(),
            ),
            origin_redirect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("originRedirect").unwrap(),
            ),
            port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("port").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protocol").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            retry_conditions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retryConditions").unwrap(),
            ),
            timeout: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeout").unwrap(),
            ),
        }
    }
}
