/// Represents a RegionTargetHttpsProxy resource, which is used by one or more
/// forwarding rules to route incoming HTTPS requests to a URL map.
///
///
/// To get more information about RegionTargetHttpsProxy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/regionTargetHttpsProxies)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/compute/docs/load-balancing/http/target-proxies)
///
/// ## Example Usage
///
/// ### Region Target Https Proxy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionTargetHttpsProxy
///     properties:
///       region: us-central1
///       name: test-proxy
///       urlMap: ${defaultRegionUrlMap.id}
///       sslCertificates:
///         - ${defaultRegionSslCertificate.id}
///   defaultRegionSslCertificate:
///     type: gcp:compute:RegionSslCertificate
///     name: default
///     properties:
///       region: us-central1
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
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       region: us-central1
///       name: url-map
///       description: a description
///       defaultService: ${defaultRegionBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultRegionBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       region: us-central1
///       name: backend-service
///       protocol: HTTP
///       loadBalancingScheme: INTERNAL_MANAGED
///       timeoutSec: 10
///       healthChecks: ${defaultRegionHealthCheck.id}
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: http-health-check
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Region Target Https Proxy Http Keep Alive Timeout
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionTargetHttpsProxy
///     properties:
///       region: us-central1
///       name: test-http-keep-alive-timeout-proxy
///       httpKeepAliveTimeoutSec: 600
///       urlMap: ${defaultRegionUrlMap.id}
///       sslCertificates:
///         - ${defaultRegionSslCertificate.id}
///   defaultRegionSslCertificate:
///     type: gcp:compute:RegionSslCertificate
///     name: default
///     properties:
///       region: us-central1
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
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       region: us-central1
///       name: url-map
///       description: a description
///       defaultService: ${defaultRegionBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultRegionBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       region: us-central1
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: INTERNAL_MANAGED
///       healthChecks: ${defaultRegionHealthCheck.id}
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: http-health-check
///       httpHealthCheck:
///         port: 80
/// ```
/// ### Region Target Https Proxy Mtls
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionTargetHttpsProxy
///     properties:
///       region: us-central1
///       name: test-mtls-proxy
///       urlMap: ${defaultRegionUrlMap.id}
///       sslCertificates:
///         - ${defaultRegionSslCertificate.id}
///       serverTlsPolicy: ${defaultServerTlsPolicy.id}
///   defaultTrustConfig:
///     type: gcp:certificatemanager:TrustConfig
///     name: default
///     properties:
///       location: us-central1
///       name: my-trust-config
///       description: sample description for trust config
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
///       location: us-central1
///       name: my-tls-policy
///       description: my description
///       allowOpen: 'false'
///       mtlsPolicy:
///         clientValidationMode: REJECT_INVALID
///         clientValidationTrustConfig: projects/${project.number}/locations/us-central1/trustConfigs/${defaultTrustConfig.name}
///   defaultRegionSslCertificate:
///     type: gcp:compute:RegionSslCertificate
///     name: default
///     properties:
///       region: us-central1
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
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       region: us-central1
///       name: url-map
///       description: a description
///       defaultService: ${defaultRegionBackendService.id}
///       hostRules:
///         - hosts:
///             - mysite.com
///           pathMatcher: allpaths
///       pathMatchers:
///         - name: allpaths
///           defaultService: ${defaultRegionBackendService.id}
///           pathRules:
///             - paths:
///                 - /*
///               service: ${defaultRegionBackendService.id}
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       region: us-central1
///       name: backend-service
///       portName: http
///       protocol: HTTP
///       timeoutSec: 10
///       loadBalancingScheme: INTERNAL_MANAGED
///       healthChecks: ${defaultRegionHealthCheck.id}
///   defaultRegionHealthCheck:
///     type: gcp:compute:RegionHealthCheck
///     name: default
///     properties:
///       region: us-central1
///       name: http-health-check
///       checkIntervalSec: 1
///       timeoutSec: 1
///       httpHealthCheck:
///         port: 80
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Region Target Https Proxy Certificate Manager Certificate
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:RegionTargetHttpsProxy
///     properties:
///       name: target-http-proxy
///       urlMap: ${defaultRegionUrlMap.id}
///       certificateManagerCertificates: # [google_certificate_manager_certificate.default.id] is also acceptable
///         - //certificatemanager.googleapis.com/${defaultCertificate.id}
///   defaultCertificate:
///     type: gcp:certificatemanager:Certificate
///     name: default
///     properties:
///       name: my-certificate
///       location: us-central1
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
///   defaultRegionUrlMap:
///     type: gcp:compute:RegionUrlMap
///     name: default
///     properties:
///       name: url-map
///       defaultService: ${defaultRegionBackendService.id}
///       region: us-central1
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       name: backend-service
///       region: us-central1
///       protocol: HTTPS
///       timeoutSec: 30
///       loadBalancingScheme: INTERNAL_MANAGED
/// ```
///
/// ## Import
///
/// RegionTargetHttpsProxy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/targetHttpsProxies/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, RegionTargetHttpsProxy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpsProxy:RegionTargetHttpsProxy default projects/{{project}}/regions/{{region}}/targetHttpsProxies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpsProxy:RegionTargetHttpsProxy default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpsProxy:RegionTargetHttpsProxy default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/regionTargetHttpsProxy:RegionTargetHttpsProxy default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod region_target_https_proxy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RegionTargetHttpsProxyArgs {
        /// URLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.
        /// sslCertificates and certificateManagerCertificates can't be defined together.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}` or just the self_link `projects/{project}/locations/{location}/certificates/{resourceName}`
        #[builder(into, default)]
        pub certificate_manager_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value (600 seconds) will be used. For Regioanl
        /// HTTP(S) load balancer, the minimum allowed value is 5 seconds and the
        /// maximum allowed value is 600 seconds.
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
        /// The Region in which the created target https proxy should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        /// At least one SSL certificate must be specified. Currently, you may specify up to 15 SSL certificates.
        /// sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.
        #[builder(into, default)]
        pub ssl_certificates: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A reference to the Region SslPolicy resource that will be associated with
        /// the TargetHttpsProxy resource. If not set, the TargetHttpsProxy
        /// resource will not have any SSL policy configured.
        #[builder(into, default)]
        pub ssl_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A reference to the RegionUrlMap resource that defines the mapping from URL
        /// to the RegionBackendService.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub url_map: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RegionTargetHttpsProxyResult {
        /// URLs to certificate manager certificate resources that are used to authenticate connections between users and the load balancer.
        /// sslCertificates and certificateManagerCertificates can't be defined together.
        /// Accepted format is `//certificatemanager.googleapis.com/projects/{project}/locations/{location}/certificates/{resourceName}` or just the self_link `projects/{project}/locations/{location}/certificates/{resourceName}`
        pub certificate_manager_certificates: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies how long to keep a connection open, after completing a response,
        /// while there is no matching traffic (in seconds). If an HTTP keepalive is
        /// not specified, a default value (600 seconds) will be used. For Regioanl
        /// HTTP(S) load balancer, the minimum allowed value is 5 seconds and the
        /// maximum allowed value is 600 seconds.
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
        /// The unique identifier for the resource.
        pub proxy_id: pulumi_gestalt_rust::Output<i32>,
        /// The Region in which the created target https proxy should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
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
        /// At least one SSL certificate must be specified. Currently, you may specify up to 15 SSL certificates.
        /// sslCertificates do not apply when the load balancing scheme is set to INTERNAL_SELF_MANAGED.
        pub ssl_certificates: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A reference to the Region SslPolicy resource that will be associated with
        /// the TargetHttpsProxy resource. If not set, the TargetHttpsProxy
        /// resource will not have any SSL policy configured.
        pub ssl_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// A reference to the RegionUrlMap resource that defines the mapping from URL
        /// to the RegionBackendService.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RegionTargetHttpsProxyArgs,
    ) -> RegionTargetHttpsProxyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_manager_certificates_binding = args
            .certificate_manager_certificates
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let http_keep_alive_timeout_sec_binding = args
            .http_keep_alive_timeout_sec
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let server_tls_policy_binding = args
            .server_tls_policy
            .get_output(context)
            .get_inner();
        let ssl_certificates_binding = args
            .ssl_certificates
            .get_output(context)
            .get_inner();
        let ssl_policy_binding = args.ssl_policy.get_output(context).get_inner();
        let url_map_binding = args.url_map.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/regionTargetHttpsProxy:RegionTargetHttpsProxy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateManagerCertificates".into(),
                    value: &certificate_manager_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "httpKeepAliveTimeoutSec".into(),
                    value: &http_keep_alive_timeout_sec_binding,
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
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "serverTlsPolicy".into(),
                    value: &server_tls_policy_binding,
                },
                register_interface::ObjectField {
                    name: "sslCertificates".into(),
                    value: &ssl_certificates_binding,
                },
                register_interface::ObjectField {
                    name: "sslPolicy".into(),
                    value: &ssl_policy_binding,
                },
                register_interface::ObjectField {
                    name: "urlMap".into(),
                    value: &url_map_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RegionTargetHttpsProxyResult {
            certificate_manager_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateManagerCertificates"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            http_keep_alive_timeout_sec: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpKeepAliveTimeoutSec"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            proxy_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("proxyId"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            server_tls_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverTlsPolicy"),
            ),
            ssl_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslCertificates"),
            ),
            ssl_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sslPolicy"),
            ),
            url_map: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("urlMap"),
            ),
        }
    }
}
