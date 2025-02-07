/// ServerTlsPolicy is a resource that specifies how a server should authenticate incoming requests. This resource itself does not affect configuration unless it is attached to a target HTTPS proxy or endpoint config selector resource.
///
///
/// To get more information about ServerTlsPolicy, see:
///
/// * [API documentation](https://cloud.google.com/traffic-director/docs/reference/network-security/rest/v1beta1/projects.locations.serverTlsPolicies)
///
/// ## Example Usage
///
/// ### Network Security Server Tls Policy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ServerTlsPolicy
///     properties:
///       name: my-server-tls-policy
///       labels:
///         foo: bar
///       description: my description
///       allowOpen: 'false'
///       serverCertificate:
///         certificateProviderInstance:
///           pluginInstance: google_cloud_private_spiffe
///       mtlsPolicy:
///         clientValidationCas:
///           - grpcEndpoint:
///               targetUri: unix:mypath
/// ```
/// ### Network Security Server Tls Policy Advanced
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ServerTlsPolicy
///     properties:
///       name: my-server-tls-policy
///       labels:
///         foo: bar
///       description: my description
///       location: global
///       allowOpen: 'false'
///       mtlsPolicy:
///         clientValidationMode: ALLOW_INVALID_OR_MISSING_CLIENT_CERT
/// ```
/// ### Network Security Server Tls Policy Server Cert
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ServerTlsPolicy
///     properties:
///       name: my-server-tls-policy
///       labels:
///         foo: bar
///       description: my description
///       location: global
///       allowOpen: 'false'
///       serverCertificate:
///         grpcEndpoint:
///           targetUri: unix:mypath
/// ```
/// ### Network Security Server Tls Policy Mtls
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ServerTlsPolicy
///     properties:
///       name: my-server-tls-policy
///       description: my description
///       location: global
///       allowOpen: 'false'
///       mtlsPolicy:
///         clientValidationMode: REJECT_INVALID
///         clientValidationTrustConfig: projects/${project.number}/locations/global/trustConfigs/${defaultTrustConfig.name}
///       labels:
///         foo: bar
///   defaultTrustConfig:
///     type: gcp:certificatemanager:TrustConfig
///     name: default
///     properties:
///       name: my-trust-config
///       description: sample trust config description
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
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// ServerTlsPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/serverTlsPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ServerTlsPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/serverTlsPolicy:ServerTlsPolicy default projects/{{project}}/locations/{{location}}/serverTlsPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/serverTlsPolicy:ServerTlsPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/serverTlsPolicy:ServerTlsPolicy default {{location}}/{{name}}
/// ```
///
pub mod server_tls_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerTlsPolicyArgs {
        /// This field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.
        /// Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.
        /// Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80.
        #[builder(into, default)]
        pub allow_open: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the ServerTlsPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the server tls policy.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// This field is required if the policy is used with external HTTPS load balancers. This field can be empty for Traffic Director.
        /// Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mtls_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicy>,
        >,
        /// Name of the ServerTlsPolicy resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS.
        /// Structure is documented below.
        #[builder(into, default)]
        pub server_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networksecurity::ServerTlsPolicyServerCertificate,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct ServerTlsPolicyResult {
        /// This field applies only for Traffic Director policies. It is must be set to false for external HTTPS load balancer policies.
        /// Determines if server allows plaintext connections. If set to true, server allows plain text connections. By default, it is set to false. This setting is not exclusive of other encryption modes. For example, if allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections. See documentation of other encryption modes to confirm compatibility.
        /// Consider using it if you wish to upgrade in place your deployment to TLS while having mixed TLS and non-TLS traffic reaching port :80.
        pub allow_open: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Time the ServerTlsPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the ServerTlsPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the server tls policy.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// This field is required if the policy is used with external HTTPS load balancers. This field can be empty for Traffic Director.
        /// Defines a mechanism to provision peer validation certificates for peer to peer authentication (Mutual TLS - mTLS). If not specified, client certificate will not be requested. The connection is treated as TLS and not mTLS. If allowOpen and mtlsPolicy are set, server allows both plain text and mTLS connections.
        /// Structure is documented below.
        pub mtls_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicy>,
        >,
        /// Name of the ServerTlsPolicy resource.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS.
        /// Structure is documented below.
        pub server_certificate: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networksecurity::ServerTlsPolicyServerCertificate,
            >,
        >,
        /// Time the ServerTlsPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServerTlsPolicyArgs,
    ) -> ServerTlsPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allow_open_binding = args.allow_open.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let mtls_policy_binding = args.mtls_policy.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let server_certificate_binding = args
            .server_certificate
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/serverTlsPolicy:ServerTlsPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowOpen".into(),
                    value: &allow_open_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mtlsPolicy".into(),
                    value: &mtls_policy_binding,
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
                    name: "serverCertificate".into(),
                    value: &server_certificate_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServerTlsPolicyResult {
            allow_open: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowOpen"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            mtls_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mtlsPolicy"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            server_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serverCertificate"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
