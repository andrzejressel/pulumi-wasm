/// ClientTlsPolicy is a resource that specifies how a client should authenticate connections to backends of a service. This resource itself does not affect configuration unless it is attached to a backend service resource.
///
///
/// To get more information about ClientTlsPolicy, see:
///
/// * [API documentation](https://cloud.google.com/traffic-director/docs/reference/network-security/rest/v1beta1/projects.locations.clientTlsPolicies)
/// * How-to Guides
///     * [Service Security](https://cloud.google.com/traffic-director/docs/security-use-cases)
///
/// ## Example Usage
///
/// ### Network Security Client Tls Policy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ClientTlsPolicy
///     properties:
///       name: my-client-tls-policy
///       labels:
///         foo: bar
///       description: my description
///       sni: secure.example.com
/// ```
/// ### Network Security Client Tls Policy Advanced
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:networksecurity:ClientTlsPolicy
///     properties:
///       name: my-client-tls-policy
///       labels:
///         foo: bar
///       description: my description
///       clientCertificate:
///         certificateProviderInstance:
///           pluginInstance: google_cloud_private_spiffe
///       serverValidationCas:
///         - grpcEndpoint:
///             targetUri: unix:mypath
/// ```
///
/// ## Import
///
/// ClientTlsPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/clientTlsPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, ClientTlsPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/clientTlsPolicy:ClientTlsPolicy default projects/{{project}}/locations/{{location}}/clientTlsPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/clientTlsPolicy:ClientTlsPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/clientTlsPolicy:ClientTlsPolicy default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod client_tls_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClientTlsPolicyArgs {
        /// Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS.
        /// Structure is documented below.
        #[builder(into, default)]
        pub client_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::networksecurity::ClientTlsPolicyClientCertificate,
            >,
        >,
        /// A free-text description of the resource. Max length 1024 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the ClientTlsPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the client tls policy.
        /// The default value is `global`.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the ClientTlsPolicy resource.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate.
        /// Structure is documented below.
        #[builder(into, default)]
        pub server_validation_cas: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::networksecurity::ClientTlsPolicyServerValidationCa,
                >,
            >,
        >,
        /// Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com".
        #[builder(into, default)]
        pub sni: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClientTlsPolicyResult {
        /// Defines a mechanism to provision client identity (public and private keys) for peer to peer authentication. The presence of this dictates mTLS.
        /// Structure is documented below.
        pub client_certificate: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::networksecurity::ClientTlsPolicyClientCertificate,
            >,
        >,
        /// Time the ClientTlsPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// A free-text description of the resource. Max length 1024 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the ClientTlsPolicy resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location of the client tls policy.
        /// The default value is `global`.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the ClientTlsPolicy resource.
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
        /// Defines the mechanism to obtain the Certificate Authority certificate to validate the server certificate. If empty, client does not validate the server certificate.
        /// Structure is documented below.
        pub server_validation_cas: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::networksecurity::ClientTlsPolicyServerValidationCa,
                >,
            >,
        >,
        /// Server Name Indication string to present to the server during TLS handshake. E.g: "secure.example.com".
        pub sni: pulumi_gestalt_rust::Output<Option<String>>,
        /// Time the ClientTlsPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClientTlsPolicyArgs,
    ) -> ClientTlsPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let client_certificate_binding = args.client_certificate.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let server_validation_cas_binding = args
            .server_validation_cas
            .get_output(context);
        let sni_binding = args.sni.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:networksecurity/clientTlsPolicy:ClientTlsPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificate".into(),
                    value: client_certificate_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
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
                    name: "serverValidationCas".into(),
                    value: server_validation_cas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sni".into(),
                    value: sni_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClientTlsPolicyResult {
            client_certificate: o.get_field("clientCertificate"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            server_validation_cas: o.get_field("serverValidationCas"),
            sni: o.get_field("sni"),
            update_time: o.get_field("updateTime"),
        }
    }
}
