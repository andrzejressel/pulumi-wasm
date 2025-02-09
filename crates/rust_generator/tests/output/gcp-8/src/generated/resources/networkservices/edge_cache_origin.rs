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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod edge_cache_origin {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EdgeCacheOriginArgs {
        /// Enable AWS Signature Version 4 origin authentication.
        /// Structure is documented below.
        #[builder(into, default)]
        pub aws_v4_authentication: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkservices::EdgeCacheOriginAwsV4Authentication,
            >,
        >,
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Origin resource to try when the current origin cannot be reached.
        /// After maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.
        /// The value of timeout.maxAttemptsTimeout dictates the timeout across all origins.
        /// A reference to a Topic resource.
        #[builder(into, default)]
        pub failover_origin: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
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
        pub max_attempts: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.
        /// This address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname
        /// When providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.
        /// If a Cloud Storage bucket is provided, it must be in the canonical "gs://bucketname" format. Other forms, such as "storage.googleapis.com", will be rejected.
        #[builder(into)]
        pub origin_address: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The override actions, including url rewrites and header
        /// additions, for requests that use this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub origin_override_action: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networkservices::EdgeCacheOriginOriginOverrideAction,
            >,
        >,
        /// Follow redirects from this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub origin_redirect: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkservices::EdgeCacheOriginOriginRedirect>,
        >,
        /// The port to connect to the origin on.
        /// Defaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.
        /// When using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server.
        /// Possible values are: `HTTP2`, `HTTPS`, `HTTP`.
        #[builder(into, default)]
        pub protocol: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub retry_conditions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The connection and HTTP timeout configuration for this origin.
        /// Structure is documented below.
        #[builder(into, default)]
        pub timeout: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkservices::EdgeCacheOriginTimeout>,
        >,
    }
    #[allow(dead_code)]
    pub struct EdgeCacheOriginResult {
        /// Enable AWS Signature Version 4 origin authentication.
        /// Structure is documented below.
        pub aws_v4_authentication: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginAwsV4Authentication,
            >,
        >,
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Origin resource to try when the current origin cannot be reached.
        /// After maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.
        /// The value of timeout.maxAttemptsTimeout dictates the timeout across all origins.
        /// A reference to a Topic resource.
        pub failover_origin: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of label tags associated with the EdgeCache resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
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
        pub max_attempts: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.
        /// This address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname
        /// When providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.
        /// If a Cloud Storage bucket is provided, it must be in the canonical "gs://bucketname" format. Other forms, such as "storage.googleapis.com", will be rejected.
        pub origin_address: pulumi_gestalt_rust::Output<String>,
        /// The override actions, including url rewrites and header
        /// additions, for requests that use this origin.
        /// Structure is documented below.
        pub origin_override_action: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networkservices::EdgeCacheOriginOriginOverrideAction,
            >,
        >,
        /// Follow redirects from this origin.
        /// Structure is documented below.
        pub origin_redirect: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginOriginRedirect>,
        >,
        /// The port to connect to the origin on.
        /// Defaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.
        /// When using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server.
        /// Possible values are: `HTTP2`, `HTTPS`, `HTTP`.
        pub protocol: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
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
        pub retry_conditions: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The connection and HTTP timeout configuration for this origin.
        /// Structure is documented below.
        pub timeout: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheOriginTimeout>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EdgeCacheOriginArgs,
    ) -> EdgeCacheOriginResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aws_v4_authentication_binding = args
            .aws_v4_authentication
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let failover_origin_binding = args.failover_origin.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let max_attempts_binding = args.max_attempts.get_output(context);
        let name_binding = args.name.get_output(context);
        let origin_address_binding = args.origin_address.get_output(context);
        let origin_override_action_binding = args
            .origin_override_action
            .get_output(context);
        let origin_redirect_binding = args.origin_redirect.get_output(context);
        let port_binding = args.port.get_output(context);
        let project_binding = args.project.get_output(context);
        let protocol_binding = args.protocol.get_output(context);
        let retry_conditions_binding = args.retry_conditions.get_output(context);
        let timeout_binding = args.timeout.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networkservices/edgeCacheOrigin:EdgeCacheOrigin".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "awsV4Authentication".into(),
                    value: aws_v4_authentication_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "failoverOrigin".into(),
                    value: failover_origin_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxAttempts".into(),
                    value: max_attempts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originAddress".into(),
                    value: origin_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originOverrideAction".into(),
                    value: origin_override_action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "originRedirect".into(),
                    value: origin_redirect_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: port_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protocol".into(),
                    value: protocol_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retryConditions".into(),
                    value: retry_conditions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeout".into(),
                    value: timeout_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EdgeCacheOriginResult {
            aws_v4_authentication: o.get_field("awsV4Authentication"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            failover_origin: o.get_field("failoverOrigin"),
            labels: o.get_field("labels"),
            max_attempts: o.get_field("maxAttempts"),
            name: o.get_field("name"),
            origin_address: o.get_field("originAddress"),
            origin_override_action: o.get_field("originOverrideAction"),
            origin_redirect: o.get_field("originRedirect"),
            port: o.get_field("port"),
            project: o.get_field("project"),
            protocol: o.get_field("protocol"),
            pulumi_labels: o.get_field("pulumiLabels"),
            retry_conditions: o.get_field("retryConditions"),
            timeout: o.get_field("timeout"),
        }
    }
}
