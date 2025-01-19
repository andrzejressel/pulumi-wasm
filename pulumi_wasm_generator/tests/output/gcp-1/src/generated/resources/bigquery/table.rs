/// Creates a table resource in a dataset for Google BigQuery. For more information see
/// [the official documentation](https://cloud.google.com/bigquery/docs/) and
/// [API](https://cloud.google.com/bigquery/docs/reference/rest/v2/tables).
///
/// > **Note**: On newer versions of the provider, you must explicitly set `deletion_protection=false`
/// (and run `pulumi update` to write the field to state) in order to destroy an instance.
/// It is recommended to not set this field (or set it to true) until you're ready to destroy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: foo
///       friendlyName: test
///       description: This is a test description
///       location: EU
///       defaultTableExpirationMs: 3.6e+06
///       labels:
///         env: default
///   defaultTable:
///     type: gcp:bigquery:Table
///     name: default
///     properties:
///       datasetId: ${default.datasetId}
///       tableId: bar
///       timePartitioning:
///         type: DAY
///       labels:
///         env: default
///       schema: |
///         [
///           {
///             "name": "permalink",
///             "type": "STRING",
///             "mode": "NULLABLE",
///             "description": "The Permalink"
///           },
///           {
///             "name": "state",
///             "type": "STRING",
///             "mode": "NULLABLE",
///             "description": "State where the head office is located"
///           }
///         ]
///   sheet:
///     type: gcp:bigquery:Table
///     properties:
///       datasetId: ${default.datasetId}
///       tableId: sheet
///       externalDataConfiguration:
///         autodetect: true
///         sourceFormat: GOOGLE_SHEETS
///         googleSheetsOptions:
///           skipLeadingRows: 1
///         sourceUris:
///           - https://docs.google.com/spreadsheets/d/123456789012345
/// ```
///
/// ## Import
///
/// BigQuery tables can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}`
///
/// * `{{project}}/{{dataset_id}}/{{table_id}}`
///
/// * `{{dataset_id}}/{{table_id}}`
///
/// When using the `pulumi import` command, BigQuery tables can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:bigquery/table:Table default projects/{{project}}/datasets/{{dataset_id}}/tables/{{table_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/table:Table default {{project}}/{{dataset_id}}/{{table_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:bigquery/table:Table default {{dataset_id}}/{{table_id}}
/// ```
///
pub mod table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Specifies the configuration of a BigLake managed table. Structure is documented below
        #[builder(into, default)]
        pub biglake_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableBiglakeConfiguration>,
        >,
        /// Specifies column names to use for data clustering.
        /// Up to four top-level columns are allowed, and should be specified in
        /// descending priority order.
        #[builder(into, default)]
        pub clusterings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The dataset ID to create the table in.
        /// Changing this forces a new resource to be created.
        #[builder(into)]
        pub dataset_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `=destroy` or `=update` that would delete the instance will fail.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The field description.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how the table should be encrypted.
        /// If left blank, the table will be encrypted with a Google-managed key; that process
        /// is transparent to the user.  Structure is documented below.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableEncryptionConfiguration>,
        >,
        /// The time when this table expires, in
        /// milliseconds since the epoch. If not present, the table will persist
        /// indefinitely. Expired tables will be deleted and their storage
        /// reclaimed.
        #[builder(into, default)]
        pub expiration_time: pulumi_wasm_rust::Output<Option<i32>>,
        /// Describes the data format,
        /// location, and other properties of a table stored outside of BigQuery.
        /// By defining these properties, the data source can then be queried as
        /// if it were a standard BigQuery table. Structure is documented below.
        #[builder(into, default)]
        pub external_data_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableExternalDataConfiguration>,
        >,
        /// A descriptive name for the table.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of labels to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If specified, configures this table as a materialized view.
        /// Structure is documented below.
        #[builder(into, default)]
        pub materialized_view: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableMaterializedView>,
        >,
        /// The maximum staleness of data that could be
        /// returned when the table (or stale MV) is queried. Staleness encoded as a
        /// string encoding of [SQL IntervalValue
        /// type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type).
        #[builder(into, default)]
        pub max_staleness: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// If specified, configures range-based
        /// partitioning for this table. Structure is documented below.
        #[builder(into, default)]
        pub range_partitioning: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableRangePartitioning>,
        >,
        /// If set to true, queries over this table
        /// require a partition filter that can be used for partition elimination to be
        /// specified.
        #[builder(into, default)]
        pub require_partition_filter: pulumi_wasm_rust::Output<Option<bool>>,
        /// The tags attached to this table. Tag keys are
        /// globally unique. Tag key is expected to be in the namespaced format, for
        /// example "123456789012/environment" where 123456789012 is the ID of the
        /// parent organization or project resource for this tag key. Tag value is
        /// expected to be the short name, for example "Production".
        #[builder(into, default)]
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON schema for the table.
        #[builder(into, default)]
        pub schema: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the primary key and foreign keys.
        /// Structure is documented below.
        #[builder(into, default)]
        pub table_constraints: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTableConstraints>,
        >,
        /// A unique ID for the resource.
        /// Changing this forces a new resource to be created.
        #[builder(into)]
        pub table_id: pulumi_wasm_rust::Output<String>,
        /// Replication info of a table created
        /// using "AS REPLICA" DDL like:
        /// `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`.
        /// Structure is documented below.
        #[builder(into, default)]
        pub table_replication_info: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTableReplicationInfo>,
        >,
        /// If specified, configures time-based
        /// partitioning for this table. Structure is documented below.
        #[builder(into, default)]
        pub time_partitioning: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTimePartitioning>,
        >,
        /// If specified, configures this table as a view.
        /// Structure is documented below.
        #[builder(into, default)]
        pub view: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableView>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// Specifies the configuration of a BigLake managed table. Structure is documented below
        pub biglake_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableBiglakeConfiguration>,
        >,
        /// Specifies column names to use for data clustering.
        /// Up to four top-level columns are allowed, and should be specified in
        /// descending priority order.
        pub clusterings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The time when this table was created, in milliseconds since the epoch.
        pub creation_time: pulumi_wasm_rust::Output<i32>,
        /// The dataset ID to create the table in.
        /// Changing this forces a new resource to be created.
        pub dataset_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `=destroy` or `=update` that would delete the instance will fail.
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The field description.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        ///
        /// * <a name="schema"></a>`schema` - (Optional) A JSON schema for the table.
        ///
        /// ~>**NOTE:** Because this field expects a JSON string, any changes to the
        /// string will create a diff, even if the JSON itself hasn't changed.
        /// If the API returns a different value for the same schema, e.g. it
        /// switched the order of values or replaced `STRUCT` field type with `RECORD`
        /// field type, we currently cannot suppress the recurring diff this causes.
        /// As a workaround, we recommend using the schema as returned by the API.
        ///
        /// ~>**NOTE:**  If you use `external_data_configuration`
        /// documented below and do **not** set
        /// `external_data_configuration.connection_id`, schemas must be specified
        /// with `external_data_configuration.schema`. Otherwise, schemas must be
        /// specified with this top-level field.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies how the table should be encrypted.
        /// If left blank, the table will be encrypted with a Google-managed key; that process
        /// is transparent to the user.  Structure is documented below.
        pub encryption_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableEncryptionConfiguration>,
        >,
        /// A hash of the resource.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The time when this table expires, in
        /// milliseconds since the epoch. If not present, the table will persist
        /// indefinitely. Expired tables will be deleted and their storage
        /// reclaimed.
        pub expiration_time: pulumi_wasm_rust::Output<i32>,
        /// Describes the data format,
        /// location, and other properties of a table stored outside of BigQuery.
        /// By defining these properties, the data source can then be queried as
        /// if it were a standard BigQuery table. Structure is documented below.
        pub external_data_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableExternalDataConfiguration>,
        >,
        /// A descriptive name for the table.
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of labels to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time when this table was last modified, in milliseconds since the epoch.
        pub last_modified_time: pulumi_wasm_rust::Output<i32>,
        /// The geographic location where the table resides. This value is inherited from the dataset.
        pub location: pulumi_wasm_rust::Output<String>,
        /// If specified, configures this table as a materialized view.
        /// Structure is documented below.
        pub materialized_view: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableMaterializedView>,
        >,
        /// The maximum staleness of data that could be
        /// returned when the table (or stale MV) is queried. Staleness encoded as a
        /// string encoding of [SQL IntervalValue
        /// type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type).
        pub max_staleness: pulumi_wasm_rust::Output<Option<String>>,
        /// The size of this table in bytes, excluding any data in the streaming buffer.
        pub num_bytes: pulumi_wasm_rust::Output<i32>,
        /// The number of bytes in the table that are considered "long-term storage".
        pub num_long_term_bytes: pulumi_wasm_rust::Output<i32>,
        /// The number of rows of data in this table, excluding any data in the streaming buffer.
        pub num_rows: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If specified, configures range-based
        /// partitioning for this table. Structure is documented below.
        pub range_partitioning: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableRangePartitioning>,
        >,
        /// If set to true, queries over this table
        /// require a partition filter that can be used for partition elimination to be
        /// specified.
        pub require_partition_filter: pulumi_wasm_rust::Output<Option<bool>>,
        /// The tags attached to this table. Tag keys are
        /// globally unique. Tag key is expected to be in the namespaced format, for
        /// example "123456789012/environment" where 123456789012 is the ID of the
        /// parent organization or project resource for this tag key. Tag value is
        /// expected to be the short name, for example "Production".
        pub resource_tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON schema for the table.
        pub schema: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// Defines the primary key and foreign keys.
        /// Structure is documented below.
        pub table_constraints: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTableConstraints>,
        >,
        /// A unique ID for the resource.
        /// Changing this forces a new resource to be created.
        pub table_id: pulumi_wasm_rust::Output<String>,
        /// Replication info of a table created
        /// using "AS REPLICA" DDL like:
        /// `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`.
        /// Structure is documented below.
        pub table_replication_info: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTableReplicationInfo>,
        >,
        /// If specified, configures time-based
        /// partitioning for this table. Structure is documented below.
        pub time_partitioning: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableTimePartitioning>,
        >,
        /// Describes the table type.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// If specified, configures this table as a view.
        /// Structure is documented below.
        pub view: pulumi_wasm_rust::Output<
            Option<super::super::types::bigquery::TableView>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: TableArgs) -> TableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let biglake_configuration_binding = args.biglake_configuration.get_inner();
        let clusterings_binding = args.clusterings.get_inner();
        let dataset_id_binding = args.dataset_id.get_inner();
        let deletion_protection_binding = args.deletion_protection.get_inner();
        let description_binding = args.description.get_inner();
        let encryption_configuration_binding = args.encryption_configuration.get_inner();
        let expiration_time_binding = args.expiration_time.get_inner();
        let external_data_configuration_binding = args
            .external_data_configuration
            .get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let materialized_view_binding = args.materialized_view.get_inner();
        let max_staleness_binding = args.max_staleness.get_inner();
        let project_binding = args.project.get_inner();
        let range_partitioning_binding = args.range_partitioning.get_inner();
        let require_partition_filter_binding = args.require_partition_filter.get_inner();
        let resource_tags_binding = args.resource_tags.get_inner();
        let schema_binding = args.schema.get_inner();
        let table_constraints_binding = args.table_constraints.get_inner();
        let table_id_binding = args.table_id.get_inner();
        let table_replication_info_binding = args.table_replication_info.get_inner();
        let time_partitioning_binding = args.time_partitioning.get_inner();
        let view_binding = args.view.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:bigquery/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "biglakeConfiguration".into(),
                    value: &biglake_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "clusterings".into(),
                    value: &clusterings_binding,
                },
                register_interface::ObjectField {
                    name: "datasetId".into(),
                    value: &dataset_id_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfiguration".into(),
                    value: &encryption_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "expirationTime".into(),
                    value: &expiration_time_binding,
                },
                register_interface::ObjectField {
                    name: "externalDataConfiguration".into(),
                    value: &external_data_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "materializedView".into(),
                    value: &materialized_view_binding,
                },
                register_interface::ObjectField {
                    name: "maxStaleness".into(),
                    value: &max_staleness_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "rangePartitioning".into(),
                    value: &range_partitioning_binding,
                },
                register_interface::ObjectField {
                    name: "requirePartitionFilter".into(),
                    value: &require_partition_filter_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTags".into(),
                    value: &resource_tags_binding,
                },
                register_interface::ObjectField {
                    name: "schema".into(),
                    value: &schema_binding,
                },
                register_interface::ObjectField {
                    name: "tableConstraints".into(),
                    value: &table_constraints_binding,
                },
                register_interface::ObjectField {
                    name: "tableId".into(),
                    value: &table_id_binding,
                },
                register_interface::ObjectField {
                    name: "tableReplicationInfo".into(),
                    value: &table_replication_info_binding,
                },
                register_interface::ObjectField {
                    name: "timePartitioning".into(),
                    value: &time_partitioning_binding,
                },
                register_interface::ObjectField {
                    name: "view".into(),
                    value: &view_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "biglakeConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "clusterings".into(),
                },
                register_interface::ResultField {
                    name: "creationTime".into(),
                },
                register_interface::ResultField {
                    name: "datasetId".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "encryptionConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "expirationTime".into(),
                },
                register_interface::ResultField {
                    name: "externalDataConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "lastModifiedTime".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "materializedView".into(),
                },
                register_interface::ResultField {
                    name: "maxStaleness".into(),
                },
                register_interface::ResultField {
                    name: "numBytes".into(),
                },
                register_interface::ResultField {
                    name: "numLongTermBytes".into(),
                },
                register_interface::ResultField {
                    name: "numRows".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "rangePartitioning".into(),
                },
                register_interface::ResultField {
                    name: "requirePartitionFilter".into(),
                },
                register_interface::ResultField {
                    name: "resourceTags".into(),
                },
                register_interface::ResultField {
                    name: "schema".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "tableConstraints".into(),
                },
                register_interface::ResultField {
                    name: "tableId".into(),
                },
                register_interface::ResultField {
                    name: "tableReplicationInfo".into(),
                },
                register_interface::ResultField {
                    name: "timePartitioning".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "view".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        TableResult {
            biglake_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("biglakeConfiguration").unwrap(),
            ),
            clusterings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterings").unwrap(),
            ),
            creation_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTime").unwrap(),
            ),
            dataset_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datasetId").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            encryption_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptionConfiguration").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            expiration_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expirationTime").unwrap(),
            ),
            external_data_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("externalDataConfiguration").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            last_modified_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastModifiedTime").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            materialized_view: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("materializedView").unwrap(),
            ),
            max_staleness: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxStaleness").unwrap(),
            ),
            num_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numBytes").unwrap(),
            ),
            num_long_term_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numLongTermBytes").unwrap(),
            ),
            num_rows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("numRows").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            range_partitioning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rangePartitioning").unwrap(),
            ),
            require_partition_filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("requirePartitionFilter").unwrap(),
            ),
            resource_tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTags").unwrap(),
            ),
            schema: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schema").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            table_constraints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableConstraints").unwrap(),
            ),
            table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableId").unwrap(),
            ),
            table_replication_info: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tableReplicationInfo").unwrap(),
            ),
            time_partitioning: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timePartitioning").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            view: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("view").unwrap(),
            ),
        }
    }
}
