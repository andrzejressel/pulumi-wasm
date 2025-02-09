/// Resource for managing a Roles Anywhere Trust Anchor.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:acmpca:CertificateAuthority
///     properties:
///       permanentDeletionTimeInDays: 7
///       type: ROOT
///       certificateAuthorityConfiguration:
///         keyAlgorithm: RSA_4096
///         signingAlgorithm: SHA512WITHRSA
///         subject:
///           commonName: example.com
///   test:
///     type: aws:acmpca:Certificate
///     properties:
///       certificateAuthorityArn: ${example.arn}
///       certificateSigningRequest: ${example.certificateSigningRequest}
///       signingAlgorithm: SHA512WITHRSA
///       templateArn: arn:${current.partition}:acm-pca:::template/RootCACertificate/V1
///       validity:
///         type: YEARS
///         value: 1
///   exampleCertificateAuthorityCertificate:
///     type: aws:acmpca:CertificateAuthorityCertificate
///     name: example
///     properties:
///       certificateAuthorityArn: ${example.arn}
///       certificate: ${exampleAwsAcmpcaCertificate.certificate}
///       certificateChain: ${exampleAwsAcmpcaCertificate.certificateChain}
///   testTrustAnchor:
///     type: aws:rolesanywhere:TrustAnchor
///     name: test
///     properties:
///       name: example
///       source:
///         sourceData:
///           acmPcaArn: ${example.arn}
///         sourceType: AWS_ACM_PCA
///     options:
///       dependsOn:
///         - ${exampleCertificateAuthorityCertificate}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getPartition
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rolesanywhere_trust_anchor` using its `id`. For example:
///
/// ```sh
/// $ pulumi import aws:rolesanywhere/trustAnchor:TrustAnchor example 92b2fbbb-984d-41a3-a765-e3cbdb69ebb1
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod trust_anchor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustAnchorArgs {
        /// Whether or not the Trust Anchor should be enabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the Trust Anchor.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub notification_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::rolesanywhere::TrustAnchorNotificationSetting>,
            >,
        >,
        /// The source of trust, documented below
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::rolesanywhere::TrustAnchorSource,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustAnchorResult {
        /// Amazon Resource Name (ARN) of the Trust Anchor
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the Trust Anchor should be enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name of the Trust Anchor.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub notification_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rolesanywhere::TrustAnchorNotificationSetting>,
        >,
        /// The source of trust, documented below
        pub source: pulumi_gestalt_rust::Output<
            super::super::types::rolesanywhere::TrustAnchorSource,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TrustAnchorArgs,
    ) -> TrustAnchorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_settings_binding = args
            .notification_settings
            .get_output(context);
        let source_binding = args.source.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rolesanywhere/trustAnchor:TrustAnchor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationSettings".into(),
                    value: notification_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TrustAnchorResult {
            arn: o.get_field("arn"),
            enabled: o.get_field("enabled"),
            name: o.get_field("name"),
            notification_settings: o.get_field("notificationSettings"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
