/// The TlsInspectionPolicy resource contains references to CA pools in Certificate Authority Service and associated metadata.
///
///
/// To get more information about TlsInspectionPolicy, see:
///
/// * [API documentation](https://cloud.google.com/secure-web-proxy/docs/reference/network-security/rest/v1/projects.locations.tlsInspectionPolicies)
/// * How-to Guides
///     * [Use TlsInspectionPolicy](https://cloud.google.com/secure-web-proxy/docs/tls-inspection-overview)
///
/// ## Example Usage
///
/// ### Network Security Tls Inspection Policy Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: my-basic-ca-pool
///       location: us-central1
///       tier: DEVOPS
///       publishingOptions:
///         publishCaCert: false
///         publishCrl: false
///       issuancePolicy:
///         maximumLifetime: 1209600s
///         baselineValues:
///           caOptions:
///             isCa: false
///           keyUsage:
///             baseKeyUsage: {}
///             extendedKeyUsage:
///               serverAuth: true
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       pool: ${default.name}
///       certificateAuthorityId: my-basic-certificate-authority
///       location: us-central1
///       lifetime: 86400s
///       type: SELF_SIGNED
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///       config:
///         subjectConfig:
///           subject:
///             organization: Test LLC
///             commonName: my-ca
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
///   tlsInspectionPermission:
///     type: gcp:certificateauthority:CaPoolIamMember
///     name: tls_inspection_permission
///     properties:
///       caPool: ${default.id}
///       role: roles/privateca.certificateManager
///       member: serviceAccount:service-${project.number}@gcp-sa-networksecurity.iam.gserviceaccount.com
///   defaultTlsInspectionPolicy:
///     type: gcp:networksecurity:TlsInspectionPolicy
///     name: default
///     properties:
///       name: my-tls-inspection-policy
///       location: us-central1
///       caPool: ${default.id}
///       excludePublicCaSet: false
///     options:
///       dependsOn:
///         - ${default}
///         - ${defaultAuthority}
///         - ${tlsInspectionPermission}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Network Security Tls Inspection Policy Custom
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CaPool
///     properties:
///       name: my-basic-ca-pool
///       location: us-central1
///       tier: DEVOPS
///       publishingOptions:
///         publishCaCert: false
///         publishCrl: false
///       issuancePolicy:
///         maximumLifetime: 1209600s
///         baselineValues:
///           caOptions:
///             isCa: false
///           keyUsage:
///             baseKeyUsage: {}
///             extendedKeyUsage:
///               serverAuth: true
///   defaultAuthority:
///     type: gcp:certificateauthority:Authority
///     name: default
///     properties:
///       pool: ${default.name}
///       certificateAuthorityId: my-basic-certificate-authority
///       location: us-central1
///       lifetime: 86400s
///       type: SELF_SIGNED
///       deletionProtection: false
///       skipGracePeriod: true
///       ignoreActiveCertificatesOnDeletion: true
///       config:
///         subjectConfig:
///           subject:
///             organization: Test LLC
///             commonName: my-ca
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
///   nsSa:
///     type: gcp:projects:ServiceIdentity
///     name: ns_sa
///     properties:
///       service: networksecurity.googleapis.com
///   defaultCaPoolIamMember:
///     type: gcp:certificateauthority:CaPoolIamMember
///     name: default
///     properties:
///       caPool: ${default.id}
///       role: roles/privateca.certificateManager
///       member: ${nsSa.member}
///   defaultTrustConfig:
///     type: gcp:certificatemanager:TrustConfig
///     name: default
///     properties:
///       name: my-trust-config
///       description: sample trust config description
///       location: us-central1
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
///   defaultTlsInspectionPolicy:
///     type: gcp:networksecurity:TlsInspectionPolicy
///     name: default
///     properties:
///       name: my-tls-inspection-policy
///       location: us-central1
///       caPool: ${default.id}
///       excludePublicCaSet: false
///       minTlsVersion: TLS_1_0
///       trustConfig: ${defaultTrustConfig.id}
///       tlsFeatureProfile: PROFILE_CUSTOM
///       customTlsFeatures:
///         - TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA
///         - TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
///         - TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA
///         - TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
///         - TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256
///         - TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA
///         - TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
///         - TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA
///         - TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
///         - TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256
///         - TLS_RSA_WITH_3DES_EDE_CBC_SHA
///         - TLS_RSA_WITH_AES_128_CBC_SHA
///         - TLS_RSA_WITH_AES_128_GCM_SHA256
///         - TLS_RSA_WITH_AES_256_CBC_SHA
///         - TLS_RSA_WITH_AES_256_GCM_SHA384
///     options:
///       dependsOn:
///         - ${defaultAuthority}
///         - ${defaultCaPoolIamMember}
/// ```
///
/// ## Import
///
/// TlsInspectionPolicy can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/tlsInspectionPolicies/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, TlsInspectionPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:networksecurity/tlsInspectionPolicy:TlsInspectionPolicy default projects/{{project}}/locations/{{location}}/tlsInspectionPolicies/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/tlsInspectionPolicy:TlsInspectionPolicy default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:networksecurity/tlsInspectionPolicy:TlsInspectionPolicy default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod tls_inspection_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TlsInspectionPolicyArgs {
        /// A CA pool resource used to issue interception certificates.
        #[builder(into)]
        pub ca_pool: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field.
        #[builder(into, default)]
        pub custom_tls_features: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Free-text description of the resource.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trustConfig. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trustConfig will be accepted.
        #[builder(into, default)]
        pub exclude_public_ca_set: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The location of the tls inspection policy.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
        /// Possible values are: `TLS_VERSION_UNSPECIFIED`, `TLS_1_0`, `TLS_1_1`, `TLS_1_2`, `TLS_1_3`.
        #[builder(into, default)]
        pub min_tls_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Short name of the TlsInspectionPolicy resource to be created.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers (\"PROFILE_COMPATIBLE\"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
        /// Possible values are: `PROFILE_UNSPECIFIED`, `PROFILE_COMPATIBLE`, `PROFILE_MODERN`, `PROFILE_RESTRICTED`, `PROFILE_CUSTOM`.
        #[builder(into, default)]
        pub tls_feature_profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form \"projects/{project}/locations/{location}/trustConfigs/{trust_config}\". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Trust config and the TLS inspection policy must be in the same region. Note that Secure Web Proxy does not yet honor this field.
        #[builder(into, default)]
        pub trust_config: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TlsInspectionPolicyResult {
        /// A CA pool resource used to issue interception certificates.
        pub ca_pool: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// List of custom TLS cipher suites selected. This field is valid only if the selected tls_feature_profile is CUSTOM. The compute.SslPoliciesService.ListAvailableFeatures method returns the set of features that can be specified in this list. Note that Secure Web Proxy does not yet honor this field.
        pub custom_tls_features: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Free-text description of the resource.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If FALSE (the default), use our default set of public CAs in addition to any CAs specified in trustConfig. These public CAs are currently based on the Mozilla Root Program and are subject to change over time. If TRUE, do not accept our default set of public CAs. Only CAs specified in trustConfig will be accepted.
        pub exclude_public_ca_set: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location of the tls inspection policy.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Minimum TLS version that the firewall should use when negotiating connections with both clients and servers. If this is not set, then the default value is to allow the broadest set of clients and servers (TLS 1.0 or higher). Setting this to more restrictive values may improve security, but may also prevent the firewall from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
        /// Possible values are: `TLS_VERSION_UNSPECIFIED`, `TLS_1_0`, `TLS_1_1`, `TLS_1_2`, `TLS_1_3`.
        pub min_tls_version: pulumi_gestalt_rust::Output<Option<String>>,
        /// Short name of the TlsInspectionPolicy resource to be created.
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The selected Profile. If this is not set, then the default value is to allow the broadest set of clients and servers (\"PROFILE_COMPATIBLE\"). Setting this to more restrictive values may improve security, but may also prevent the TLS inspection proxy from connecting to some clients or servers. Note that Secure Web Proxy does not yet honor this field.
        /// Possible values are: `PROFILE_UNSPECIFIED`, `PROFILE_COMPATIBLE`, `PROFILE_MODERN`, `PROFILE_RESTRICTED`, `PROFILE_CUSTOM`.
        pub tls_feature_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// A TrustConfig resource used when making a connection to the TLS server. This is a relative resource path following the form \"projects/{project}/locations/{location}/trustConfigs/{trust_config}\". This is necessary to intercept TLS connections to servers with certificates signed by a private CA or self-signed certificates. Trust config and the TLS inspection policy must be in the same region. Note that Secure Web Proxy does not yet honor this field.
        pub trust_config: pulumi_gestalt_rust::Output<Option<String>>,
        /// The timestamp when the resource was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TlsInspectionPolicyArgs,
    ) -> TlsInspectionPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ca_pool_binding = args.ca_pool.get_output(context).get_inner();
        let custom_tls_features_binding = args
            .custom_tls_features
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let exclude_public_ca_set_binding = args
            .exclude_public_ca_set
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let min_tls_version_binding = args
            .min_tls_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let tls_feature_profile_binding = args
            .tls_feature_profile
            .get_output(context)
            .get_inner();
        let trust_config_binding = args.trust_config.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:networksecurity/tlsInspectionPolicy:TlsInspectionPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "caPool".into(),
                    value: &ca_pool_binding,
                },
                register_interface::ObjectField {
                    name: "customTlsFeatures".into(),
                    value: &custom_tls_features_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "excludePublicCaSet".into(),
                    value: &exclude_public_ca_set_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minTlsVersion".into(),
                    value: &min_tls_version_binding,
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
                    name: "tlsFeatureProfile".into(),
                    value: &tls_feature_profile_binding,
                },
                register_interface::ObjectField {
                    name: "trustConfig".into(),
                    value: &trust_config_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TlsInspectionPolicyResult {
            ca_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caPool"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            custom_tls_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customTlsFeatures"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            exclude_public_ca_set: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("excludePublicCaSet"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            min_tls_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minTlsVersion"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            tls_feature_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsFeatureProfile"),
            ),
            trust_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("trustConfig"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
