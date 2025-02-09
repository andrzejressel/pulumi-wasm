/// Certificate Authority Service provides reusable and parameterized templates that you can use for common certificate issuance scenarios. A certificate template represents a relatively static and well-defined certificate issuance schema within an organization. A certificate template can essentially become a full-fledged vertical certificate issuance framework.
///
///
/// To get more information about CertificateTemplate, see:
///
/// * [API documentation](https://cloud.google.com/certificate-authority-service/docs/reference/rest)
/// * How-to Guides
///     * [Common configurations and Certificate Profiles](https://cloud.google.com/certificate-authority-service/docs/certificate-profile)
///     * [Official Documentation](https://cloud.google.com/certificate-authority-service)
///     * [Understanding Certificate Templates](https://cloud.google.com/certificate-authority-service/docs/certificate-template)
///
/// ## Example Usage
///
/// ### Privateca Template Basic
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:certificateauthority:CertificateTemplate
///     properties:
///       name: my-template
///       location: us-central1
///       description: A sample certificate template
///       identityConstraints:
///         allowSubjectAltNamesPassthrough: true
///         allowSubjectPassthrough: true
///         celExpression:
///           description: Always true
///           expression: 'true'
///           location: any.file.anywhere
///           title: Sample expression
///       maximumLifetime: 86400s
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
///       labels:
///         label-one: value-one
/// ```
///
/// ## Import
///
/// CertificateTemplate can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/certificateTemplates/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, CertificateTemplate can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplate:CertificateTemplate default projects/{{project}}/locations/{{location}}/certificateTemplates/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplate:CertificateTemplate default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:certificateauthority/certificateTemplate:CertificateTemplate default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateTemplateArgs {
        /// Optional. A human-readable description of scenarios this template is intended for.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Describes constraints on identities that may be appear in Certificates issued using this template. If this is omitted, then this template will not add restrictions on a certificate's identity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub identity_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::certificateauthority::CertificateTemplateIdentityConstraints,
            >,
        >,
        /// Optional. Labels with user-defined metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Optional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it.
        #[builder(into, default)]
        pub maximum_lifetime: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optional. Describes the set of X.509 extensions that may appear in a Certificate issued using this CertificateTemplate. If a certificate request sets extensions that don't appear in the passthrough_extensions, those extensions will be dropped. If the issuing CaPool's IssuancePolicy defines baseline_values that don't appear here, the certificate issuance request will fail. If this is omitted, then this template will not add restrictions on a certificate's X.509 extensions. These constraints do not apply to X.509 extensions set in this CertificateTemplate's predefined_values.
        /// Structure is documented below.
        #[builder(into, default)]
        pub passthrough_extensions: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::certificateauthority::CertificateTemplatePassthroughExtensions,
            >,
        >,
        /// Optional. A set of X.509 values that will be applied to all issued certificates that use this template. If the certificate request includes conflicting values for the same properties, they will be overwritten by the values defined here. If the issuing CaPool's IssuancePolicy defines conflicting baseline_values for the same properties, the certificate issuance request will fail.
        /// Structure is documented below.
        #[builder(into, default)]
        pub predefined_values: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::certificateauthority::CertificateTemplatePredefinedValues,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct CertificateTemplateResult {
        /// Output only. The time at which this CertificateTemplate was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Optional. A human-readable description of scenarios this template is intended for.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Optional. Describes constraints on identities that may be appear in Certificates issued using this template. If this is omitted, then this template will not add restrictions on a certificate's identity.
        /// Structure is documented below.
        pub identity_constraints: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::certificateauthority::CertificateTemplateIdentityConstraints,
            >,
        >,
        /// Optional. Labels with user-defined metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location for the resource
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Optional. The maximum lifetime allowed for all issued certificates that use this template. If the issuing CaPool's IssuancePolicy specifies a maximum lifetime the minimum of the two durations will be the maximum lifetime for issued. Note that if the issuing CertificateAuthority expires before a Certificate's requested maximum_lifetime, the effective lifetime will be explicitly truncated to match it.
        pub maximum_lifetime: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource name for this CertificateTemplate in the format `projects/*/locations/*/certificateTemplates/*`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Optional. Describes the set of X.509 extensions that may appear in a Certificate issued using this CertificateTemplate. If a certificate request sets extensions that don't appear in the passthrough_extensions, those extensions will be dropped. If the issuing CaPool's IssuancePolicy defines baseline_values that don't appear here, the certificate issuance request will fail. If this is omitted, then this template will not add restrictions on a certificate's X.509 extensions. These constraints do not apply to X.509 extensions set in this CertificateTemplate's predefined_values.
        /// Structure is documented below.
        pub passthrough_extensions: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::certificateauthority::CertificateTemplatePassthroughExtensions,
            >,
        >,
        /// Optional. A set of X.509 values that will be applied to all issued certificates that use this template. If the certificate request includes conflicting values for the same properties, they will be overwritten by the values defined here. If the issuing CaPool's IssuancePolicy defines conflicting baseline_values for the same properties, the certificate issuance request will fail.
        /// Structure is documented below.
        pub predefined_values: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::certificateauthority::CertificateTemplatePredefinedValues,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The time at which this CertificateTemplate was updated.
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CertificateTemplateArgs,
    ) -> CertificateTemplateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let identity_constraints_binding = args.identity_constraints.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let maximum_lifetime_binding = args.maximum_lifetime.get_output(context);
        let name_binding = args.name.get_output(context);
        let passthrough_extensions_binding = args
            .passthrough_extensions
            .get_output(context);
        let predefined_values_binding = args.predefined_values.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:certificateauthority/certificateTemplate:CertificateTemplate"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identityConstraints".into(),
                    value: identity_constraints_binding.get_id(),
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
                    name: "maximumLifetime".into(),
                    value: maximum_lifetime_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "passthroughExtensions".into(),
                    value: passthrough_extensions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "predefinedValues".into(),
                    value: predefined_values_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CertificateTemplateResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            identity_constraints: o.get_field("identityConstraints"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            maximum_lifetime: o.get_field("maximumLifetime"),
            name: o.get_field("name"),
            passthrough_extensions: o.get_field("passthroughExtensions"),
            predefined_values: o.get_field("predefinedValues"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            update_time: o.get_field("updateTime"),
        }
    }
}
