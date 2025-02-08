/// An SslCertificate resource, used for HTTPS load balancing.  This resource
/// represents a certificate for which the certificate secrets are created and
/// managed by Google.
///
/// For a resource where you provide the key, see the
/// SSL Certificate resource.
///
///
/// To get more information about ManagedSslCertificate, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/sslCertificates)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/load-balancing/docs/ssl-certificates)
///
/// > **Warning:** This resource should be used with extreme caution!  Provisioning an SSL
/// certificate is complex.  Ensure that you understand the lifecycle of a
/// certificate before attempting complex tasks like cert rotation automatically.
/// This resource will "return" as soon as the certificate object is created,
/// but post-creation the certificate object will go through a "provisioning"
/// process.  The provisioning process can complete only when the domain name
/// for which the certificate is created points to a target pool which, itself,
/// points at the certificate.  Depending on your DNS provider, this may take
/// some time, and migrating from self-managed certificates to Google-managed
/// certificates may entail some downtime while the certificate provisions.
///
/// In conclusion: Be extremely cautious.
///
/// ## Example Usage
///
/// ### Managed Ssl Certificate Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:ManagedSslCertificate
///     properties:
///       name: test-cert
///       managed:
///         domains:
///           - sslcert.tf-test.club.
///   defaultTargetHttpsProxy:
///     type: gcp:compute:TargetHttpsProxy
///     name: default
///     properties:
///       name: test-proxy
///       urlMap: ${defaultURLMap.id}
///       sslCertificates:
///         - ${default.id}
///   defaultURLMap:
///     type: gcp:compute:URLMap
///     name: default
///     properties:
///       name: url-map
///       description: a description
///       defaultService: ${defaultBackendService.id}
///       hostRules:
///         - hosts:
///             - sslcert.tf-test.club
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
///   defaultGlobalForwardingRule:
///     type: gcp:compute:GlobalForwardingRule
///     name: default
///     properties:
///       name: forwarding-rule
///       target: ${defaultTargetHttpsProxy.id}
///       portRange: 443
/// ```
/// ## Import
///
/// ManagedSslCertificate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/global/sslCertificates/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ManagedSslCertificate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/mangedSslCertificate:MangedSslCertificate default projects/{{project}}/global/sslCertificates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/mangedSslCertificate:MangedSslCertificate default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/mangedSslCertificate:MangedSslCertificate default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod manged_ssl_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MangedSslCertificateArgs {
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Properties relevant to a managed certificate.  These will be used if the
        /// certificate is managed (as indicated by a value of `MANAGED` in `type`).
        /// Structure is documented below.
        #[builder(into, default)]
        pub managed: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::MangedSslCertificateManaged>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enum field whose value is always `MANAGED` - used to signal to the API
        /// which type this is.
        /// Default value is `MANAGED`.
        /// Possible values are: `MANAGED`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MangedSslCertificateResult {
        /// The unique identifier for the resource.
        pub certificate_id: pulumi_gestalt_rust::Output<i32>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_gestalt_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Expire time of the certificate in RFC3339 text format.
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// Properties relevant to a managed certificate.  These will be used if the
        /// certificate is managed (as indicated by a value of `MANAGED` in `type`).
        /// Structure is documented below.
        pub managed: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::MangedSslCertificateManaged>,
        >,
        /// Name of the resource. Provided by the client when the resource is
        /// created. The name must be 1-63 characters long, and comply with
        /// RFC1035. Specifically, the name must be 1-63 characters long and match
        /// the regular expression `a-z?` which means the
        /// first character must be a lowercase letter, and all following
        /// characters must be a dash, lowercase letter, or digit, except the last
        /// character, which cannot be a dash.
        /// These are in the same namespace as the managed SSL certificates.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Domains associated with the certificate via Subject Alternative Name.
        pub subject_alternative_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Enum field whose value is always `MANAGED` - used to signal to the API
        /// which type this is.
        /// Default value is `MANAGED`.
        /// Possible values are: `MANAGED`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: MangedSslCertificateArgs,
    ) -> MangedSslCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let managed_binding = args.managed.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/mangedSslCertificate:MangedSslCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "managed".into(),
                    value: &managed_binding,
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
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        MangedSslCertificateResult {
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            creation_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTimestamp"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            managed: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managed"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            subject_alternative_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subjectAlternativeNames"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
