/// Certificate represents a HTTP-reachable backend for a Certificate.
///
///
/// To get more information about CertificateIssuanceConfig, see:
///
/// * [API documentation](https://cloud.google.com/certificate-manager/docs/reference/certificate-manager/rest/v1/projects.locations.certificateIssuanceConfigs)
/// * How-to Guides
///     * [Manage certificate issuance configs](https://cloud.google.com/certificate-manager/docs/issuance-configs)
///
/// ## Example Usage
///
/// ### Certificate Manager Certificate Issuance Config
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:CertificateIssuanceConfig
///     properties:
///       name: issuance-config
///       description: sample description for the certificate issuanceConfigs
///       certificateAuthorityConfig:
///         certificateAuthorityServiceConfig:
///           caPool: ${pool.id}
///       lifetime: 1814400s
///       rotationWindowPercentage: 34
///       keyAlgorithm: ECDSA_P256
///       labels:
///         name: wrench
///         count: '3'
///     options:
///       dependsOn:
///         - ${caAuthority}
///   pool:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: ca-pool
///       location: us-central1
///       tier: ENTERPRISE
///   caAuthority:
///     type: gcp:certificateauthority:Authority
///     name: ca_authority
///     properties:
///       location: us-central1
///       pool: ${pool.name}
///       certificateAuthorityId: ca-authority
///       config:
///         subjectConfig:
///           subject:
///             organization: HashiCorp
///             commonName: my-certificate-authority
///           subjectAltName:
///             dnsNames:
///               - hashicorp.com
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
/// ```
///
/// ## Import
///
/// CertificateIssuanceConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/certificateIssuanceConfigs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, CertificateIssuanceConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateIssuanceConfig:CertificateIssuanceConfig default projects/{{project}}/locations/{{location}}/certificateIssuanceConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateIssuanceConfig:CertificateIssuanceConfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificateIssuanceConfig:CertificateIssuanceConfig default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_issuance_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateIssuanceConfigArgs {
        /// The CA that issues the workload certificate. It includes the CA address, type, authentication to CA service, etc.
        /// Structure is documented below.
        #[builder(into)]
        pub certificate_authority_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::certificatemanager::CertificateIssuanceConfigCertificateAuthorityConfig,
        >,
        /// One or more paragraphs of text description of a CertificateIssuanceConfig.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key algorithm to use when generating the private key.
        /// Possible values are: `RSA_2048`, `ECDSA_P256`.
        #[builder(into)]
        pub key_algorithm: pulumi_gestalt_rust::InputOrOutput<String>,
        /// 'Set of label tags associated with the CertificateIssuanceConfig resource. An object containing a list of "key": value
        /// pairs. Example: { "name": "wrench", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Lifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.
        /// Example: "1814400s". Valid values are from 21 days (1814400s) to 30 days (2592000s)
        #[builder(into)]
        pub lifetime: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Certificate Manager location. If not specified, "global" is used.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A user-defined name of the certificate issuance config.
        /// CertificateIssuanceConfig names must be unique globally.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// It specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.
        /// Must be a number between 1-99, inclusive.
        /// You must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after
        /// the certificate has been issued and at least 7 days before it expires.
        #[builder(into)]
        pub rotation_window_percentage: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct CertificateIssuanceConfigResult {
        /// The CA that issues the workload certificate. It includes the CA address, type, authentication to CA service, etc.
        /// Structure is documented below.
        pub certificate_authority_config: pulumi_gestalt_rust::Output<
            super::super::types::certificatemanager::CertificateIssuanceConfigCertificateAuthorityConfig,
        >,
        /// The creation timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC "Zulu" format,
        /// accurate to nanoseconds with up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// One or more paragraphs of text description of a CertificateIssuanceConfig.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Key algorithm to use when generating the private key.
        /// Possible values are: `RSA_2048`, `ECDSA_P256`.
        pub key_algorithm: pulumi_gestalt_rust::Output<String>,
        /// 'Set of label tags associated with the CertificateIssuanceConfig resource. An object containing a list of "key": value
        /// pairs. Example: { "name": "wrench", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the
        /// labels present in your configuration. Please refer to the field 'effective_labels' for all of the labels present on the
        /// resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Lifetime of issued certificates. A duration in seconds with up to nine fractional digits, ending with 's'.
        /// Example: "1814400s". Valid values are from 21 days (1814400s) to 30 days (2592000s)
        pub lifetime: pulumi_gestalt_rust::Output<String>,
        /// The Certificate Manager location. If not specified, "global" is used.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// A user-defined name of the certificate issuance config.
        /// CertificateIssuanceConfig names must be unique globally.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// It specifies the percentage of elapsed time of the certificate lifetime to wait before renewing the certificate.
        /// Must be a number between 1-99, inclusive.
        /// You must set the rotation window percentage in relation to the certificate lifetime so that certificate renewal occurs at least 7 days after
        /// the certificate has been issued and at least 7 days before it expires.
        pub rotation_window_percentage: pulumi_gestalt_rust::Output<i32>,
        /// The last update timestamp of a CertificateIssuanceConfig. Timestamp is in RFC3339 UTC "Zulu" format,
        /// accurate to nanoseconds with up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateIssuanceConfigArgs,
    ) -> CertificateIssuanceConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_authority_config_binding = args
            .certificate_authority_config
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let key_algorithm_binding = args.key_algorithm.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let lifetime_binding = args.lifetime.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let rotation_window_percentage_binding = args
            .rotation_window_percentage
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificatemanager/certificateIssuanceConfig:CertificateIssuanceConfig"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityConfig".into(),
                    value: certificate_authority_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyAlgorithm".into(),
                    value: key_algorithm_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lifetime".into(),
                    value: lifetime_binding.get_id(),
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
                    name: "rotationWindowPercentage".into(),
                    value: rotation_window_percentage_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateIssuanceConfigResult {
            certificate_authority_config: o.get_field("certificateAuthorityConfig"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            key_algorithm: o.get_field("keyAlgorithm"),
            labels: o.get_field("labels"),
            lifetime: o.get_field("lifetime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            rotation_window_percentage: o.get_field("rotationWindowPercentage"),
            update_time: o.get_field("updateTime"),
        }
    }
}
