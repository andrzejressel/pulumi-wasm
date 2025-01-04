/// An Environment Keystore Alias for Self Signed Certificate Format in Apigee
///
///
/// To get more information about KeystoresAliasesSelfSignedCert, see:
///
/// * [API documentation](https://cloud.google.com/apigee/docs/reference/apis/apigee/rest/v1/organizations.environments.keystores.aliases/create)
/// * How-to Guides
///     * [Creating an environment](https://cloud.google.com/apigee/docs/api-platform/get-started/create-environment)
///
/// ## Example Usage
///
/// ### Apigee Env Keystore Alias Self Signed Cert
///
///
/// ```yaml
/// resources:
///   project:
///     type: gcp:organizations:Project
///     properties:
///       projectId: my-project
///       name: my-project
///       orgId: '123456789'
///       billingAccount: 000000-0000000-0000000-000000
///       deletionPolicy: DELETE
///   apigee:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: apigee.googleapis.com
///   servicenetworking:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: servicenetworking.googleapis.com
///     options:
///       dependsOn:
///         - ${apigee}
///   compute:
///     type: gcp:projects:Service
///     properties:
///       project: ${project.projectId}
///       service: compute.googleapis.com
///     options:
///       dependsOn:
///         - ${servicenetworking}
///   apigeeNetwork:
///     type: gcp:compute:Network
///     name: apigee_network
///     properties:
///       name: apigee-network
///       project: ${project.projectId}
///     options:
///       dependsOn:
///         - ${compute}
///   apigeeRange:
///     type: gcp:compute:GlobalAddress
///     name: apigee_range
///     properties:
///       name: apigee-range
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${apigeeNetwork.id}
///       project: ${project.projectId}
///   apigeeVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: apigee_vpc_connection
///     properties:
///       network: ${apigeeNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${apigeeRange.name}
///     options:
///       dependsOn:
///         - ${servicenetworking}
///   apigeeOrg:
///     type: gcp:apigee:Organization
///     name: apigee_org
///     properties:
///       analyticsRegion: us-central1
///       projectId: ${project.projectId}
///       authorizedNetwork: ${apigeeNetwork.id}
///     options:
///       dependsOn:
///         - ${apigeeVpcConnection}
///         - ${apigee}
///   apigeeEnvironmentKeystoreSsAlias:
///     type: gcp:apigee:Environment
///     name: apigee_environment_keystore_ss_alias
///     properties:
///       orgId: ${apigeeOrg.id}
///       name: env-name
///       description: Apigee Environment
///       displayName: environment-1
///   apigeeEnvironmentKeystoreAlias:
///     type: gcp:apigee:EnvKeystore
///     name: apigee_environment_keystore_alias
///     properties:
///       name: env-keystore
///       envId: ${apigeeEnvironmentKeystoreSsAlias.id}
///   apigeeEnvironmentKeystoreSsAliasKeystoresAliasesSelfSignedCert:
///     type: gcp:apigee:KeystoresAliasesSelfSignedCert
///     name: apigee_environment_keystore_ss_alias
///     properties:
///       environment: ${apigeeEnvironmentKeystoreSsAlias.name}
///       orgId: ${apigeeOrg.name}
///       keystore: ${apigeeEnvironmentKeystoreAlias.name}
///       alias: alias
///       keySize: 1024
///       sigAlg: SHA512withRSA
///       certValidityInDays: 4
///       subject:
///         commonName: selfsigned_example
///         countryCode: US
///         locality: TX
///         org: CCE
///         orgUnit: PSO
/// ```
///
/// ## Import
///
/// KeystoresAliasesSelfSignedCert can be imported using any of these accepted formats:
///
/// * `organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}`
///
/// * `{{org_id}}/{{environment}}/{{keystore}}/{{alias}}`
///
/// When using the `pulumi import` command, KeystoresAliasesSelfSignedCert can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesSelfSignedCert:KeystoresAliasesSelfSignedCert default organizations/{{org_id}}/environments/{{environment}}/keystores/{{keystore}}/aliases/{{alias}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:apigee/keystoresAliasesSelfSignedCert:KeystoresAliasesSelfSignedCert default {{org_id}}/{{environment}}/{{keystore}}/{{alias}}
/// ```
///
pub mod keystores_aliases_self_signed_cert {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeystoresAliasesSelfSignedCertArgs {
        /// Alias for the key/certificate pair. Values must match the regular expression [\w\s-.]{1,255}.
        /// This must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either
        /// this parameter or the JSON body.
        #[builder(into)]
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Validity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365.
        #[builder(into, default)]
        pub cert_validity_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// The Apigee environment name
        #[builder(into)]
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Key size. Default and maximum value is 2048 bits.
        #[builder(into, default)]
        pub key_size: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee keystore name associated in an Apigee environment
        #[builder(into)]
        pub keystore: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization name associated with the Apigee environment
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Signature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA
        #[builder(into)]
        pub sig_alg: pulumi_wasm_rust::Output<String>,
        /// Subject details.
        /// Structure is documented below.
        #[builder(into)]
        pub subject: pulumi_wasm_rust::Output<
            super::super::types::apigee::KeystoresAliasesSelfSignedCertSubject,
        >,
        /// List of alternative host names. Maximum length is 255 characters for each value.
        #[builder(into, default)]
        pub subject_alternative_dns_names: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apigee::KeystoresAliasesSelfSignedCertSubjectAlternativeDnsNames,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct KeystoresAliasesSelfSignedCertResult {
        /// Alias for the key/certificate pair. Values must match the regular expression [\w\s-.]{1,255}.
        /// This must be provided for all formats except selfsignedcert; self-signed certs may specify the alias in either
        /// this parameter or the JSON body.
        pub alias: pulumi_wasm_rust::Output<String>,
        /// Validity duration of certificate, in days. Accepts positive non-zero value. Defaults to 365.
        pub cert_validity_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Chain of certificates under this alias.
        /// Structure is documented below.
        pub certs_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::apigee::KeystoresAliasesSelfSignedCertCertsInfo>,
        >,
        /// The Apigee environment name
        pub environment: pulumi_wasm_rust::Output<String>,
        /// Key size. Default and maximum value is 2048 bits.
        pub key_size: pulumi_wasm_rust::Output<Option<String>>,
        /// The Apigee keystore name associated in an Apigee environment
        pub keystore: pulumi_wasm_rust::Output<String>,
        /// The Apigee Organization name associated with the Apigee environment
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// Signature algorithm to generate private key. Valid values are SHA512withRSA, SHA384withRSA, and SHA256withRSA
        pub sig_alg: pulumi_wasm_rust::Output<String>,
        /// Subject details.
        /// Structure is documented below.
        pub subject: pulumi_wasm_rust::Output<
            super::super::types::apigee::KeystoresAliasesSelfSignedCertSubject,
        >,
        /// List of alternative host names. Maximum length is 255 characters for each value.
        pub subject_alternative_dns_names: pulumi_wasm_rust::Output<
            Option<
                super::super::types::apigee::KeystoresAliasesSelfSignedCertSubjectAlternativeDnsNames,
            >,
        >,
        /// Optional.Type of Alias
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: KeystoresAliasesSelfSignedCertArgs,
    ) -> KeystoresAliasesSelfSignedCertResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alias_binding = args.alias.get_inner();
        let cert_validity_in_days_binding = args.cert_validity_in_days.get_inner();
        let environment_binding = args.environment.get_inner();
        let key_size_binding = args.key_size.get_inner();
        let keystore_binding = args.keystore.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let sig_alg_binding = args.sig_alg.get_inner();
        let subject_binding = args.subject.get_inner();
        let subject_alternative_dns_names_binding = args
            .subject_alternative_dns_names
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:apigee/keystoresAliasesSelfSignedCert:KeystoresAliasesSelfSignedCert"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alias".into(),
                    value: &alias_binding,
                },
                register_interface::ObjectField {
                    name: "certValidityInDays".into(),
                    value: &cert_validity_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "environment".into(),
                    value: &environment_binding,
                },
                register_interface::ObjectField {
                    name: "keySize".into(),
                    value: &key_size_binding,
                },
                register_interface::ObjectField {
                    name: "keystore".into(),
                    value: &keystore_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
                register_interface::ObjectField {
                    name: "sigAlg".into(),
                    value: &sig_alg_binding,
                },
                register_interface::ObjectField {
                    name: "subject".into(),
                    value: &subject_binding,
                },
                register_interface::ObjectField {
                    name: "subjectAlternativeDnsNames".into(),
                    value: &subject_alternative_dns_names_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alias".into(),
                },
                register_interface::ResultField {
                    name: "certValidityInDays".into(),
                },
                register_interface::ResultField {
                    name: "certsInfos".into(),
                },
                register_interface::ResultField {
                    name: "environment".into(),
                },
                register_interface::ResultField {
                    name: "keySize".into(),
                },
                register_interface::ResultField {
                    name: "keystore".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
                },
                register_interface::ResultField {
                    name: "sigAlg".into(),
                },
                register_interface::ResultField {
                    name: "subject".into(),
                },
                register_interface::ResultField {
                    name: "subjectAlternativeDnsNames".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KeystoresAliasesSelfSignedCertResult {
            alias: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alias").unwrap(),
            ),
            cert_validity_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certValidityInDays").unwrap(),
            ),
            certs_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("certsInfos").unwrap(),
            ),
            environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("environment").unwrap(),
            ),
            key_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keySize").unwrap(),
            ),
            keystore: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keystore").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            sig_alg: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sigAlg").unwrap(),
            ),
            subject: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subject").unwrap(),
            ),
            subject_alternative_dns_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subjectAlternativeDnsNames").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
