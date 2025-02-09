/// A Certificate corresponds to a signed X.509 certificate issued by a Certificate.
///
///
/// > **Note:** The Certificate Authority that is referenced by this resource **must** be
/// `tier = "ENTERPRISE"`
///
///
///
/// ## Example Usage
///
/// ### Privateca Certificate Generated Key
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       location: us-central1
///       name: default
///       tier: ENTERPRISE
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthorityId: my-authority
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
///   certKey:
///     type: tls:PrivateKey
///     name: cert_key
///     properties:
///       algorithm: RSA
///   defaultCertificate:
///     type: gcp:certificateauthority:Certificate
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthority: ${defaultAuthority.certificateAuthorityId}
///       lifetime: 86000s
///       name: cert-1
///       config:
///         subjectConfig:
///           subject:
///             commonName: san1.example.com
///             countryCode: us
///             organization: google
///             organizationalUnit: enterprise
///             locality: mountain view
///             province: california
///             streetAddress: 1600 amphitheatre parkway
///           subjectAltName:
///             emailAddresses:
///               - email@example.com
///             ipAddresses:
///               - 127.0.0.1
///             uris:
///               - http://www.ietf.org/rfc/rfc3986.txt
///         x509Config:
///           caOptions:
///             isCa: true
///           keyUsage:
///             baseKeyUsage:
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: false
///           nameConstraints:
///             critical: true
///             permittedDnsNames:
///               - '*.example.com'
///             excludedDnsNames:
///               - '*.deny.example.com'
///             permittedIpRanges:
///               - 10.0.0.0/8
///             excludedIpRanges:
///               - 10.1.1.0/24
///             permittedEmailAddresses:
///               - .example.com
///             excludedEmailAddresses:
///               - .deny.example.com
///             permittedUris:
///               - .example.com
///             excludedUris:
///               - .deny.example.com
///         publicKey:
///           format: PEM
///           key:
///             fn::invoke:
///               function: std:base64encode
///               arguments:
///                 input: ${certKey.publicKeyPem}
///               return: result
/// ```
/// ### Privateca Certificate With Template
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       location: us-central1
///       name: my-pool
///       tier: ENTERPRISE
///   defaultCertificateTemplate:
///     type: gcp:certificateauthority:CertificateTemplate
///     name: default
///     properties:
///       location: us-central1
///       name: my-certificate-template
///       description: An updated sample certificate template
///       identityConstraints:
///         allowSubjectAltNamesPassthrough: true
///         allowSubjectPassthrough: true
///         celExpression:
///           description: Always true
///           expression: 'true'
///           location: any.file.anywhere
///           title: Sample expression
///       passthroughExtensions:
///         additionalExtensions:
///           - objectIdPaths:
///               - 1
///               - 6
///         knownExtensions:
///           - EXTENDED_KEY_USAGE
///       predefinedValues:
///         additionalExtensions:
///           - objectId:
///               objectIdPaths:
///                 - 1
///                 - 6
///             value: c3RyaW5nCg==
///             critical: true
///         aiaOcspServers:
///           - string
///         caOptions:
///           isCa: false
///           maxIssuerPathLength: 6
///         keyUsage:
///           baseKeyUsage:
///             certSign: false
///             contentCommitment: true
///             crlSign: false
///             dataEncipherment: true
///             decipherOnly: true
///             digitalSignature: true
///             encipherOnly: true
///             keyAgreement: true
///             keyEncipherment: true
///           extendedKeyUsage:
///             clientAuth: true
///             codeSigning: true
///             emailProtection: true
///             ocspSigning: true
///             serverAuth: true
///             timeStamping: true
///           unknownExtendedKeyUsages:
///             - objectIdPaths:
///                 - 1
///                 - 6
///         policyIds:
///           - objectIdPaths:
///               - 1
///               - 6
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthorityId: my-authority
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
///               serverAuth: false
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///   defaultCertificate:
///     type: gcp:certificateauthority:Certificate
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthority: ${defaultAuthority.certificateAuthorityId}
///       name: my-certificate
///       lifetime: 860s
///       pemCsr:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: test-fixtures/rsa_csr.pem
///           return: result
///       certificateTemplate: ${defaultCertificateTemplate.id}
/// ```
/// ### Privateca Certificate Csr
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       location: us-central1
///       name: my-pool
///       tier: ENTERPRISE
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthorityId: my-authority
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
///               serverAuth: false
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///   defaultCertificate:
///     type: gcp:certificateauthority:Certificate
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthority: ${defaultAuthority.certificateAuthorityId}
///       name: my-certificate
///       lifetime: 860s
///       pemCsr:
///         fn::invoke:
///           function: std:file
///           arguments:
///             input: test-fixtures/rsa_csr.pem
///           return: result
/// ```
/// ### Privateca Certificate No Authority
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       location: us-central1
///       name: my-pool
///       tier: ENTERPRISE
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthorityId: my-authority
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
///               digitalSignature: true
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       lifetime: 86400s
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///   defaultCertificate:
///     type: gcp:certificateauthority:Certificate
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       name: my-certificate
///       lifetime: 860s
///       config:
///         subjectConfig:
///           subject:
///             commonName: san1.example.com
///             countryCode: us
///             organization: google
///             organizationalUnit: enterprise
///             locality: mountain view
///             province: california
///             streetAddress: 1600 amphitheatre parkway
///             postalCode: '94109'
///         x509Config:
///           caOptions:
///             isCa: false
///           keyUsage:
///             baseKeyUsage:
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///         publicKey:
///           format: PEM
///           key:
///             fn::invoke:
///               function: std:filebase64
///               arguments:
///                 input: test-fixtures/rsa_public.pem
///               return: result
///     options:
///       dependsOn:
///         - ${defaultAuthority}
/// ```
/// ### Privateca Certificate Custom Ski
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       location: us-central1
///       name: my-pool
///       tier: ENTERPRISE
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       certificateAuthorityId: my-authority
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
///               digitalSignature: true
///               certSign: true
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///       lifetime: 86400s
///       keySpec:
///         algorithm: RSA_PKCS1_4096_SHA256
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///   defaultCertificate:
///     type: gcp:certificateauthority:Certificate
///     name: default
///     properties:
///       location: us-central1
///       pool: ${default.name}
///       name: my-certificate
///       lifetime: 860s
///       config:
///         subjectConfig:
///           subject:
///             commonName: san1.example.com
///             countryCode: us
///             organization: google
///             organizationalUnit: enterprise
///             locality: mountain view
///             province: california
///             streetAddress: 1600 amphitheatre parkway
///             postalCode: '94109'
///         subjectKeyId:
///           keyId: 4cf3372289b1d411b999dbb9ebcd44744b6b2fca
///         x509Config:
///           caOptions:
///             isCa: false
///           keyUsage:
///             baseKeyUsage:
///               crlSign: true
///             extendedKeyUsage:
///               serverAuth: true
///         publicKey:
///           format: PEM
///           key:
///             fn::invoke:
///               function: std:filebase64
///               arguments:
///                 input: test-fixtures/rsa_public.pem
///               return: result
///     options:
///       dependsOn:
///         - ${defaultAuthority}
/// ```
///
/// ## Import
///
/// Certificate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/caPools/{{pool}}/certificates/{{name}}`
///
/// * `{{project}}/{{location}}/{{pool}}/{{name}}`
///
/// * `{{location}}/{{pool}}/{{name}}`
///
/// When using the `pulumi import` command, Certificate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificate:Certificate default projects/{{project}}/locations/{{location}}/caPools/{{pool}}/certificates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificate:Certificate default {{project}}/{{location}}/{{pool}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificate:Certificate default {{location}}/{{pool}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The Certificate Authority ID that should issue the certificate. For example, to issue a Certificate from
        /// a Certificate Authority with resource name `projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca`,
        /// argument `pool` should be set to `projects/my-project/locations/us-central1/caPools/my-pool`, argument `certificate_authority`
        /// should be set to `my-ca`.
        #[builder(into, default)]
        pub certificate_authority: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name for a CertificateTemplate used to issue this certificate,
        /// in the format `projects/*/locations/*/certificateTemplates/*`. If this is specified,
        /// the caller must have the necessary permission to use this template. If this is
        /// omitted, no template will be used. This template must be in the same location
        /// as the Certificate.
        #[builder(into, default)]
        pub certificate_template: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The config used to create a self-signed X.509 certificate or CSR.
        /// Structure is documented below.
        #[builder(into, default)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::certificateauthority::CertificateConfig>,
        >,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and
        /// "notAfterTime" fields inside an X.509 certificate. A duration in seconds with up to nine
        /// fractional digits, terminated by 's'. Example: "3.5s".
        #[builder(into, default)]
        pub lifetime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Location of the Certificate. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for this Certificate.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Immutable. A pem-encoded X.509 certificate signing request (CSR).
        #[builder(into, default)]
        pub pem_csr: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the CaPool this Certificate belongs to.
        #[builder(into)]
        pub pool: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The Certificate Authority ID that should issue the certificate. For example, to issue a Certificate from
        /// a Certificate Authority with resource name `projects/my-project/locations/us-central1/caPools/my-pool/certificateAuthorities/my-ca`,
        /// argument `pool` should be set to `projects/my-project/locations/us-central1/caPools/my-pool`, argument `certificate_authority`
        /// should be set to `my-ca`.
        pub certificate_authority: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. Details regarding the revocation of this Certificate. This Certificate is considered revoked if and only if this field is present.
        /// Structure is documented below.
        pub certificate_descriptions: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::certificateauthority::CertificateCertificateDescription,
            >,
        >,
        /// The resource name for a CertificateTemplate used to issue this certificate,
        /// in the format `projects/*/locations/*/certificateTemplates/*`. If this is specified,
        /// the caller must have the necessary permission to use this template. If this is
        /// omitted, no template will be used. This template must be in the same location
        /// as the Certificate.
        pub certificate_template: pulumi_gestalt_rust::Output<Option<String>>,
        /// The config used to create a self-signed X.509 certificate or CSR.
        /// Structure is documented below.
        pub config: pulumi_gestalt_rust::Output<
            Option<super::super::types::certificateauthority::CertificateConfig>,
        >,
        /// The time that this resource was created on the server.
        /// This is in RFC3339 text format.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The resource name of the issuing CertificateAuthority in the format `projects/*/locations/*/caPools/*/certificateAuthorities/*`.
        pub issuer_certificate_authority: pulumi_gestalt_rust::Output<String>,
        /// Labels with user-defined metadata to apply to this resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and
        /// "notAfterTime" fields inside an X.509 certificate. A duration in seconds with up to nine
        /// fractional digits, terminated by 's'. Example: "3.5s".
        pub lifetime: pulumi_gestalt_rust::Output<Option<String>>,
        /// Location of the Certificate. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name for this Certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Output only. The pem-encoded, signed X.509 certificate.
        pub pem_certificate: pulumi_gestalt_rust::Output<String>,
        /// The chain that may be used to verify the X.509 certificate. Expected to be in issuer-to-root order according to RFC 5246.
        pub pem_certificate_chains: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Immutable. A pem-encoded X.509 certificate signing request (CSR).
        pub pem_csr: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the CaPool this Certificate belongs to.
        pub pool: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. Details regarding the revocation of this Certificate. This Certificate is
        /// considered revoked if and only if this field is present.
        /// Structure is documented below.
        pub revocation_details: pulumi_gestalt_rust::Output<
            Vec<super::super::types::certificateauthority::CertificateRevocationDetail>,
        >,
        /// Output only. The time at which this CertificateAuthority was updated.
        /// This is in RFC3339 text format.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_authority_binding_1 = args
            .certificate_authority
            .get_output(context);
        let certificate_authority_binding = certificate_authority_binding_1.get_inner();
        let certificate_template_binding_1 = args
            .certificate_template
            .get_output(context);
        let certificate_template_binding = certificate_template_binding_1.get_inner();
        let config_binding_1 = args.config.get_output(context);
        let config_binding = config_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let lifetime_binding_1 = args.lifetime.get_output(context);
        let lifetime_binding = lifetime_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let pem_csr_binding_1 = args.pem_csr.get_output(context);
        let pem_csr_binding = pem_csr_binding_1.get_inner();
        let pool_binding_1 = args.pool.get_output(context);
        let pool_binding = pool_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificateauthority/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateAuthority".into(),
                    value: &certificate_authority_binding,
                },
                register_interface::ObjectField {
                    name: "certificateTemplate".into(),
                    value: &certificate_template_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "lifetime".into(),
                    value: &lifetime_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pemCsr".into(),
                    value: &pem_csr_binding,
                },
                register_interface::ObjectField {
                    name: "pool".into(),
                    value: &pool_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            certificate_authority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateAuthority"),
            ),
            certificate_descriptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateDescriptions"),
            ),
            certificate_template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateTemplate"),
            ),
            config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("config"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            issuer_certificate_authority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("issuerCertificateAuthority"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            lifetime: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lifetime"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            pem_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pemCertificate"),
            ),
            pem_certificate_chains: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pemCertificateChains"),
            ),
            pem_csr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pemCsr"),
            ),
            pool: pulumi_gestalt_rust::__private::into_domain(o.extract_field("pool")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            revocation_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("revocationDetails"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
