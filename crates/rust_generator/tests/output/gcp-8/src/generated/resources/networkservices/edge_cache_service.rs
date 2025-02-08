/// EdgeCacheService defines the IP addresses, protocols, security policies, cache policies and routing configuration.
///
///
///
/// > **Warning:** These resources require allow-listing to use, and are not openly available to all Cloud customers. Engage with your Cloud account team to discuss how to onboard.
///
/// ## Example Usage
///
/// ### Network Services Edge Cache Service Basic
///
///
/// ```yaml
/// resources:
///   dest:
///     type: gcp:storage:Bucket
///     properties:
///       name: my-bucket
///       location: US
///       forceDestroy: true
///   instance:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-origin
///       originAddress: ${dest.url}
///       description: The default bucket for media edge test
///       maxAttempts: 2
///       timeout:
///         connectTimeout: 10s
///   instanceEdgeCacheService:
///     type: gcp:networkservices:EdgeCacheService
///     name: instance
///     properties:
///       name: my-service
///       description: some description
///       routing:
///         hostRules:
///           - description: host rule description
///             hosts:
///               - sslcert.tf-test.club
///             pathMatcher: routes
///         pathMatchers:
///           - name: routes
///             routeRules:
///               - description: a route rule to match against
///                 priority: 1
///                 matchRules:
///                   - prefixMatch: /
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     cacheMode: CACHE_ALL_STATIC
///                     defaultTtl: 3600s
///                 headerAction:
///                   responseHeaderToAdds:
///                     - headerName: x-cache-status
///                       headerValue: '{cdn_cache_status}'
/// ```
/// ### Network Services Edge Cache Service Advanced
///
///
/// ```yaml
/// resources:
///   dest:
///     type: gcp:storage:Bucket
///     properties:
///       name: my-bucket
///       location: US
///       forceDestroy: true
///   google:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: origin-google
///       originAddress: google.com
///       description: The default bucket for media edge test
///       maxAttempts: 2
///       timeout:
///         connectTimeout: 10s
///   instance:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-origin
///       originAddress: ${dest.url}
///       description: The default bucket for media edge test
///       maxAttempts: 2
///       timeout:
///         connectTimeout: 10s
///   instanceEdgeCacheService:
///     type: gcp:networkservices:EdgeCacheService
///     name: instance
///     properties:
///       name: my-service
///       description: some description
///       disableQuic: true
///       disableHttp2: true
///       labels:
///         a: b
///       routing:
///         hostRules:
///           - description: host rule description
///             hosts:
///               - sslcert.tf-test.club
///             pathMatcher: routes
///           - description: host rule2
///             hosts:
///               - sslcert.tf-test2.club
///             pathMatcher: routes
///           - description: host rule3
///             hosts:
///               - sslcert.tf-test3.club
///             pathMatcher: routesAdvanced
///         pathMatchers:
///           - name: routes
///             routeRules:
///               - description: a route rule to match against
///                 priority: 1
///                 matchRules:
///                   - prefixMatch: /
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     cacheMode: CACHE_ALL_STATIC
///                     defaultTtl: 3600s
///                 headerAction:
///                   responseHeaderToAdds:
///                     - headerName: x-cache-status
///                       headerValue: '{cdn_cache_status}'
///           - name: routesAdvanced
///             description: an advanced ruleset
///             routeRules:
///               - description: an advanced route rule to match against
///                 priority: 1
///                 matchRules:
///                   - prefixMatch: /potato/
///                     queryParameterMatches:
///                       - name: debug
///                         presentMatch: true
///                       - name: state
///                         exactMatch: debug
///                   - fullPathMatch: /apple
///                 headerAction:
///                   requestHeaderToAdds:
///                     - headerName: debug
///                       headerValue: 'true'
///                       replace: true
///                     - headerName: potato
///                       headerValue: plant
///                   responseHeaderToAdds:
///                     - headerName: potato
///                       headerValue: plant
///                       replace: true
///                   requestHeaderToRemoves:
///                     - headerName: prod
///                   responseHeaderToRemoves:
///                     - headerName: prod
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     cacheMode: CACHE_ALL_STATIC
///                     defaultTtl: 3800s
///                     clientTtl: 3600s
///                     maxTtl: 9000s
///                     cacheKeyPolicy:
///                       includeProtocol: true
///                       excludeHost: true
///                       includedQueryParameters:
///                         - apple
///                         - dev
///                         - santa
///                         - claus
///                       includedHeaderNames:
///                         - banana
///                       includedCookieNames:
///                         - orange
///                     negativeCaching: true
///                     signedRequestMode: DISABLED
///                     negativeCachingPolicy:
///                       '500': 3000s
///                   urlRewrite:
///                     pathPrefixRewrite: /dev
///                     hostRewrite: dev.club
///                   corsPolicy:
///                     maxAge: 2500s
///                     allowCredentials: true
///                     allowOrigins:
///                       - '*'
///                     allowMethods:
///                       - GET
///                     allowHeaders:
///                       - dev
///                     exposeHeaders:
///                       - prod
///               - description: a second route rule to match against
///                 priority: 2
///                 matchRules:
///                   - fullPathMatch: /yay
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     cacheMode: CACHE_ALL_STATIC
///                     defaultTtl: 3600s
///                     cacheKeyPolicy:
///                       excludedQueryParameters:
///                         - dev
///                   corsPolicy:
///                     maxAge: 3000s
///                     allowHeaders:
///                       - dev
///                     disabled: true
///       logConfig:
///         enable: true
///         sampleRate: 0.01
/// ```
/// ### Network Services Edge Cache Service Dual Token
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
///   keyset:
///     type: gcp:networkservices:EdgeCacheKeyset
///     properties:
///       name: keyset-name
///       description: The default keyset
///       publicKeys:
///         - id: my-public-key
///           managed: true
///       validationSharedKeys:
///         - secretVersion: ${["secret-version-basic"].id}
///   instance:
///     type: gcp:networkservices:EdgeCacheOrigin
///     properties:
///       name: my-origin
///       originAddress: gs://media-edge-default
///       description: The default bucket for media edge test
///   instanceEdgeCacheService:
///     type: gcp:networkservices:EdgeCacheService
///     name: instance
///     properties:
///       name: my-service
///       description: some description
///       routing:
///         hostRules:
///           - description: host rule description
///             hosts:
///               - sslcert.tf-test.club
///             pathMatcher: routes
///         pathMatchers:
///           - name: routes
///             routeRules:
///               - description: a route rule to match against master playlist
///                 priority: 1
///                 matchRules:
///                   - pathTemplateMatch: /master.m3u8
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     signedRequestMode: REQUIRE_TOKENS
///                     signedRequestKeyset: ${keyset.id}
///                     signedTokenOptions:
///                       tokenQueryParameter: edge-cache-token
///                     signedRequestMaximumExpirationTtl: 600s
///                     addSignatures:
///                       actions: GENERATE_COOKIE
///                       keyset: ${keyset.id}
///                       copiedParameters:
///                         - PathGlobs
///                         - SessionID
///               - description: a route rule to match against all playlists
///                 priority: 2
///                 matchRules:
///                   - pathTemplateMatch: /*.m3u8
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     signedRequestMode: REQUIRE_TOKENS
///                     signedRequestKeyset: ${keyset.id}
///                     signedTokenOptions:
///                       tokenQueryParameter: hdnts
///                       allowedSignatureAlgorithms:
///                         - ED25519
///                         - HMAC_SHA_256
///                         - HMAC_SHA1
///                     addSignatures:
///                       actions: GENERATE_TOKEN_HLS_COOKIELESS
///                       keyset: ${keyset.id}
///                       tokenTtl: 1200s
///                       tokenQueryParameter: hdntl
///                       copiedParameters:
///                         - URLPrefix
///               - description: a route rule to match against
///                 priority: 3
///                 matchRules:
///                   - pathTemplateMatch: /**.m3u8
///                 origin: ${instance.name}
///                 routeAction:
///                   cdnPolicy:
///                     signedRequestMode: REQUIRE_TOKENS
///                     signedRequestKeyset: ${keyset.id}
///                     signedTokenOptions:
///                       tokenQueryParameter: hdntl
///                     addSignatures:
///                       actions: PROPAGATE_TOKEN_HLS_COOKIELESS
///                       tokenQueryParameter: hdntl
/// ```
///
/// ## Import
///
/// EdgeCacheService can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/global/edgeCacheServices/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, EdgeCacheService can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheService:EdgeCacheService default projects/{{project}}/locations/global/edgeCacheServices/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheService:EdgeCacheService default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networkservices/edgeCacheService:EdgeCacheService default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod edge_cache_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EdgeCacheServiceArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Disables HTTP/2. HTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use
        /// and reduces connection setup overhead by sending multiple streams over the same connection. Some legacy HTTP clients may
        /// have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from
        /// being advertised and negotiated.
        #[builder(into, default)]
        pub disable_http2: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// HTTP/3 (IETF QUIC) and Google QUIC are enabled by default.
        #[builder(into, default)]
        pub disable_quic: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Resource URL that points at the Cloud Armor edge security policy that is applied on each request against the
        /// EdgeCacheService.
        #[builder(into, default)]
        pub edge_security_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService. Note
        /// that only "global" certificates with a "scope" of "EDGE_CACHE" can be attached to an EdgeCacheService.
        #[builder(into, default)]
        pub edge_ssl_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Set of label tags associated with the EdgeCache resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the logging options for the traffic served by this service. If logging is enabled, logs will be exported to
        /// Cloud Logging.
        #[builder(into, default)]
        pub log_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networkservices::EdgeCacheServiceLogConfig>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Require TLS (HTTPS) for all clients connecting to this service. Clients who connect over HTTP (port 80) will receive a
        /// HTTP 301 to the same URL over HTTPS (port 443). You must have at least one (1) edgeSslCertificate specified to enable
        /// this.
        #[builder(into, default)]
        pub require_tls: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Defines how requests are routed, modified, cached and/or which origin content is filled from.
        /// Structure is documented below.
        #[builder(into)]
        pub routing: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkservices::EdgeCacheServiceRouting,
        >,
        /// URL of the SslPolicy resource that will be associated with the EdgeCacheService. If not set, the EdgeCacheService has no
        /// SSL policy configured, and will default to the "COMPATIBLE" policy.
        #[builder(into, default)]
        pub ssl_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct EdgeCacheServiceResult {
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Disables HTTP/2. HTTP/2 (h2) is enabled by default and recommended for performance. HTTP/2 improves connection re-use
        /// and reduces connection setup overhead by sending multiple streams over the same connection. Some legacy HTTP clients may
        /// have issues with HTTP/2 connections due to broken HTTP/2 implementations. Setting this to true will prevent HTTP/2 from
        /// being advertised and negotiated.
        pub disable_http2: pulumi_gestalt_rust::Output<Option<bool>>,
        /// HTTP/3 (IETF QUIC) and Google QUIC are enabled by default.
        pub disable_quic: pulumi_gestalt_rust::Output<bool>,
        /// Resource URL that points at the Cloud Armor edge security policy that is applied on each request against the
        /// EdgeCacheService.
        pub edge_security_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// URLs to sslCertificate resources that are used to authenticate connections between users and the EdgeCacheService. Note
        /// that only "global" certificates with a "scope" of "EDGE_CACHE" can be attached to an EdgeCacheService.
        pub edge_ssl_certificates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IPv4 addresses associated with this service. Addresses are static for the lifetime of the service.
        pub ipv4_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IPv6 addresses associated with this service. Addresses are static for the lifetime of the service.
        pub ipv6_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Set of label tags associated with the EdgeCache resource. **Note**: This field is non-authoritative, and will only
        /// manage the labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels
        /// present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the logging options for the traffic served by this service. If logging is enabled, logs will be exported to
        /// Cloud Logging.
        pub log_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::networkservices::EdgeCacheServiceLogConfig>,
        >,
        /// Name of the resource; provided by the client when the resource is created.
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Require TLS (HTTPS) for all clients connecting to this service. Clients who connect over HTTP (port 80) will receive a
        /// HTTP 301 to the same URL over HTTPS (port 443). You must have at least one (1) edgeSslCertificate specified to enable
        /// this.
        pub require_tls: pulumi_gestalt_rust::Output<bool>,
        /// Defines how requests are routed, modified, cached and/or which origin content is filled from.
        /// Structure is documented below.
        pub routing: pulumi_gestalt_rust::Output<
            super::super::types::networkservices::EdgeCacheServiceRouting,
        >,
        /// URL of the SslPolicy resource that will be associated with the EdgeCacheService. If not set, the EdgeCacheService has no
        /// SSL policy configured, and will default to the "COMPATIBLE" policy.
        pub ssl_policy: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EdgeCacheServiceArgs,
    ) -> EdgeCacheServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let disable_http2_binding = args.disable_http2.get_output(context).get_inner();
        let disable_quic_binding = args.disable_quic.get_output(context).get_inner();
        let edge_security_policy_binding = args
            .edge_security_policy
            .get_output(context)
            .get_inner();
        let edge_ssl_certificates_binding = args
            .edge_ssl_certificates
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let log_config_binding = args.log_config.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let require_tls_binding = args.require_tls.get_output(context).get_inner();
        let routing_binding = args.routing.get_output(context).get_inner();
        let ssl_policy_binding = args.ssl_policy.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networkservices/edgeCacheService:EdgeCacheService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "disableHttp2".into(),
                    value: &disable_http2_binding,
                },
                register_interface::ObjectField {
                    name: "disableQuic".into(),
                    value: &disable_quic_binding,
                },
                register_interface::ObjectField {
                    name: "edgeSecurityPolicy".into(),
                    value: &edge_security_policy_binding,
                },
                register_interface::ObjectField {
                    name: "edgeSslCertificates".into(),
                    value: &edge_ssl_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "logConfig".into(),
                    value: &log_config_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "requireTls".into(),
                    value: &require_tls_binding,
                },
                register_interface::ObjectField {
                    name: "routing".into(),
                    value: &routing_binding,
                },
                register_interface::ObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EdgeCacheServiceResult {
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            disable_http2: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableHttp2"),
            ),
            disable_quic: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disableQuic"),
            ),
            edge_security_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSecurityPolicy"),
            ),
            edge_ssl_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeSslCertificates"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            ipv4_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv4Addresses"),
            ),
            ipv6_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipv6Addresses"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            log_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("logConfig"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            require_tls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requireTls"),
            ),
            routing: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routing"),
            ),
            ssl_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslPolicy"),
            ),
        }
    }
}
