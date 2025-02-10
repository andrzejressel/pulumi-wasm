/// Represents a TargetHttpsProxy resource, which is used by one or more
/// global forwarding rule to route incoming HTTPS requests to a URL map.
///
///
/// To get more information about TargetHttpsProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/v1/targetHttpsProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/target-proxies)
///
/// ## Example Usage
///
/// ### Target Https Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetHttpsProxy
///     properties:
///       name: test-proxy
///       urlMap: ${defaultURLMap.id}
///       sslCertificates:
///         - ${defaultSSLCertificate.id}
///   defaultSSLCertificate:
///     type: gcp:compute:SSLCertificate
///     name: default
///     properties:
///       name: my-certificate
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private.key
///           return: result
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/certificate.crt
///           return: result
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: http-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// ```
/// ### Target Https Proxy Http Keep Alive Timeout
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetHttpsProxy
///     properties:
///       name: test-http-keep-alive-timeout-proxy
///       httpKeepAliveTimeoutSec: 610
///       urlMap: ${defaultURLMap.id}
///       sslCertificates:
///         - ${defaultSSLCertificate.id}
///   defaultSSLCertificate:
///     type: gcp:compute:SSLCertificate
///     name: default
///     properties:
///       name: my-certificate
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private.key
///           return: result
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/certificate.crt
///           return: result
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: EXTERNAL_MANAGED
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: http-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// ```
/// ### Target Https Proxy Mtls
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetHttpsProxy
///     properties:
///       name: test-mtls-proxy
///       urlMap: ${defaultURLMap.id}
///       sslCertificates:
///         - ${defaultSSLCertificate.id}
///       serverTlsPolicy: ${defaultServerTlsPolicy.id}
///   defaultTrustConfig:
///     type: gcp:certificatemanager:TrustConfig
///     name: default
///     properties:
///       name: my-trust-config
///       description: sample description for the trust config
///       location: global
///       trustStores:
///         - trustAnchors:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/ca_cert.pem
///                   return: result
///           intermediateCas:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/ca_cert.pem
///                   return: result
///       labels:
///         foo: bar
///   defaultServerTlsPolicy:
///     type: gcp:networksecurity:ServerTlsPolicy
///     name: default
///     properties:
///       name: my-tls-policy
///       description: my description
///       location: global
///       allowOpen: 'false'
///       mtlsPolicy:
///         clientValidationMode: ALLOW_INVALID_OR_MISSING_CLIENT_CERT
///         clientValidationTrustConfig: projects/${project.number}/locations/global/trustConfigs/${defaultTrustConfig.name}
///   defaultSSLCertificate:
///     type: gcp:compute:SSLCertificate
///     name: default
///     properties:
///       name: my-certificate
///       privateKey:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/private.key
///           return: result
///       certificate:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: path/to/certificate.crt
///           return: result
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       healthChecks: ${defaultHttpHealthCheck.id}
///   defaultHttpHealthCheck:
///     type: gcp:compute:HttpHealthCheck
///     name: default
///     properties:
///       name: http-health-check
///       requestPath: /
///       checkIntervalSec: 1
///       timeoutSec: 1
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Target Https Proxy Certificate Manager Certificate
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:TargetHttpsProxy
///     properties:
///       name: target-http-proxy
///       urlMap: ${defaultURLMap.id}
///       certificateManagerCertificates: # [google_certificate_manager_certificate.default.id] is also acceptable
///         - //certificatemanager.googleapis.com/${defaultCertificate.id}
///   defaultCertificate:
///     type: gcp:certificatemanager:Certificate
///     name: default
///     properties:
///       name: my-certificate
///       scope: ALL_REGIONS
///       selfManaged:
///         pemCertificate:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/cert.pem
///             return: result
///         pemPrivateKey:
///           fn::invoke:
///             function: std:file
///             arguments:
///               input: test-fixtures/private-key.pem
///             return: result
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultBackendService.id}
///   defaultBackendService:
///     type: gcp:compute:BackendService
///     name: default
///     properties:
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: INTERNAL_MANAGED
/// ```
///
/// ## Import
///
/// TargetHttpsProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/targetHttpsProxies/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, TargetHttpsProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpsProxy:TargetHttpsProxy default projects/{{project}}/global/targetHttpsProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpsProxy:TargetHttpsProxy default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/targetHttpsProxy:TargetHttpsProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_https_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetHttpsProxyArgs {
        /// URLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.
        /// Certificate manager certificates only apply when the load balancing scheme is set to INTERNAL_MANAGED.
        /// For EXTERNAL and EXTERNAL_MANAGED, use certificate_map instead.
        /// sslCertificates and certificateManagerCertificates fields can not be defined together.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}` or just the self_link `projects/{project}/locations/{location}/certificates/{resourceName}`
        #[builder(into, default)]
        pub certificate_manager_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field is only supported for EXTERNAL and EXTERNAL_MANAGED load balancing schemes.
        /// For INTERNAL_MANAGED, use certificate_manager_certificates instead.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        #[builder(into, default)]
        pub certificate_map: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
        #[builder(into, default)]
        pub http_keep_alive_timeout_sec: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub proxy_bind: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the QUIC override policy for this resource. This determines
        /// whether the load balancer will attempt to negotiate QUIC with clients
        /// or not. Can specify one of NONE, ENABLE, or DISABLE. If NONE is
        /// specified, Google manages whether QUIC is used.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `ENABLE`, `DISABLE`.
        #[builder(into, default)]
        pub quic_override: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A URL referring to a networksecurity.ServerTlsPolicy
        /// resource that describes how the proxy should authenticate inbound
        /// traffic. serverTlsPolicy only applies to a global TargetHttpsProxy
        /// attached to globalForwardingRules with the loadBalancingScheme
        /// set to INTERNAL_SELF_MANAGED or EXTERNAL or EXTERNAL_MANAGED.
        /// For details which ServerTlsPolicy resources are accepted with
        /// INTERNAL_SELF_MANAGED and which with EXTERNAL, EXTERNAL_MANAGED
        /// loadBalancingScheme consult ServerTlsPolicy documentation.
        /// If left blank, communications are not encrypted.
        /// If you remove this field from your configuration at the same time as
        /// deleting or recreating a referenced ServerTlsPolicy resource, you will
        /// receive a resourceInUseByAnotherResource error. Use lifecycle.create_before_destroy
        /// within the ServerTlsPolicy resource to avoid this.
        #[builder(into, default)]
        pub server_tls_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URLs to SslCertificate resources that are used to authenticate connections between users and the load balancer.
        /// Currently, you may specify up to 15 SSL certificates. sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.
        /// sslCertificates and certificateManagerCertificates can not be defined together.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetHttpsProxy resource. If not set, the TargetHttpsProxy
        /// resource will not have any SSL policy configured.
        #[builder(into, default)]
        pub ssl_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether TLS 1.3 0-RTT Data (“Early Data”) should be accepted for this service.
        /// Early Data allows a TLS resumption handshake to include the initial application payload
        /// (a HTTP request) alongside the handshake, reducing the effective round trips to “zero”.
        /// This applies to TLS 1.3 connections over TCP (HTTP/2) as well as over UDP (QUIC/h3).
        /// Possible values are: `STRICT`, `PERMISSIVE`, `DISABLED`.
        #[builder(into, default)]
        pub tls_early_data: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the UrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub url_map: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetHttpsProxyResult {
        /// URLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.
        /// Certificate manager certificates only apply when the load balancing scheme is set to INTERNAL_MANAGED.
        /// For EXTERNAL and EXTERNAL_MANAGED, use certificate_map instead.
        /// sslCertificates and certificateManagerCertificates fields can not be defined together.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}` or just the self_link `projects/{project}/locations/{location}/certificates/{resourceName}`
        pub certificate_manager_certificates: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// A reference to the CertificateMap resource uri that identifies a certificate map
        /// associated with the given target proxy. This field is only supported for EXTERNAL and EXTERNAL_MANAGED load balancing schemes.
        /// For INTERNAL_MANAGED, use certificate_manager_certificates instead.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificateMaps/{resourceName}`.
        pub certificate_map: pulumi_gestalt_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value will be used. For Global
        /// external HTTP(S) load balancer, the default value is 610 seconds, the
        /// minimum allowed value is 5 seconds and the maximum allowed value is 1200
        /// seconds. For cross-region internal HTTP(S) load balancer, the default
        /// value is 600 seconds, the minimum allowed value is 5 seconds, and the
        /// maximum allowed value is 600 seconds. For Global external HTTP(S) load
        /// balancer (classic), this option is not available publicly.
        pub http_keep_alive_timeout_sec: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// This field only applies when the forwarding rule that references
        /// this target proxy has a loadBalancingScheme set to INTERNAL_SELF_MANAGED.
        pub proxy_bind: pulumi_gestalt_rust::Output<bool>,
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// Specifies the QUIC override policy for this resource. This determines
        /// whether the load balancer will attempt to negotiate QUIC with clients
        /// or not. Can specify one of NONE, ENABLE, or DISABLE. If NONE is
        /// specified, Google manages whether QUIC is used.
        /// Default value is `NONE`.
        /// Possible values are: `NONE`, `ENABLE`, `DISABLE`.
        pub quic_override: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// A URL referring to a networksecurity.ServerTlsPolicy
        /// resource that describes how the proxy should authenticate inbound
        /// traffic. serverTlsPolicy only applies to a global TargetHttpsProxy
        /// attached to globalForwardingRules with the loadBalancingScheme
        /// set to INTERNAL_SELF_MANAGED or EXTERNAL or EXTERNAL_MANAGED.
        /// For details which ServerTlsPolicy resources are accepted with
        /// INTERNAL_SELF_MANAGED and which with EXTERNAL, EXTERNAL_MANAGED
        /// loadBalancingScheme consult ServerTlsPolicy documentation.
        /// If left blank, communications are not encrypted.
        /// If you remove this field from your configuration at the same time as
        /// deleting or recreating a referenced ServerTlsPolicy resource, you will
        /// receive a resourceInUseByAnotherResource error. Use lifecycle.create_before_destroy
        /// within the ServerTlsPolicy resource to avoid this.
        pub server_tls_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// URLs to SslCertificate resources that are used to authenticate connections between users and the load balancer.
        /// Currently, you may specify up to 15 SSL certificates. sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.
        /// sslCertificates and certificateManagerCertificates can not be defined together.
        pub ssl_certificates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A reference to the SslPolicy resource that will be associated with
        /// the TargetHttpsProxy resource. If not set, the TargetHttpsProxy
        /// resource will not have any SSL policy configured.
        pub ssl_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether TLS 1.3 0-RTT Data (“Early Data”) should be accepted for this service.
        /// Early Data allows a TLS resumption handshake to include the initial application payload
        /// (a HTTP request) alongside the handshake, reducing the effective round trips to “zero”.
        /// This applies to TLS 1.3 connections over TCP (HTTP/2) as well as over UDP (QUIC/h3).
        /// Possible values are: `STRICT`, `PERMISSIVE`, `DISABLED`.
        pub tls_early_data: pulumi_gestalt_rust::Output<String>,
        /// A reference to the UrlMap resource that defines the mapping from URL
        /// to the BackendService.
        ///
        ///
        /// - - -
        pub url_map: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetHttpsProxyArgs,
    ) -> TargetHttpsProxyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_manager_certificates_binding = args
            .certificate_manager_certificates
            .get_output(context);
        let certificate_map_binding = args.certificate_map.get_output(context);
        let description_binding = args.description.get_output(context);
        let http_keep_alive_timeout_sec_binding = args
            .http_keep_alive_timeout_sec
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let proxy_bind_binding = args.proxy_bind.get_output(context);
        let quic_override_binding = args.quic_override.get_output(context);
        let server_tls_policy_binding = args.server_tls_policy.get_output(context);
        let ssl_certificates_binding = args.ssl_certificates.get_output(context);
        let ssl_policy_binding = args.ssl_policy.get_output(context);
        let tls_early_data_binding = args.tls_early_data.get_output(context);
        let url_map_binding = args.url_map.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/targetHttpsProxy:TargetHttpsProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateManagerCertificates".into(),
                    value: certificate_manager_certificates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateMap".into(),
                    value: certificate_map_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpKeepAliveTimeoutSec".into(),
                    value: http_keep_alive_timeout_sec_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "proxyBind".into(),
                    value: proxy_bind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "quicOverride".into(),
                    value: quic_override_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverTlsPolicy".into(),
                    value: server_tls_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslCertificates".into(),
                    value: ssl_certificates_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sslPolicy".into(),
                    value: ssl_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsEarlyData".into(),
                    value: tls_early_data_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "urlMap".into(),
                    value: url_map_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetHttpsProxyResult {
            certificate_manager_certificates: o
                .get_field("certificateManagerCertificates"),
            certificate_map: o.get_field("certificateMap"),
            creation_timestamp: o.get_field("creationTimestamp"),
            description: o.get_field("description"),
            http_keep_alive_timeout_sec: o.get_field("httpKeepAliveTimeoutSec"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            proxy_bind: o.get_field("proxyBind"),
            proxy_id: o.get_field("proxyId"),
            quic_override: o.get_field("quicOverride"),
            self_link: o.get_field("selfLink"),
            server_tls_policy: o.get_field("serverTlsPolicy"),
            ssl_certificates: o.get_field("sslCertificates"),
            ssl_policy: o.get_field("sslPolicy"),
            tls_early_data: o.get_field("tlsEarlyData"),
            url_map: o.get_field("urlMap"),
        }
    }
}
