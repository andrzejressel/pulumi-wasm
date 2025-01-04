/// TrustConfig represents a resource that represents your Public Key Infrastructure (PKI) configuration in Certificate Manager for use in mutual TLS authentication scenarios.
///
///
/// To get more information about TrustConfig, see:
///
/// * [API documentation](https://cloud.google.com/certificate-manager/docs/reference/certificate-manager/rest/v1/projects.locations.trustConfigs/create)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/certificate-manager/docs)
///
///
///
/// ## Example Usage
///
/// ### Certificate Manager Trust Config
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:TrustConfig
///     properties:
///       name: trust-config
///       description: sample description for the trust config
///       location: us-central1
///       trustStores:
///         - trustAnchors:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/cert.pem
///                   return: result
///           intermediateCas:
///             - pemCertificate:
///                 fn::invoke:
///                   function: std:file
///                   arguments:
///                     input: test-fixtures/cert.pem
///                   return: result
///       labels:
///         foo: bar
/// ```
/// ### Certificate Manager Trust Config Allowlisted Certificates
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificatemanager:TrustConfig
///     properties:
///       name: trust-config
///       description: A sample trust config resource with allowlisted certificates
///       location: global
///       allowlistedCertificates:
///         - pemCertificate:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: test-fixtures/cert.pem
///               return: result
///         - pemCertificate:
///             fn::invoke:
///               function: std:file
///               arguments:
///                 input: test-fixtures/cert2.pem
///               return: result
///       labels:
///         foo: bar
/// ```
///
/// ## Import
///
/// TrustConfig can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/trustConfigs/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, TrustConfig can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/trustConfig:TrustConfig default projects/{{project}}/locations/{{location}}/trustConfigs/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/trustConfig:TrustConfig default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificatemanager/trustConfig:TrustConfig default {{location}}/{{name}}
/// ```
///
pub mod trust_config {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustConfigArgs {
        /// Allowlisted PEM-encoded certificates. A certificate matching an allowlisted certificate is always considered valid as long as
        /// the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met.
        /// Structure is documented below.
        #[builder(into, default)]
        pub allowlisted_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::certificatemanager::TrustConfigAllowlistedCertificate,
                >,
            >,
        >,
        /// One or more paragraphs of text description of a trust config.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of label tags associated with the trust config.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The trust config location.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// A user-defined name of the trust config. Trust config names must be unique globally.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Set of trust stores to perform validation against.
        /// This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub trust_stores: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStore>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustConfigResult {
        /// Allowlisted PEM-encoded certificates. A certificate matching an allowlisted certificate is always considered valid as long as
        /// the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met.
        /// Structure is documented below.
        pub allowlisted_certificates: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::certificatemanager::TrustConfigAllowlistedCertificate,
                >,
            >,
        >,
        /// The creation timestamp of a TrustConfig.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// One or more paragraphs of text description of a trust config.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the trust config.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The trust config location.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// A user-defined name of the trust config. Trust config names must be unique globally.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of trust stores to perform validation against.
        /// This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation.
        /// Structure is documented below.
        pub trust_stores: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStore>>,
        >,
        /// The last update timestamp of a TrustConfig.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TrustConfigArgs) -> TrustConfigResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let allowlisted_certificates_binding = args.allowlisted_certificates.get_inner();
        let description_binding = args.description.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let trust_stores_binding = args.trust_stores.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:certificatemanager/trustConfig:TrustConfig".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allowlistedCertificates".into(),
                    value: &allowlisted_certificates_binding,
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "trustStores".into(),
                    value: &trust_stores_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "allowlistedCertificates".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "trustStores".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrustConfigResult {
            allowlisted_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowlistedCertificates").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
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
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            trust_stores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trustStores").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
