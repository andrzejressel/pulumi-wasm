/// Resource for managing an AWS RDS (Relational Database) zero-ETL integration. You can refer to the [User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/zero-etl.setting-up.html).
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = namespace::create(
///         "example",
///         NamespaceArgs::builder().namespace_name("redshift-example").build_struct(),
///     );
///     let exampleIntegration = integration::create(
///         "exampleIntegration",
///         IntegrationArgs::builder()
///             .integration_name("example")
///             .source_arn("${exampleAwsRdsCluster.arn}")
///             .target_arn("${example.arn}")
///             .build_struct(),
///     );
///     let exampleWorkgroup = workgroup::create(
///         "exampleWorkgroup",
///         WorkgroupArgs::builder()
///             .base_capacity(8)
///             .config_parameters(
///                 vec![
///                     WorkgroupConfigParameter::builder()
///                     .parameterKey("enable_case_sensitive_identifier")
///                     .parameterValue("true").build_struct(),
///                 ],
///             )
///             .namespace_name("${example.namespaceName}")
///             .publicly_accessible(false)
///             .subnet_ids(vec!["${example1.id}", "${example2.id}", "${example3.id}",])
///             .workgroup_name("example-workspace")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Use own KMS key
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kms:Key
///     properties:
///       deletionWindowInDays: 10
///       policy: ${keyPolicy.json}
///   exampleIntegration:
///     type: aws:rds:Integration
///     name: example
///     properties:
///       integrationName: example
///       sourceArn: ${exampleAwsRdsCluster.arn}
///       targetArn: ${exampleAwsRedshiftserverlessNamespace.arn}
///       kmsKeyId: ${example.arn}
///       additionalEncryptionContext:
///         example: test
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
///   keyPolicy:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - actions:
///               - kms:*
///             resources:
///               - '*'
///             principals:
///               - type: AWS
///                 identifiers:
///                   - arn:aws:iam::${current.accountId}:root
///           - actions:
///               - kms:CreateGrant
///             resources:
///               - '*'
///             principals:
///               - type: Service
///                 identifiers:
///                   - redshift.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS (Relational Database) Integration using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/integration:Integration example arn:aws:rds:us-west-2:123456789012:integration:abcdefgh-0000-1111-2222-123456789012
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod integration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationArgs {
        /// Set of non-secret key–value pairs that contains additional contextual information about the data. For more information, see the [User Guide](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context). You can only include this parameter if you specify the `kms_key_id` parameter.
        #[builder(into, default)]
        pub additional_encryption_context: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the integration.
        #[builder(into)]
        pub integration_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// KMS key identifier for the key to use to encrypt the integration. If you don't specify an encryption key, RDS uses a default AWS owned key. If you use the default AWS owned key, you should ignore `kms_key_id` parameter by using `lifecycle` parameter to avoid unintended change after the first creation.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the database to use as the source for replication.
        #[builder(into)]
        pub source_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of the Redshift data warehouse to use as the target for replication.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub target_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::IntegrationTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct IntegrationResult {
        /// Set of non-secret key–value pairs that contains additional contextual information about the data. For more information, see the [User Guide](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#encrypt_context). You can only include this parameter if you specify the `kms_key_id` parameter.
        pub additional_encryption_context: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ARN of the Integration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the integration.
        pub integration_name: pulumi_gestalt_rust::Output<String>,
        /// KMS key identifier for the key to use to encrypt the integration. If you don't specify an encryption key, RDS uses a default AWS owned key. If you use the default AWS owned key, you should ignore `kms_key_id` parameter by using `lifecycle` parameter to avoid unintended change after the first creation.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the database to use as the source for replication.
        pub source_arn: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// ARN of the Redshift data warehouse to use as the target for replication.
        ///
        /// The following arguments are optional:
        pub target_arn: pulumi_gestalt_rust::Output<String>,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::IntegrationTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: IntegrationArgs,
    ) -> IntegrationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let additional_encryption_context_binding = args
            .additional_encryption_context
            .get_output(context)
            .get_inner();
        let integration_name_binding = args
            .integration_name
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let source_arn_binding = args.source_arn.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_arn_binding = args.target_arn.get_output(context).get_inner();
        let timeouts_binding = args.timeouts.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/integration:Integration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "additionalEncryptionContext".into(),
                    value: &additional_encryption_context_binding,
                },
                register_interface::ObjectField {
                    name: "integrationName".into(),
                    value: &integration_name_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "sourceArn".into(),
                    value: &source_arn_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetArn".into(),
                    value: &target_arn_binding,
                },
                register_interface::ObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        IntegrationResult {
            additional_encryption_context: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("additionalEncryptionContext"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            integration_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("integrationName"),
            ),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            source_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceArn"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            target_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetArn"),
            ),
            timeouts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeouts"),
            ),
        }
    }
}
