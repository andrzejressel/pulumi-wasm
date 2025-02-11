/// Certificate represents a HTTP-reachable backend for a Certificate.
///
///
///
///
///
/// ## Example Usage
///
/// ### Certificate Manager Google Managed Certificate Dns
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:Certificate
///     properties:
///       name: dns-cert
///       description: The default cert
///       scope: EDGE_CACHE
///       labels:
///         env: test
///       managed:
///         domains:
///           - ${instance.domain}
///           - ${instance2.domain}
///         dnsAuthorizations:
///           - ${instance.id}
///           - ${instance2.id}
///   instance:
///     type: gcp:certificatemanager:DnsAuthorization
///     properties:
///       name: dns-auth
///       description: The default dnss
///       domain: subdomain.hashicorptest.com
///   instance2:
///     type: gcp:certificatemanager:DnsAuthorization
///     properties:
///       name: dns-auth2
///       description: The default dnss
///       domain: subdomain2.hashicorptest.com
/// ```
/// ### Certificate Manager Google Managed Certificate Issuance Config
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let caAuthority = authority::create(
///         "caAuthority",
///         AuthorityArgs::builder()
///             .certificate_authority_id("ca-authority")
///             .config(
///                 AuthorityConfig::builder()
///                     .subjectConfig(
///                         AuthorityConfigSubjectConfig::builder()
///                             .subject(
///                                 AuthorityConfigSubjectConfigSubject::builder()
///                                     .commonName("my-certificate-authority")
///                                     .organization("HashiCorp")
///                                     .build_struct(),
///                             )
///                             .subjectAltName(
///                                 AuthorityConfigSubjectConfigSubjectAltName::builder()
///                                     .dnsNames(vec!["hashicorp.com",])
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .x509Config(
///                         AuthorityConfigX509Config::builder()
///                             .caOptions(
///                                 AuthorityConfigX509ConfigCaOptions::builder()
///                                     .isCa(true)
///                                     .build_struct(),
///                             )
///                             .keyUsage(
///                                 AuthorityConfigX509ConfigKeyUsage::builder()
///                                     .baseKeyUsage(
///                                         AuthorityConfigX509ConfigKeyUsageBaseKeyUsage::builder()
///                                             .certSign(true)
///                                             .crlSign(true)
///                                             .build_struct(),
///                                     )
///                                     .extendedKeyUsage(
///                                         AuthorityConfigX509ConfigKeyUsageExtendedKeyUsage::builder()
///                                             .serverAuth(true)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(false)
///             .ignore_active_certificates_on_deletion(true)
///             .key_spec(
///                 AuthorityKeySpec::builder()
///                     .algorithm("RSA_PKCS1_4096_SHA256")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .pool("${pool.name}")
///             .skip_grace_period(true)
///             .build_struct(),
///     );
///     let default = certificate::create(
///         "default",
///         CertificateArgs::builder()
///             .description("The default cert")
///             .managed(
///                 CertificateManaged::builder()
///                     .domains(vec!["terraform.subdomain1.com",])
///                     .issuanceConfig("${issuanceconfig.id}")
///                     .build_struct(),
///             )
///             .name("issuance-config-cert")
///             .scope("EDGE_CACHE")
///             .build_struct(),
///     );
///     let issuanceconfig = certificate_issuance_config::create(
///         "issuanceconfig",
///         CertificateIssuanceConfigArgs::builder()
///             .certificate_authority_config(
///                 CertificateIssuanceConfigCertificateAuthorityConfig::builder()
///                     .certificateAuthorityServiceConfig(
///                         CertificateIssuanceConfigCertificateAuthorityConfigCertificateAuthorityServiceConfig::builder()
///                             .caPool("${pool.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("sample description for the certificate issuanceConfigs")
///             .key_algorithm("ECDSA_P256")
///             .lifetime("1814400s")
///             .name("issuance-config")
///             .rotation_window_percentage(34)
///             .build_struct(),
///     );
///     let pool = ca_pool::create(
///         "pool",
///         CaPoolArgs::builder()
///             .location("us-central1")
///             .name("ca-pool")
///             .tier("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Certificate Manager Certificate Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = certificate::create(
///         "default",
///         CertificateArgs::builder()
///             .description("Global cert")
///             .managed(
///                 CertificateManaged::builder()
///                     .dnsAuthorizations(vec!["${instance.id}", "${instance2.id}",])
///                     .domains(vec!["${instance.domain}", "${instance2.domain}",])
///                     .build_struct(),
///             )
///             .name("self-managed-cert")
///             .scope("EDGE_CACHE")
///             .build_struct(),
///     );
///     let instance = dns_authorization::create(
///         "instance",
///         DnsAuthorizationArgs::builder()
///             .description("The default dnss")
///             .domain("subdomain.hashicorptest.com")
///             .name("dns-auth")
///             .build_struct(),
///     );
///     let instance2 = dns_authorization::create(
///         "instance2",
///         DnsAuthorizationArgs::builder()
///             .description("The default dnss")
///             .domain("subdomain2.hashicorptest.com")
///             .name("dns-auth2")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Certificate Manager Self Managed Certificate Regional
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:Certificate
///     properties:
///       name: self-managed-cert
///       description: Regional cert
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
/// ```
/// ### Certificate Manager Google Managed Certificate Issuance Config All Regions
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let caAuthority = authority::create(
///         "caAuthority",
///         AuthorityArgs::builder()
///             .certificate_authority_id("ca-authority")
///             .config(
///                 AuthorityConfig::builder()
///                     .subjectConfig(
///                         AuthorityConfigSubjectConfig::builder()
///                             .subject(
///                                 AuthorityConfigSubjectConfigSubject::builder()
///                                     .commonName("my-certificate-authority")
///                                     .organization("HashiCorp")
///                                     .build_struct(),
///                             )
///                             .subjectAltName(
///                                 AuthorityConfigSubjectConfigSubjectAltName::builder()
///                                     .dnsNames(vec!["hashicorp.com",])
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .x509Config(
///                         AuthorityConfigX509Config::builder()
///                             .caOptions(
///                                 AuthorityConfigX509ConfigCaOptions::builder()
///                                     .isCa(true)
///                                     .build_struct(),
///                             )
///                             .keyUsage(
///                                 AuthorityConfigX509ConfigKeyUsage::builder()
///                                     .baseKeyUsage(
///                                         AuthorityConfigX509ConfigKeyUsageBaseKeyUsage::builder()
///                                             .certSign(true)
///                                             .crlSign(true)
///                                             .build_struct(),
///                                     )
///                                     .extendedKeyUsage(
///                                         AuthorityConfigX509ConfigKeyUsageExtendedKeyUsage::builder()
///                                             .serverAuth(true)
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(false)
///             .ignore_active_certificates_on_deletion(true)
///             .key_spec(
///                 AuthorityKeySpec::builder()
///                     .algorithm("RSA_PKCS1_4096_SHA256")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .pool("${pool.name}")
///             .skip_grace_period(true)
///             .build_struct(),
///     );
///     let default = certificate::create(
///         "default",
///         CertificateArgs::builder()
///             .description(
///                 "sample google managed all_regions certificate with issuance config for terraform",
///             )
///             .managed(
///                 CertificateManaged::builder()
///                     .domains(vec!["terraform.subdomain1.com",])
///                     .issuanceConfig("${issuanceconfig.id}")
///                     .build_struct(),
///             )
///             .name("issuance-config-cert")
///             .scope("ALL_REGIONS")
///             .build_struct(),
///     );
///     let issuanceconfig = certificate_issuance_config::create(
///         "issuanceconfig",
///         CertificateIssuanceConfigArgs::builder()
///             .certificate_authority_config(
///                 CertificateIssuanceConfigCertificateAuthorityConfig::builder()
///                     .certificateAuthorityServiceConfig(
///                         CertificateIssuanceConfigCertificateAuthorityConfigCertificateAuthorityServiceConfig::builder()
///                             .caPool("${pool.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .description("sample description for the certificate issuanceConfigs")
///             .key_algorithm("ECDSA_P256")
///             .lifetime("1814400s")
///             .name("issuance-config")
///             .rotation_window_percentage(34)
///             .build_struct(),
///     );
///     let pool = ca_pool::create(
///         "pool",
///         CaPoolArgs::builder()
///             .location("us-central1")
///             .name("ca-pool")
///             .tier("ENTERPRISE")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Certificate Manager Google Managed Certificate Dns All Regions
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = certificate::create(
///         "default",
///         CertificateArgs::builder()
///             .description("The default cert")
///             .managed(
///                 CertificateManaged::builder()
///                     .dnsAuthorizations(vec!["${instance.id}", "${instance2.id}",])
///                     .domains(vec!["${instance.domain}", "${instance2.domain}",])
///                     .build_struct(),
///             )
///             .name("dns-cert")
///             .scope("ALL_REGIONS")
///             .build_struct(),
///     );
///     let instance = dns_authorization::create(
///         "instance",
///         DnsAuthorizationArgs::builder()
///             .description("The default dnss")
///             .domain("subdomain.hashicorptest.com")
///             .name("dns-auth")
///             .build_struct(),
///     );
///     let instance2 = dns_authorization::create(
///         "instance2",
///         DnsAuthorizationArgs::builder()
///             .description("The default dnss")
///             .domain("subdomain2.hashicorptest.com")
///             .name("dns-auth2")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Certificate Manager Google Managed Regional Certificate Dns Auth
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = certificate::create(
///         "default",
///         CertificateArgs::builder()
///             .description("regional managed certs")
///             .location("us-central1")
///             .managed(
///                 CertificateManaged::builder()
///                     .dnsAuthorizations(vec!["${instance.id}",])
///                     .domains(vec!["${instance.domain}",])
///                     .build_struct(),
///             )
///             .name("dns-cert")
///             .build_struct(),
///     );
///     let instance = dns_authorization::create(
///         "instance",
///         DnsAuthorizationArgs::builder()
///             .description("The default dnss")
///             .domain("subdomain.hashicorptest.com")
///             .location("us-central1")
///             .name("dns-auth")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Certificate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/certificates/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Certificate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificate:Certificate default projects/{{project}}/locations/{{location}}/certificates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificate:Certificate default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/certificate:Certificate default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the Certificate resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration and state of a Managed Certificate.
        /// Certificate Manager provisions and renews Managed Certificates
        /// automatically, for as long as it's authorized to do so.
        /// Structure is documented below.
        #[builder(into, default)]
        pub managed: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::certificatemanager::CertificateManaged>,
        >,
        /// A user-defined name of the certificate. Certificate names must be unique
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The scope of the certificate.
        /// DEFAULT: Certificates with default scope are served from core Google data centers.
        /// If unsure, choose this option.
        /// EDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.
        /// See https://cloud.google.com/vpc/docs/edge-locations.
        /// ALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).
        /// See https://cloud.google.com/compute/docs/regions-zones
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Certificate data for a SelfManaged Certificate.
        /// SelfManaged Certificates are uploaded by the user. Updating such
        /// certificates before they expire remains the user's responsibility.
        /// Structure is documented below.
        #[builder(into, default)]
        pub self_managed: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::certificatemanager::CertificateSelfManaged>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// A human-readable description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the Certificate resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Configuration and state of a Managed Certificate.
        /// Certificate Manager provisions and renews Managed Certificates
        /// automatically, for as long as it's authorized to do so.
        /// Structure is documented below.
        pub managed: pulumi_gestalt_rust::Output<
            Option<super::super::types::certificatemanager::CertificateManaged>,
        >,
        /// A user-defined name of the certificate. Certificate names must be unique
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
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
        /// The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6)
        pub san_dnsnames: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The scope of the certificate.
        /// DEFAULT: Certificates with default scope are served from core Google data centers.
        /// If unsure, choose this option.
        /// EDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.
        /// See https://cloud.google.com/vpc/docs/edge-locations.
        /// ALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).
        /// See https://cloud.google.com/compute/docs/regions-zones
        pub scope: pulumi_gestalt_rust::Output<Option<String>>,
        /// Certificate data for a SelfManaged Certificate.
        /// SelfManaged Certificates are uploaded by the user. Updating such
        /// certificates before they expire remains the user's responsibility.
        /// Structure is documented below.
        pub self_managed: pulumi_gestalt_rust::Output<
            Option<super::super::types::certificatemanager::CertificateSelfManaged>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let managed_binding = args.managed.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let self_managed_binding = args.self_managed.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificatemanager/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managed".into(),
                    value: &managed_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "selfManaged".into(),
                    value: &self_managed_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateResult {
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            managed: o.get_field("managed"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            san_dnsnames: o.get_field("sanDnsnames"),
            scope: o.get_field("scope"),
            self_managed: o.get_field("selfManaged"),
        }
    }
}
