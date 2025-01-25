/// Provides a AWS Transfer AS2 Connector resource.
///
/// ## Example Usage
///
/// ### Basic
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connector::create(
///         "example",
///         ConnectorArgs::builder()
///             .access_role("${test.arn}")
///             .as_2_config(
///                 ConnectorAs2Config::builder()
///                     .compression("DISABLED")
///                     .encryptionAlgorithm("AWS128_CBC")
///                     .localProfileId("${local.profileId}")
///                     .mdnResponse("NONE")
///                     .mdnSigningAlgorithm("NONE")
///                     .messageSubject("For Connector")
///                     .partnerProfileId("${partner.profileId}")
///                     .signingAlgorithm("NONE")
///                     .build_struct(),
///             )
///             .url("http://www.test.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### SFTP Connector
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = connector::create(
///         "example",
///         ConnectorArgs::builder()
///             .access_role("${test.arn}")
///             .sftp_config(
///                 ConnectorSftpConfig::builder()
///                     .trustedHostKeys(vec!["ssh-rsa AAAAB3NYourKeysHere",])
///                     .userSecretId("${exampleAwsSecretsmanagerSecret.id}")
///                     .build_struct(),
///             )
///             .url("sftp://test.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Transfer AS2 Connector using the `connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:transfer/connector:Connector example c-4221a88afd5f4362a
/// ```
pub mod connector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorArgs {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        #[builder(into)]
        pub access_role: pulumi_wasm_rust::InputOrOutput<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        #[builder(into, default)]
        pub as2_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::transfer::ConnectorAs2Config>,
        >,
        /// The IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
        #[builder(into, default)]
        pub logging_role: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the security policy for the connector.
        #[builder(into, default)]
        pub security_policy_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        #[builder(into, default)]
        pub sftp_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::transfer::ConnectorSftpConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The URL of the partners AS2 endpoint or SFTP endpoint.
        #[builder(into)]
        pub url: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConnectorResult {
        /// The IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
        pub access_role: pulumi_wasm_rust::Output<String>,
        /// The ARN of the connector.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        pub as2_config: pulumi_wasm_rust::Output<
            Option<super::super::types::transfer::ConnectorAs2Config>,
        >,
        /// The unique identifier for the AS2 profile or SFTP Profile.
        pub connector_id: pulumi_wasm_rust::Output<String>,
        /// The IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
        pub logging_role: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the security policy for the connector.
        pub security_policy_name: pulumi_wasm_rust::Output<String>,
        /// Either SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
        pub sftp_config: pulumi_wasm_rust::Output<
            Option<super::super::types::transfer::ConnectorSftpConfig>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The URL of the partners AS2 endpoint or SFTP endpoint.
        pub url: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ConnectorArgs,
    ) -> ConnectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_role_binding = args.access_role.get_output(context).get_inner();
        let as2_config_binding = args.as2_config.get_output(context).get_inner();
        let logging_role_binding = args.logging_role.get_output(context).get_inner();
        let security_policy_name_binding = args
            .security_policy_name
            .get_output(context)
            .get_inner();
        let sftp_config_binding = args.sftp_config.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let url_binding = args.url.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:transfer/connector:Connector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessRole".into(),
                    value: &access_role_binding,
                },
                register_interface::ObjectField {
                    name: "as2Config".into(),
                    value: &as2_config_binding,
                },
                register_interface::ObjectField {
                    name: "loggingRole".into(),
                    value: &logging_role_binding,
                },
                register_interface::ObjectField {
                    name: "securityPolicyName".into(),
                    value: &security_policy_name_binding,
                },
                register_interface::ObjectField {
                    name: "sftpConfig".into(),
                    value: &sftp_config_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "url".into(),
                    value: &url_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessRole".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "as2Config".into(),
                },
                register_interface::ResultField {
                    name: "connectorId".into(),
                },
                register_interface::ResultField {
                    name: "loggingRole".into(),
                },
                register_interface::ResultField {
                    name: "securityPolicyName".into(),
                },
                register_interface::ResultField {
                    name: "sftpConfig".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "url".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectorResult {
            access_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessRole").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            as2_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("as2Config").unwrap(),
            ),
            connector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorId").unwrap(),
            ),
            logging_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingRole").unwrap(),
            ),
            security_policy_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPolicyName").unwrap(),
            ),
            sftp_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sftpConfig").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            url: pulumi_wasm_rust::__private::into_domain(hashmap.remove("url").unwrap()),
        }
    }
}
