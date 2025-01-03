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
pub mod trust_anchor {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TrustAnchorArgs {
        /// Whether or not the Trust Anchor should be enabled.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Trust Anchor.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub notification_settings: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::rolesanywhere::TrustAnchorNotificationSetting>,
            >,
        >,
        /// The source of trust, documented below
        #[builder(into)]
        pub source: pulumi_wasm_rust::Output<
            super::super::types::rolesanywhere::TrustAnchorSource,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TrustAnchorResult {
        /// Amazon Resource Name (ARN) of the Trust Anchor
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Whether or not the Trust Anchor should be enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The name of the Trust Anchor.
        pub name: pulumi_wasm_rust::Output<String>,
        pub notification_settings: pulumi_wasm_rust::Output<
            Vec<super::super::types::rolesanywhere::TrustAnchorNotificationSetting>,
        >,
        /// The source of trust, documented below
        pub source: pulumi_wasm_rust::Output<
            super::super::types::rolesanywhere::TrustAnchorSource,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TrustAnchorArgs) -> TrustAnchorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_inner();
        let name_binding = args.name.get_inner();
        let notification_settings_binding = args.notification_settings.get_inner();
        let source_binding = args.source.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rolesanywhere/trustAnchor:TrustAnchor".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notificationSettings".into(),
                    value: &notification_settings_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "notificationSettings".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TrustAnchorResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            notification_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationSettings").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
