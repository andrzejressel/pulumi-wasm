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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Set of nested attribute definitions. Only required for `hash_key` and `range_key` attributes. See below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableAttribute>>,
        >,
        /// Controls how you are charged for read and write throughput and how you manage capacity. The valid values are `PROVISIONED` and `PAY_PER_REQUEST`. Defaults to `PROVISIONED`.
        #[builder(into, default)]
        pub billing_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Enables deletion protection for table. Defaults to `false`.
        #[builder(into, default)]
        pub deletion_protection_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Describe a GSI for the table; subject to the normal limits on the number of GSIs, projected attributes, etc. See below.
        #[builder(into, default)]
        pub global_secondary_indexes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableGlobalSecondaryIndex>>,
        >,
        /// Attribute to use as the hash (partition) key. Must also be defined as an `attribute`. See below.
        #[builder(into, default)]
        pub hash_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Import Amazon S3 data into a new table. See below.
        #[builder(into, default)]
        pub import_table: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableImportTable>,
        >,
        /// Describe an LSI on the table; these can only be allocated _at creation_ so you cannot change this definition after you have created the resource. See below.
        #[builder(into, default)]
        pub local_secondary_indexes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableLocalSecondaryIndex>>,
        >,
        /// Unique within a region name of the table.
        ///
        /// Optional arguments:
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets the maximum number of read and write units for the specified on-demand table. See below.
        #[builder(into, default)]
        pub on_demand_throughput: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableOnDemandThroughput>,
        >,
        /// Enable point-in-time recovery options. See below.
        #[builder(into, default)]
        pub point_in_time_recovery: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TablePointInTimeRecovery>,
        >,
        /// Attribute to use as the range (sort) key. Must also be defined as an `attribute`, see below.
        #[builder(into, default)]
        pub range_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Number of read units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        #[builder(into, default)]
        pub read_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Configuration block(s) with [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html) replication configurations. See below.
        #[builder(into, default)]
        pub replicas: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::dynamodb::TableReplica>>,
        >,
        /// Time of the point-in-time recovery point to restore.
        #[builder(into, default)]
        pub restore_date_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Name of the table to restore. Must match the name of an existing table.
        #[builder(into, default)]
        pub restore_source_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// ARN of the source table to restore. Must be supplied for cross-region restores.
        #[builder(into, default)]
        pub restore_source_table_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If set, restores table to the most recent point-in-time recovery point.
        #[builder(into, default)]
        pub restore_to_latest_time: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Encryption at rest options. AWS DynamoDB tables are automatically encrypted at rest with an AWS-owned Customer Master Key if this argument isn't specified. Must be supplied for cross-region restores. See below.
        #[builder(into, default)]
        pub server_side_encryption: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableServerSideEncryption>,
        >,
        /// Whether Streams are enabled.
        #[builder(into, default)]
        pub stream_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// When an item in the table is modified, StreamViewType determines what information is written to the table's stream. Valid values are `KEYS_ONLY`, `NEW_IMAGE`, `OLD_IMAGE`, `NEW_AND_OLD_IMAGES`.
        #[builder(into, default)]
        pub stream_view_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Storage class of the table.
        /// Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`.
        /// Default value is `STANDARD`.
        #[builder(into, default)]
        pub table_class: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Configuration block for TTL. See below.
        #[builder(into, default)]
        pub ttl: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::dynamodb::TableTtl>,
        >,
        /// Number of write units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        #[builder(into, default)]
        pub write_capacity: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// ARN of the table
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Set of nested attribute definitions. Only required for `hash_key` and `range_key` attributes. See below.
        pub attributes: pulumi_wasm_rust::Output<
            Vec<super::super::types::dynamodb::TableAttribute>,
        >,
        /// Controls how you are charged for read and write throughput and how you manage capacity. The valid values are `PROVISIONED` and `PAY_PER_REQUEST`. Defaults to `PROVISIONED`.
        pub billing_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// Enables deletion protection for table. Defaults to `false`.
        pub deletion_protection_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Describe a GSI for the table; subject to the normal limits on the number of GSIs, projected attributes, etc. See below.
        pub global_secondary_indexes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableGlobalSecondaryIndex>>,
        >,
        /// Attribute to use as the hash (partition) key. Must also be defined as an `attribute`. See below.
        pub hash_key: pulumi_wasm_rust::Output<String>,
        /// Import Amazon S3 data into a new table. See below.
        pub import_table: pulumi_wasm_rust::Output<
            Option<super::super::types::dynamodb::TableImportTable>,
        >,
        /// Describe an LSI on the table; these can only be allocated _at creation_ so you cannot change this definition after you have created the resource. See below.
        pub local_secondary_indexes: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableLocalSecondaryIndex>>,
        >,
        /// Unique within a region name of the table.
        ///
        /// Optional arguments:
        pub name: pulumi_wasm_rust::Output<String>,
        /// Sets the maximum number of read and write units for the specified on-demand table. See below.
        pub on_demand_throughput: pulumi_wasm_rust::Output<
            Option<super::super::types::dynamodb::TableOnDemandThroughput>,
        >,
        /// Enable point-in-time recovery options. See below.
        pub point_in_time_recovery: pulumi_wasm_rust::Output<
            super::super::types::dynamodb::TablePointInTimeRecovery,
        >,
        /// Attribute to use as the range (sort) key. Must also be defined as an `attribute`, see below.
        pub range_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of read units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        pub read_capacity: pulumi_wasm_rust::Output<i32>,
        /// Configuration block(s) with [DynamoDB Global Tables V2 (version 2019.11.21)](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html) replication configurations. See below.
        pub replicas: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::dynamodb::TableReplica>>,
        >,
        /// Time of the point-in-time recovery point to restore.
        pub restore_date_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the table to restore. Must match the name of an existing table.
        pub restore_source_name: pulumi_wasm_rust::Output<Option<String>>,
        /// ARN of the source table to restore. Must be supplied for cross-region restores.
        pub restore_source_table_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// If set, restores table to the most recent point-in-time recovery point.
        pub restore_to_latest_time: pulumi_wasm_rust::Output<Option<bool>>,
        /// Encryption at rest options. AWS DynamoDB tables are automatically encrypted at rest with an AWS-owned Customer Master Key if this argument isn't specified. Must be supplied for cross-region restores. See below.
        pub server_side_encryption: pulumi_wasm_rust::Output<
            super::super::types::dynamodb::TableServerSideEncryption,
        >,
        /// ARN of the Table Stream. Only available when `stream_enabled = true`
        pub stream_arn: pulumi_wasm_rust::Output<String>,
        /// Whether Streams are enabled.
        pub stream_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Timestamp, in ISO 8601 format, for this stream. Note that this timestamp is not a unique identifier for the stream on its own. However, the combination of AWS customer ID, table name and this field is guaranteed to be unique. It can be used for creating CloudWatch Alarms. Only available when `stream_enabled = true`.
        pub stream_label: pulumi_wasm_rust::Output<String>,
        /// When an item in the table is modified, StreamViewType determines what information is written to the table's stream. Valid values are `KEYS_ONLY`, `NEW_IMAGE`, `OLD_IMAGE`, `NEW_AND_OLD_IMAGES`.
        pub stream_view_type: pulumi_wasm_rust::Output<String>,
        /// Storage class of the table.
        /// Valid values are `STANDARD` and `STANDARD_INFREQUENT_ACCESS`.
        /// Default value is `STANDARD`.
        pub table_class: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to populate on the created table. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Configuration block for TTL. See below.
        pub ttl: pulumi_wasm_rust::Output<super::super::types::dynamodb::TableTtl>,
        /// Number of write units for this table. If the `billing_mode` is `PROVISIONED`, this field is required.
        pub write_capacity: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_output(context).get_inner();
        let billing_mode_binding = args.billing_mode.get_output(context).get_inner();
        let deletion_protection_enabled_binding = args
            .deletion_protection_enabled
            .get_output(context)
            .get_inner();
        let global_secondary_indexes_binding = args
            .global_secondary_indexes
            .get_output(context)
            .get_inner();
        let hash_key_binding = args.hash_key.get_output(context).get_inner();
        let import_table_binding = args.import_table.get_output(context).get_inner();
        let local_secondary_indexes_binding = args
            .local_secondary_indexes
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let on_demand_throughput_binding = args
            .on_demand_throughput
            .get_output(context)
            .get_inner();
        let point_in_time_recovery_binding = args
            .point_in_time_recovery
            .get_output(context)
            .get_inner();
        let range_key_binding = args.range_key.get_output(context).get_inner();
        let read_capacity_binding = args.read_capacity.get_output(context).get_inner();
        let replicas_binding = args.replicas.get_output(context).get_inner();
        let restore_date_time_binding = args
            .restore_date_time
            .get_output(context)
            .get_inner();
        let restore_source_name_binding = args
            .restore_source_name
            .get_output(context)
            .get_inner();
        let restore_source_table_arn_binding = args
            .restore_source_table_arn
            .get_output(context)
            .get_inner();
        let restore_to_latest_time_binding = args
            .restore_to_latest_time
            .get_output(context)
            .get_inner();
        let server_side_encryption_binding = args
            .server_side_encryption
            .get_output(context)
            .get_inner();
        let stream_enabled_binding = args.stream_enabled.get_output(context).get_inner();
        let stream_view_type_binding = args
            .stream_view_type
            .get_output(context)
            .get_inner();
        let table_class_binding = args.table_class.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let ttl_binding = args.ttl.get_output(context).get_inner();
        let write_capacity_binding = args.write_capacity.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dynamodb/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "billingMode".into(),
                    value: &billing_mode_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtectionEnabled".into(),
                    value: &deletion_protection_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "globalSecondaryIndexes".into(),
                    value: &global_secondary_indexes_binding,
                },
                register_interface::ObjectField {
                    name: "hashKey".into(),
                    value: &hash_key_binding,
                },
                register_interface::ObjectField {
                    name: "importTable".into(),
                    value: &import_table_binding,
                },
                register_interface::ObjectField {
                    name: "localSecondaryIndexes".into(),
                    value: &local_secondary_indexes_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "onDemandThroughput".into(),
                    value: &on_demand_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "pointInTimeRecovery".into(),
                    value: &point_in_time_recovery_binding,
                },
                register_interface::ObjectField {
                    name: "rangeKey".into(),
                    value: &range_key_binding,
                },
                register_interface::ObjectField {
                    name: "readCapacity".into(),
                    value: &read_capacity_binding,
                },
                register_interface::ObjectField {
                    name: "replicas".into(),
                    value: &replicas_binding,
                },
                register_interface::ObjectField {
                    name: "restoreDateTime".into(),
                    value: &restore_date_time_binding,
                },
                register_interface::ObjectField {
                    name: "restoreSourceName".into(),
                    value: &restore_source_name_binding,
                },
                register_interface::ObjectField {
                    name: "restoreSourceTableArn".into(),
                    value: &restore_source_table_arn_binding,
                },
                register_interface::ObjectField {
                    name: "restoreToLatestTime".into(),
                    value: &restore_to_latest_time_binding,
                },
                register_interface::ObjectField {
                    name: "serverSideEncryption".into(),
                    value: &server_side_encryption_binding,
                },
                register_interface::ObjectField {
                    name: "streamEnabled".into(),
                    value: &stream_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "streamViewType".into(),
                    value: &stream_view_type_binding,
                },
                register_interface::ObjectField {
                    name: "tableClass".into(),
                    value: &table_class_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "ttl".into(),
                    value: &ttl_binding,
                },
                register_interface::ObjectField {
                    name: "writeCapacity".into(),
                    value: &write_capacity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "billingMode".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtectionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "globalSecondaryIndexes".into(),
                },
                register_interface::ResultField {
                    name: "hashKey".into(),
                },
                register_interface::ResultField {
                    name: "importTable".into(),
                },
                register_interface::ResultField {
                    name: "localSecondaryIndexes".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "onDemandThroughput".into(),
                },
                register_interface::ResultField {
                    name: "pointInTimeRecovery".into(),
                },
                register_interface::ResultField {
                    name: "rangeKey".into(),
                },
                register_interface::ResultField {
                    name: "readCapacity".into(),
                },
                register_interface::ResultField {
                    name: "replicas".into(),
                },
                register_interface::ResultField {
                    name: "restoreDateTime".into(),
                },
                register_interface::ResultField {
                    name: "restoreSourceName".into(),
                },
                register_interface::ResultField {
                    name: "restoreSourceTableArn".into(),
                },
                register_interface::ResultField {
                    name: "restoreToLatestTime".into(),
                },
                register_interface::ResultField {
                    name: "serverSideEncryption".into(),
                },
                register_interface::ResultField {
                    name: "streamArn".into(),
                },
                register_interface::ResultField {
                    name: "streamEnabled".into(),
                },
                register_interface::ResultField {
                    name: "streamLabel".into(),
                },
                register_interface::ResultField {
                    name: "streamViewType".into(),
                },
                register_interface::ResultField {
                    name: "tableClass".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "ttl".into(),
                },
                register_interface::ResultField {
                    name: "writeCapacity".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            billing_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("billingMode").unwrap(),
            ),
            deletion_protection_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtectionEnabled").unwrap(),
            ),
            global_secondary_indexes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("globalSecondaryIndexes").unwrap(),
            ),
            hash_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hashKey").unwrap(),
            ),
            import_table: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("importTable").unwrap(),
            ),
            local_secondary_indexes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localSecondaryIndexes").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            on_demand_throughput: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onDemandThroughput").unwrap(),
            ),
            point_in_time_recovery: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pointInTimeRecovery").unwrap(),
            ),
            range_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangeKey").unwrap(),
            ),
            read_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("readCapacity").unwrap(),
            ),
            replicas: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("replicas").unwrap(),
            ),
            restore_date_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreDateTime").unwrap(),
            ),
            restore_source_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreSourceName").unwrap(),
            ),
            restore_source_table_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreSourceTableArn").unwrap(),
            ),
            restore_to_latest_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restoreToLatestTime").unwrap(),
            ),
            server_side_encryption: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serverSideEncryption").unwrap(),
            ),
            stream_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamArn").unwrap(),
            ),
            stream_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamEnabled").unwrap(),
            ),
            stream_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamLabel").unwrap(),
            ),
            stream_view_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamViewType").unwrap(),
            ),
            table_class: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableClass").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            ttl: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ttl").unwrap(),
            ),
            write_capacity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writeCapacity").unwrap(),
            ),
        }
    }
}
