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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// A human-readable description of the resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of label tags associated with the Certificate resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration and state of a Managed Certificate.
        /// Certificate Manager provisions and renews Managed Certificates
        /// automatically, for as long as it's authorized to do so.
        /// Structure is documented below.
        #[builder(into, default)]
        pub managed: pulumi_wasm_rust::Output<
            Option<super::super::types::certificatemanager::CertificateManaged>,
        >,
        /// A user-defined name of the certificate. Certificate names must be unique
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The scope of the certificate.
        /// DEFAULT: Certificates with default scope are served from core Google data centers.
        /// If unsure, choose this option.
        /// EDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.
        /// See https://cloud.google.com/vpc/docs/edge-locations.
        /// ALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).
        /// See https://cloud.google.com/compute/docs/regions-zones
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate data for a SelfManaged Certificate.
        /// SelfManaged Certificates are uploaded by the user. Updating such
        /// certificates before they expire remains the user's responsibility.
        /// Structure is documented below.
        #[builder(into, default)]
        pub self_managed: pulumi_wasm_rust::Output<
            Option<super::super::types::certificatemanager::CertificateSelfManaged>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// A human-readable description of the resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the Certificate resource.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The Certificate Manager location. If not specified, "global" is used.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Configuration and state of a Managed Certificate.
        /// Certificate Manager provisions and renews Managed Certificates
        /// automatically, for as long as it's authorized to do so.
        /// Structure is documented below.
        pub managed: pulumi_wasm_rust::Output<
            Option<super::super::types::certificatemanager::CertificateManaged>,
        >,
        /// A user-defined name of the certificate. Certificate names must be unique
        /// The name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,
        /// and all following characters must be a dash, underscore, letter or digit.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The list of Subject Alternative Names of dnsName type defined in the certificate (see RFC 5280 4.2.1.6)
        pub san_dnsnames: pulumi_wasm_rust::Output<Vec<String>>,
        /// The scope of the certificate.
        /// DEFAULT: Certificates with default scope are served from core Google data centers.
        /// If unsure, choose this option.
        /// EDGE_CACHE: Certificates with scope EDGE_CACHE are special-purposed certificates, served from Edge Points of Presence.
        /// See https://cloud.google.com/vpc/docs/edge-locations.
        /// ALL_REGIONS: Certificates with ALL_REGIONS scope are served from all GCP regions (You can only use ALL_REGIONS with global certs).
        /// See https://cloud.google.com/compute/docs/regions-zones
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Certificate data for a SelfManaged Certificate.
        /// SelfManaged Certificates are uploaded by the user. Updating such
        /// certificates before they expire remains the user's responsibility.
        /// Structure is documented below.
        pub self_managed: pulumi_wasm_rust::Output<
            Option<super::super::types::certificatemanager::CertificateSelfManaged>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CertificateArgs) -> CertificateResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let managed_binding = args.managed.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let scope_binding = args.scope.get_inner();
        let self_managed_binding = args.self_managed.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificatemanager/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "selfManaged".into(),
                    value: &self_managed_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "managed".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "sanDnsnames".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "selfManaged".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CertificateResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            managed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managed").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            san_dnsnames: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sanDnsnames").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            self_managed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfManaged").unwrap(),
            ),
        }
    }
}
