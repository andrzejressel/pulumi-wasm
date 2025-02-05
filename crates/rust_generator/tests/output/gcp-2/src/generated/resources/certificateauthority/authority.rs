/// A CertificateAuthority represents an individual Certificate Authority. A
/// CertificateAuthority can be used to create Certificates.
///
///
/// To get more information about CertificateAuthority, see:
///
/// * [API documentation](https://cloud.google.com/certificate-authority-service/docs/reference/rest)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/certificate-authority-service)
///
/// > **Warning:** On newer versions of the provider, you must explicitly set `deletion_protection=false`
/// (and run `pulumi up` to write the field to state) in order to destroy a CertificateAuthority.
/// It is recommended to not set this field (or set it to true) until you're ready to destroy.
///
/// ## Example Usage
///
/// ### Privateca Certificate Authority Basic
///
///
/// ### Privateca Certificate Authority Subordinate
///
///
/// ### Privateca Certificate Authority Byo Key
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = authority::create(
///         "default",
///         AuthorityArgs::builder()
///             .certificate_authority_id("my-certificate-authority")
///             .config(
///                 AuthorityConfig::builder()
///                     .subjectConfig(
///                         AuthorityConfigSubjectConfig::builder()
///                             .subject(
///                                 AuthorityConfigSubjectConfigSubject::builder()
///                                     .commonName("Example Authority")
///                                     .organization("Example, Org.")
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
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .nameConstraints(
///                                 AuthorityConfigX509ConfigNameConstraints::builder()
///                                     .critical(true)
///                                     .excludedDnsNames(vec!["*.deny.example.com",])
///                                     .excludedEmailAddresses(vec![".deny.example.com",])
///                                     .excludedIpRanges(vec!["10.1.1.0/24",])
///                                     .excludedUris(vec![".deny.example.com",])
///                                     .permittedDnsNames(vec!["*.example.com",])
///                                     .permittedEmailAddresses(vec![".example.com",])
///                                     .permittedIpRanges(vec!["10.0.0.0/8",])
///                                     .permittedUris(vec![".example.com",])
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .deletion_protection(true)
///             .key_spec(
///                 AuthorityKeySpec::builder()
///                     .cloudKmsKeyVersion(
///                         "projects/keys-project/locations/us-central1/keyRings/key-ring/cryptoKeys/crypto-key/cryptoKeyVersions/1",
///                     )
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .pool("ca-pool")
///             .build_struct(),
///     );
///     let privatecaSa = service_identity::create(
///         "privatecaSa",
///         ServiceIdentityArgs::builder().service("privateca.googleapis.com").build_struct(),
///     );
///     let privatecaSaKeyuserSignerverifier = crypto_key_iam_member::create(
///         "privatecaSaKeyuserSignerverifier",
///         CryptoKeyIamMemberArgs::builder()
///             .crypto_key_id(
///                 "projects/keys-project/locations/us-central1/keyRings/key-ring/cryptoKeys/crypto-key",
///             )
///             .member("${privatecaSa.member}")
///             .role("roles/cloudkms.signerVerifier")
///             .build_struct(),
///     );
///     let privatecaSaKeyuserViewer = crypto_key_iam_member::create(
///         "privatecaSaKeyuserViewer",
///         CryptoKeyIamMemberArgs::builder()
///             .crypto_key_id(
///                 "projects/keys-project/locations/us-central1/keyRings/key-ring/cryptoKeys/crypto-key",
///             )
///             .member("${privatecaSa.member}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Privateca Certificate Authority Custom Ski
///
///
///
/// ## Import
///
/// CertificateAuthority can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/caPools/{{pool}}/certificateAuthorities/{{certificate_authority_id}}`
///
/// * `{{project}}/{{location}}/{{pool}}/{{certificate_authority_id}}`
///
/// * `{{location}}/{{pool}}/{{certificate_authority_id}}`
///
/// When using the `pulumi import` command, CertificateAuthority can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/authority:Authority default projects/{{project}}/locations/{{location}}/caPools/{{pool}}/certificateAuthorities/{{certificate_authority_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/authority:Authority default {{project}}/{{location}}/{{pool}}/{{certificate_authority_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/authority:Authority default {{location}}/{{pool}}/{{certificate_authority_id}}
/// ```
///
pub mod authority {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthorityArgs {
        /// The user provided Resource ID for this Certificate Authority.
        #[builder(into)]
        pub certificate_authority_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The config used to create a self-signed X.509 certificate or CSR.
        /// Structure is documented below.
        #[builder(into)]
        pub config: pulumi_wasm_rust::InputOrOutput<
            super::super::types::certificateauthority::AuthorityConfig,
        >,
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Desired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA. Possible values:
        /// ENABLED, DISABLED, STAGED.
        #[builder(into, default)]
        pub desired_state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of a Cloud Storage bucket where this CertificateAuthority will publish content, such as the CA certificate and
        /// CRLs. This must be a bucket name, without any prefixes (such as 'gs://') or suffixes (such as '.googleapis.com'). For
        /// example, to use a bucket named my-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will
        /// be created.
        #[builder(into, default)]
        pub gcs_bucket: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// This field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and
        /// unexpired certs. Use with care. Defaults to 'false'.
        #[builder(into, default)]
        pub ignore_active_certificates_on_deletion: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Used when issuing certificates for this CertificateAuthority. If this CertificateAuthority
        /// is a self-signed CertificateAuthority, this key is also used to sign the self-signed CA
        /// certificate. Otherwise, it is used to sign a CSR.
        /// Structure is documented below.
        #[builder(into)]
        pub key_spec: pulumi_wasm_rust::InputOrOutput<
            super::super::types::certificateauthority::AuthorityKeySpec,
        >,
        /// Labels with user-defined metadata. An object containing a list of "key": value pairs. Example: { "name": "wrench",
        /// "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and "notAfterTime" fields inside an X.509
        /// certificate. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        #[builder(into, default)]
        pub lifetime: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Location of the CertificateAuthority. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with
        /// a third party issuer.
        #[builder(into, default)]
        pub pem_ca_certificate: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the CaPool this Certificate Authority belongs to.
        #[builder(into)]
        pub pool: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If this flag is set, the Certificate Authority will be deleted as soon as possible without a 30-day grace period where
        /// undeletion would have been allowed. If you proceed, there will be no way to recover this CA. Use with care. Defaults to
        /// 'false'.
        #[builder(into, default)]
        pub skip_grace_period: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// If this is a subordinate CertificateAuthority, this field will be set with the subordinate configuration, which
        /// describes its issuers.
        #[builder(into, default)]
        pub subordinate_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::certificateauthority::AuthoritySubordinateConfig>,
        >,
        /// The Type of this CertificateAuthority. > **Note:** For 'SUBORDINATE' Certificate Authorities, they need to be activated
        /// before they can issue certificates. Default value: "SELF_SIGNED" Possible values: ["SELF_SIGNED", "SUBORDINATE"]
        #[builder(into, default)]
        pub type_: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AuthorityResult {
        /// URLs for accessing content published by this CA, such as the CA certificate and CRLs.
        /// Structure is documented below.
        pub access_urls: pulumi_wasm_rust::Output<
            Vec<super::super::types::certificateauthority::AuthorityAccessUrl>,
        >,
        /// The user provided Resource ID for this Certificate Authority.
        pub certificate_authority_id: pulumi_wasm_rust::Output<String>,
        /// The config used to create a self-signed X.509 certificate or CSR.
        /// Structure is documented below.
        pub config: pulumi_wasm_rust::Output<
            super::super::types::certificateauthority::AuthorityConfig,
        >,
        /// The time at which this CertificateAuthority was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// Desired state of the CertificateAuthority. Set this field to 'STAGED' to create a 'STAGED' root CA. Possible values:
        /// ENABLED, DISABLED, STAGED.
        pub desired_state: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The name of a Cloud Storage bucket where this CertificateAuthority will publish content, such as the CA certificate and
        /// CRLs. This must be a bucket name, without any prefixes (such as 'gs://') or suffixes (such as '.googleapis.com'). For
        /// example, to use a bucket named my-bucket, you would simply specify 'my-bucket'. If not specified, a managed bucket will
        /// be created.
        pub gcs_bucket: pulumi_wasm_rust::Output<Option<String>>,
        /// This field allows the CA to be deleted even if the CA has active certs. Active certs include both unrevoked and
        /// unexpired certs. Use with care. Defaults to 'false'.
        pub ignore_active_certificates_on_deletion: pulumi_wasm_rust::Output<
            Option<bool>,
        >,
        /// Used when issuing certificates for this CertificateAuthority. If this CertificateAuthority
        /// is a self-signed CertificateAuthority, this key is also used to sign the self-signed CA
        /// certificate. Otherwise, it is used to sign a CSR.
        /// Structure is documented below.
        pub key_spec: pulumi_wasm_rust::Output<
            super::super::types::certificateauthority::AuthorityKeySpec,
        >,
        /// Labels with user-defined metadata. An object containing a list of "key": value pairs. Example: { "name": "wrench",
        /// "mass": "1.3kg", "count": "3" }. **Note**: This field is non-authoritative, and will only manage the labels present in
        /// your configuration. Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and "notAfterTime" fields inside an X.509
        /// certificate. A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
        pub lifetime: pulumi_wasm_rust::Output<Option<String>>,
        /// Location of the CertificateAuthority. A full list of valid locations can be found by
        /// running `gcloud privateca locations list`.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name for this CertificateAuthority in the format
        /// projects/*/locations/*/certificateAuthorities/*.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The signed CA certificate issued from the subordinated CA's CSR. This is needed when activating the subordiante CA with
        /// a third party issuer.
        pub pem_ca_certificate: pulumi_wasm_rust::Output<Option<String>>,
        /// This CertificateAuthority's certificate chain, including the current
        /// CertificateAuthority's certificate. Ordered such that the root issuer is the final
        /// element (consistent with RFC 5246). For a self-signed CA, this will only list the current
        /// CertificateAuthority's certificate.
        pub pem_ca_certificates: pulumi_wasm_rust::Output<Vec<String>>,
        /// The name of the CaPool this Certificate Authority belongs to.
        pub pool: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If this flag is set, the Certificate Authority will be deleted as soon as possible without a 30-day grace period where
        /// undeletion would have been allowed. If you proceed, there will be no way to recover this CA. Use with care. Defaults to
        /// 'false'.
        pub skip_grace_period: pulumi_wasm_rust::Output<Option<bool>>,
        /// The State for this CertificateAuthority.
        pub state: pulumi_wasm_rust::Output<String>,
        /// If this is a subordinate CertificateAuthority, this field will be set with the subordinate configuration, which
        /// describes its issuers.
        pub subordinate_config: pulumi_wasm_rust::Output<
            Option<super::super::types::certificateauthority::AuthoritySubordinateConfig>,
        >,
        /// The Type of this CertificateAuthority. > **Note:** For 'SUBORDINATE' Certificate Authorities, they need to be activated
        /// before they can issue certificates. Default value: "SELF_SIGNED" Possible values: ["SELF_SIGNED", "SUBORDINATE"]
        pub type_: pulumi_wasm_rust::Output<Option<String>>,
        /// The time at which this CertificateAuthority was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AuthorityArgs,
    ) -> AuthorityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_authority_id_binding = args
            .certificate_authority_id
            .get_output(context)
            .get_inner();
        let config_binding = args.config.get_output(context).get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let desired_state_binding = args.desired_state.get_output(context).get_inner();
        let gcs_bucket_binding = args.gcs_bucket.get_output(context).get_inner();
        let ignore_active_certificates_on_deletion_binding = args
            .ignore_active_certificates_on_deletion
            .get_output(context)
            .get_inner();
        let key_spec_binding = args.key_spec.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let lifetime_binding = args.lifetime.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let pem_ca_certificate_binding = args
            .pem_ca_certificate
            .get_output(context)
            .get_inner();
        let pool_binding = args.pool.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let skip_grace_period_binding = args
            .skip_grace_period
            .get_output(context)
            .get_inner();
        let subordinate_config_binding = args
            .subordinate_config
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificateauthority/authority:Authority".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateAuthorityId".into(),
                    value: &certificate_authority_id_binding,
                },
                register_interface::ObjectField {
                    name: "config".into(),
                    value: &config_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "desiredState".into(),
                    value: &desired_state_binding,
                },
                register_interface::ObjectField {
                    name: "gcsBucket".into(),
                    value: &gcs_bucket_binding,
                },
                register_interface::ObjectField {
                    name: "ignoreActiveCertificatesOnDeletion".into(),
                    value: &ignore_active_certificates_on_deletion_binding,
                },
                register_interface::ObjectField {
                    name: "keySpec".into(),
                    value: &key_spec_binding,
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
                    name: "pemCaCertificate".into(),
                    value: &pem_ca_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "pool".into(),
                    value: &pool_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "skipGracePeriod".into(),
                    value: &skip_grace_period_binding,
                },
                register_interface::ObjectField {
                    name: "subordinateConfig".into(),
                    value: &subordinate_config_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AuthorityResult {
            access_urls: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("accessUrls"),
            ),
            certificate_authority_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateAuthorityId"),
            ),
            config: pulumi_wasm_rust::__private::into_domain(o.extract_field("config")),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            desired_state: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("desiredState"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            gcs_bucket: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("gcsBucket"),
            ),
            ignore_active_certificates_on_deletion: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ignoreActiveCertificatesOnDeletion"),
            ),
            key_spec: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("keySpec"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            lifetime: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lifetime"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            pem_ca_certificate: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pemCaCertificate"),
            ),
            pem_ca_certificates: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pemCaCertificates"),
            ),
            pool: pulumi_wasm_rust::__private::into_domain(o.extract_field("pool")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            skip_grace_period: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("skipGracePeriod"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            subordinate_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subordinateConfig"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
