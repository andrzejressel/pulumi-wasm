/// Provides a DynamoDB table resource.
///
/// > **Note:** It is recommended to use [`ignoreChanges`](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) for `read_capacity` and/or `write_capacity` if there's `autoscaling policy` attached to the table.
///
/// > **Note:** When using aws.dynamodb.TableReplica with this resource, use `lifecycle` `ignore_changes` for `replica`, _e.g._, `lifecycle { ignore_changes = [replica] }`.
///
/// ## DynamoDB Table attributes
///
/// Only define attributes on the table object that are going to be used as:
///
/// * Table hash key or range key
/// * LSI or GSI hash key or range key
///
/// The DynamoDB API expects attribute structure (name and type) to be passed along when creating or updating GSI/LSIs or creating the initial table. In these cases it expects the Hash / Range keys to be provided. Because these get re-used in numerous places (i.e the table's range key could be a part of one or more GSIs), they are stored on the table object to prevent duplication and increase consistency. If you add attributes here that are not used in these scenarios it can cause an infinite loop in planning.
///
/// ## Example Usage
///
/// ### Basic Example
///
/// The following dynamodb table description models the table and GSI shown in the [AWS SDK example documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/GSI.html)
///
/// ```yaml
/// resources:
///   basic-dynamodb-table:
///     type: aws:dynamodb:Table
///     properties:
///       name: GameScores
///       billingMode: PROVISIONED
///       readCapacity: 20
///       writeCapacity: 20
///       hashKey: UserId
///       rangeKey: GameTitle
///       attributes:
///         - name: UserId
///           type: S
///         - name: GameTitle
///           type: S
///         - name: TopScore
///           type: N
///       ttl:
///         attributeName: TimeToExist
///         enabled: true
///       globalSecondaryIndexes:
///         - name: GameTitleIndex
///           hashKey: GameTitle
///           rangeKey: TopScore
///           writeCapacity: 10
///           readCapacity: 10
///           projectionType: INCLUDE
///           nonKeyAttributes:
///             - UserId
///       tags:
///         Name: dynamodb-table-1
///         Environment: production
/// ```
///
/// ### Global Tables
///
/// This resource implements support for [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html) via `replica` configuration blocks. For working with [DynamoDB Global Tables V1 (version 2017.11.29)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V1.html), see the `aws.dynamodb.GlobalTable` resource.
///
/// > **Note:** aws.dynamodb.TableReplica is an alternate way of configuring Global Tables. Do not use `replica` configuration blocks of `aws.dynamodb.Table` together with aws_dynamodb_table_replica.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = table::create(
///         "example",
///         TableArgs::builder()
///             .attributes(
///                 vec![
///                     TableAttribute::builder().name("TestTableHashKey"). type ("S")
///                     .build_struct(),
///                 ],
///             )
///             .billing_mode("PAY_PER_REQUEST")
///             .hash_key("TestTableHashKey")
///             .name("example")
///             .replicas(
///                 vec![
///                     TableReplica::builder().regionName("us-east-2").build_struct(),
///                     TableReplica::builder().regionName("us-west-2").build_struct(),
///                 ],
///             )
///             .stream_enabled(true)
///             .stream_view_type("NEW_AND_OLD_IMAGES")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Replica Tagging
///
/// You can manage global table replicas' tags in various ways. This example shows using `replica.*.propagate_tags` for the first replica and the `aws.dynamodb.Tag` resource for the other.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:dynamodb:Table
///     properties:
///       billingMode: PAY_PER_REQUEST
///       hashKey: TestTableHashKey
///       name: example-13281
///       streamEnabled: true
///       streamViewType: NEW_AND_OLD_IMAGES
///       attributes:
///         - name: TestTableHashKey
///           type: S
///       replicas:
///         - regionName: ${alternate.name}
///         - regionName: ${third.name}
///           propagateTags: true
///       tags:
///         Architect: Eleanor
///         Zone: SW
///   exampleTag:
///     type: aws:dynamodb:Tag
///     name: example
///     properties:
///       resourceArn:
///         fn::invoke:
///           function: std:replace
///           arguments:
///             text: ${example.arn}
///             search: ${current.name}
///             replace: ${alternate.name}
///           return: result
///       key: Architect
///       value: Gigi
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   alternate:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   third:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DynamoDB tables using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:dynamodb/table:Table basic-dynamodb-table GameScores
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Set of nested attribute definitions. Only required for `hash_key` and `range_key` attributes. See below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableAttribute>>,
        >,
        /// Controls how you are charged for read and write throughput and how you manage capacity. The valid values are `PROVISIONED` and `PAY_PER_REQUEST`. Defaults to `PROVISIONED`.
        #[builder(into, default)]
        pub billing_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables deletion protection for table. Defaults to `false`.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Describe a GSI for the table; subject to the normal limits on the number of GSIs, projected attributes, etc. See below.
        #[builder(into, default)]
        pub global_secondary_indexes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableGlobalSecondaryIndex>>,
        >,
        /// Attribute to use as the hash (partition) key. Must also be defined as an `attribute`. See below.
        #[builder(into, default)]
        pub hash_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Import Amazon S3 data into a new table. See below.
        #[builder(into, default)]
        pub import_table: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableImportTable>,
        >,
        /// Describe an LSI on the table; these can only be allocated _at creation_ so you cannot change this definition after you have created the resource. See below.
        #[builder(into, default)]
        pub local_secondary_indexes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableLocalSecondaryIndex>>,
        >,
        /// Unique within a region name of the table.
        ///
        /// Optional arguments:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets the maximum number of read and write units for the specified on-demand table. See below.
        #[builder(into, default)]
        pub on_demand_throughput: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableOnDemandThroughput>,
        >,
        /// Enable point-in-time recovery options. See below.
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TablePointInTimeRecovery>,
        >,
        /// Attribute to use as the range (sort) key. Must also be defined as an `attribute`, see below.
        #[builder(into, default)]
        pub range_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Number of read units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        #[builder(into, default)]
        pub read_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Configuration block(s) with [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html) replication configurations. See below.
        #[builder(into, default)]
        pub replicas: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableReplica>>,
        >,
        /// Time of the point-in-time recovery point to restore.
        #[builder(into, default)]
        pub restore_date_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the table to restore. Must match the name of an existing table.
        #[builder(into, default)]
        pub restore_source_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the source table to restore. Must be supplied for cross-region restores.
        #[builder(into, default)]
        pub restore_source_table_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set, restores table to the most recent point-in-time recovery point.
        #[builder(into, default)]
        pub restore_to_latest_time: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Encryption at rest options. AWS DynamoDB tables are automatically encrypted at rest with an AWS-owned Customer Master Key if this argument isn't specified. Must be supplied for cross-region restores. See below.
        #[builder(into, default)]
        pub server_side_encryption: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableServerSideEncryption>,
        >,
        /// Whether Streams are enabled.
        #[builder(into, default)]
        pub stream_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// When an item in the table is modified, StreamViewType determines what information is written to the table's stream. Valid values are `KEYS_ONLY`, `NEW_IMAGE`, `OLD_IMAGE`, `NEW_AND_OLD_IMAGES`.
        #[builder(into, default)]
        pub stream_view_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Storage class of the table.
        /// Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`.
        /// Default value is `STANDARD`.
        #[builder(into, default)]
        pub table_class: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for TTL. See below.
        #[builder(into, default)]
        pub ttl: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableTtl>,
        >,
        /// Number of write units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        #[builder(into, default)]
        pub write_capacity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// ARN of the table
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of nested attribute definitions. Only required for `hash_key` and `range_key` attributes. See below.
        pub attributes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::dynamodb::TableAttribute>,
        >,
        /// Controls how you are charged for read and write throughput and how you manage capacity. The valid values are `PROVISIONED` and `PAY_PER_REQUEST`. Defaults to `PROVISIONED`.
        pub billing_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// Enables deletion protection for table. Defaults to `false`.
        pub deletion_protection_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Describe a GSI for the table; subject to the normal limits on the number of GSIs, projected attributes, etc. See below.
        pub global_secondary_indexes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableGlobalSecondaryIndex>>,
        >,
        /// Attribute to use as the hash (partition) key. Must also be defined as an `attribute`. See below.
        pub hash_key: pulumi_gestalt_rust::Output<String>,
        /// Import Amazon S3 data into a new table. See below.
        pub import_table: pulumi_gestalt_rust::Output<
            Option<super::super::types::dynamodb::TableImportTable>,
        >,
        /// Describe an LSI on the table; these can only be allocated _at creation_ so you cannot change this definition after you have created the resource. See below.
        pub local_secondary_indexes: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableLocalSecondaryIndex>>,
        >,
        /// Unique within a region name of the table.
        ///
        /// Optional arguments:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Sets the maximum number of read and write units for the specified on-demand table. See below.
        pub on_demand_throughput: pulumi_gestalt_rust::Output<
            Option<super::super::types::dynamodb::TableOnDemandThroughput>,
        >,
        /// Enable point-in-time recovery options. See below.
        pub point_in_time_recovery: pulumi_gestalt_rust::Output<
            super::super::types::dynamodb::TablePointInTimeRecovery,
        >,
        /// Attribute to use as the range (sort) key. Must also be defined as an `attribute`, see below.
        pub range_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Number of read units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        pub read_capacity: pulumi_gestalt_rust::Output<i32>,
        /// Configuration block(s) with [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html) replication configurations. See below.
        pub replicas: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableReplica>>,
        >,
        /// Time of the point-in-time recovery point to restore.
        pub restore_date_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// Name of the table to restore. Must match the name of an existing table.
        pub restore_source_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the source table to restore. Must be supplied for cross-region restores.
        pub restore_source_table_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set, restores table to the most recent point-in-time recovery point.
        pub restore_to_latest_time: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Encryption at rest options. AWS DynamoDB tables are automatically encrypted at rest with an AWS-owned Customer Master Key if this argument isn't specified. Must be supplied for cross-region restores. See below.
        pub server_side_encryption: pulumi_gestalt_rust::Output<
            super::super::types::dynamodb::TableServerSideEncryption,
        >,
        /// ARN of the Table Stream. Only available when `stream_enabled = true`
        pub stream_arn: pulumi_gestalt_rust::Output<String>,
        /// Whether Streams are enabled.
        pub stream_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Timestamp, in ISO 8601 format, for this stream. Note that this timestamp is not a unique identifier for the stream on its own. However, the combination of AWS customer ID, table name and this field is guaranteed to be unique. It can be used for creating CloudWatch Alarms. Only available when `stream_enabled = true`.
        pub stream_label: pulumi_gestalt_rust::Output<String>,
        /// When an item in the table is modified, StreamViewType determines what information is written to the table's stream. Valid values are `KEYS_ONLY`, `NEW_IMAGE`, `OLD_IMAGE`, `NEW_AND_OLD_IMAGES`.
        pub stream_view_type: pulumi_gestalt_rust::Output<String>,
        /// Storage class of the table.
        /// Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`.
        /// Default value is `STANDARD`.
        pub table_class: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for TTL. See below.
        pub ttl: pulumi_gestalt_rust::Output<super::super::types::dynamodb::TableTtl>,
        /// Number of write units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        pub write_capacity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attributes_binding = args.attributes.get_output(context);
        let billing_mode_binding = args.billing_mode.get_output(context);
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context);
        let global_secondary_indexes_binding = args
            .global_secondary_indexes
            .get_output(context);
        let hash_key_binding = args.hash_key.get_output(context);
        let import_table_binding = args.import_table.get_output(context);
        let local_secondary_indexes_binding = args
            .local_secondary_indexes
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let on_demand_throughput_binding = args.on_demand_throughput.get_output(context);
        let point_in_time_recovery_binding = args
            .point_in_time_recovery
            .get_output(context);
        let range_key_binding = args.range_key.get_output(context);
        let read_capacity_binding = args.read_capacity.get_output(context);
        let replicas_binding = args.replicas.get_output(context);
        let restore_date_time_binding = args.restore_date_time.get_output(context);
        let restore_source_name_binding = args.restore_source_name.get_output(context);
        let restore_source_table_arn_binding = args
            .restore_source_table_arn
            .get_output(context);
        let restore_to_latest_time_binding = args
            .restore_to_latest_time
            .get_output(context);
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context);
        let stream_enabled_binding = args.stream_enabled.get_output(context);
        let stream_view_type_binding = args.stream_view_type.get_output(context);
        let table_class_binding = args.table_class.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let ttl_binding = args.ttl.get_output(context);
        let write_capacity_binding = args.write_capacity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:dynamodb/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: attributes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "billingMode".into(),
                    value: billing_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: deletion_protection_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalSecondaryIndexes".into(),
                    value: global_secondary_indexes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hashKey".into(),
                    value: hash_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "importTable".into(),
                    value: import_table_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "localSecondaryIndexes".into(),
                    value: local_secondary_indexes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "onDemandThroughput".into(),
                    value: on_demand_throughput_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: point_in_time_recovery_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rangeKey".into(),
                    value: range_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "readCapacity".into(),
                    value: read_capacity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "replicas".into(),
                    value: replicas_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreDateTime".into(),
                    value: restore_date_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreSourceName".into(),
                    value: restore_source_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreSourceTableArn".into(),
                    value: restore_source_table_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restoreToLatestTime".into(),
                    value: restore_to_latest_time_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serverSideEncryption".into(),
                    value: server_side_encryption_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamEnabled".into(),
                    value: stream_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamViewType".into(),
                    value: stream_view_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tableClass".into(),
                    value: table_class_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ttl".into(),
                    value: ttl_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "writeCapacity".into(),
                    value: write_capacity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TableResult {
            arn: o.get_field("arn"),
            attributes: o.get_field("attributes"),
            billing_mode: o.get_field("billingMode"),
            deletion_protection_enabled: o.get_field("deletionProtectionEnabled"),
            global_secondary_indexes: o.get_field("globalSecondaryIndexes"),
            hash_key: o.get_field("hashKey"),
            import_table: o.get_field("importTable"),
            local_secondary_indexes: o.get_field("localSecondaryIndexes"),
            name: o.get_field("name"),
            on_demand_throughput: o.get_field("onDemandThroughput"),
            point_in_time_recovery: o.get_field("pointInTimeRecovery"),
            range_key: o.get_field("rangeKey"),
            read_capacity: o.get_field("readCapacity"),
            replicas: o.get_field("replicas"),
            restore_date_time: o.get_field("restoreDateTime"),
            restore_source_name: o.get_field("restoreSourceName"),
            restore_source_table_arn: o.get_field("restoreSourceTableArn"),
            restore_to_latest_time: o.get_field("restoreToLatestTime"),
            server_side_encryption: o.get_field("serverSideEncryption"),
            stream_arn: o.get_field("streamArn"),
            stream_enabled: o.get_field("streamEnabled"),
            stream_label: o.get_field("streamLabel"),
            stream_view_type: o.get_field("streamViewType"),
            table_class: o.get_field("tableClass"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            ttl: o.get_field("ttl"),
            write_capacity: o.get_field("writeCapacity"),
        }
    }
}
