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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustConfigArgs {
        /// Allowlisted PEM-encoded certificates. A certificate matching an allowlisted certificate is always considered valid as long as
        /// the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met.
        /// Structure is documented below.
        #[builder(into, default)]
        pub allowlisted_certificates: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::certificatemanager::TrustConfigAllowlistedCertificate,
                >,
            >,
        >,
        /// One or more paragraphs of text description of a trust config.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of label tags associated with the trust config.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The trust config location.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A user-defined name of the trust config. Trust config names must be unique globally.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of trust stores to perform validation against.
        /// This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation.
        /// Structure is documented below.
        #[builder(into, default)]
        pub trust_stores: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStore>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustConfigResult {
        /// Allowlisted PEM-encoded certificates. A certificate matching an allowlisted certificate is always considered valid as long as
        /// the certificate is parseable, proof of private key possession is established, and constraints on the certificate's SAN field are met.
        /// Structure is documented below.
        pub allowlisted_certificates: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::certificatemanager::TrustConfigAllowlistedCertificate,
                >,
            >,
        >,
        /// The creation timestamp of a TrustConfig.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// One or more paragraphs of text description of a trust config.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of label tags associated with the trust config.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The trust config location.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A user-defined name of the trust config. Trust config names must be unique globally.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Set of trust stores to perform validation against.
        /// This field is supported when TrustConfig is configured with Load Balancers, currently not supported for SPIFFE certificate validation.
        /// Structure is documented below.
        pub trust_stores: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::certificatemanager::TrustConfigTrustStore>>,
        >,
        /// The last update timestamp of a TrustConfig.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
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
        args: TrustConfigArgs,
    ) -> TrustConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allowlisted_certificates_binding = args
            .allowlisted_certificates
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let trust_stores_binding = args.trust_stores.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificatemanager/trustConfig:TrustConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowlistedCertificates".into(),
                    value: &allowlisted_certificates_binding.drop_type(),
                },
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
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trustStores".into(),
                    value: &trust_stores_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustConfigResult {
            allowlisted_certificates: o.get_field("allowlistedCertificates"),
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            trust_stores: o.get_field("trustStores"),
            update_time: o.get_field("updateTime"),
        }
    }
}
