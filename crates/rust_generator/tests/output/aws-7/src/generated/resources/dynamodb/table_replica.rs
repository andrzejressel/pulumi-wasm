/// Provides a DynamoDB table replica resource for [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html).
///
/// > **Note:** Use `lifecycle` `ignore_changes` for `replica` in the associated aws.dynamodb.Table configuration.
///
/// > **Note:** Do not use the `replica` configuration block of aws.dynamodb.Table together with this resource as the two configuration options are mutually exclusive.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// ```yaml
/// resources:
///   example:
///     type: aws:dynamodb:Table
///     properties:
///       name: TestTable
///       hashKey: BrodoBaggins
///       billingMode: PAY_PER_REQUEST
///       streamEnabled: true
///       streamViewType: NEW_AND_OLD_IMAGES
///       attributes:
///         - name: BrodoBaggins
///           type: S
///   exampleTableReplica:
///     type: aws:dynamodb:TableReplica
///     name: example
///     properties:
///       globalTableArn: ${example.arn}
///       tags:
///         Name: IZPAWS
///         Pozo: Amargo
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB table replicas using the `table-name:main-region`. For example:
///
/// ~> __Note:__ When importing, use the region where the initial or _main_ global table resides, _not_ the region of the replica.
///
/// ```sh
/// $ pulumi import aws:dynamodb/tableReplica:TableReplica example TestTable:us-west-2
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table_replica {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableReplicaArgs {
        /// Whether deletion protection is enabled (true) or disabled (false) on the table replica.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// ARN of the _main_ or global table which this resource will replicate.
        ///
        /// Optional arguments:
        #[builder(into)]
        pub global_table_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the CMK that should be used for the AWS KMS encryption. This argument should only be used if the key is different from the default KMS-managed DynamoDB key, `alias/aws/dynamodb`. **Note:** This attribute will _not_ be populated with the ARN of _default_ keys.
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to enable Point In Time Recovery for the table replica. Default is `false`.
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Storage class of the table replica. Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`. If not used, the table replica will use the same class as the global table.
        #[builder(into, default)]
        pub table_class_override: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableReplicaResult {
        /// ARN of the table replica.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Whether deletion protection is enabled (true) or disabled (false) on the table replica.
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<bool>,
        /// ARN of the _main_ or global table which this resource will replicate.
        ///
        /// Optional arguments:
        pub global_table_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the CMK that should be used for the AWS KMS encryption. This argument should only be used if the key is different from the default KMS-managed DynamoDB key, `alias/aws/dynamodb`. **Note:** This attribute will _not_ be populated with the ARN of _default_ keys.
        pub kms_key_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable Point In Time Recovery for the table replica. Default is `false`.
        pub point_in_time_recovery: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Storage class of the table replica. Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`. If not used, the table replica will use the same class as the global table.
        pub table_class_override: pulumi_gestalt_rust::Output<Option<String>>,
        /// Map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: TableReplicaArgs,
    ) -> TableReplicaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context);
        let global_table_arn_binding = args.global_table_arn.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let point_in_time_recovery_binding = args
            .point_in_time_recovery
            .get_output(context);
        let table_class_override_binding = args.table_class_override.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/tableReplica:TableReplica".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: deletion_protection_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalTableArn".into(),
                    value: global_table_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: point_in_time_recovery_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableClassOverride".into(),
                    value: table_class_override_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableReplicaResult {
            arn: o.get_field("arn"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            global_table_arn: o.get_field("globalTableArn"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            point_in_time_recovery: o.get_field("pointInTimeRecovery"),
            table_class_override: o.get_field("tableClassOverride"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
