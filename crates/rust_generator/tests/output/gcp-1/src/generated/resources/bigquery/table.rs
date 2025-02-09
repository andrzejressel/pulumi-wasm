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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// Specifies the configuration of a BigLake managed table. Structure is documented below
        #[builder(into, default)]
        pub biglake_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableBiglakeConfiguration>,
        >,
        /// Specifies column names to use for data clustering.
        /// Up to four top-level columns are allowed, and should be specified in
        /// descending priority order.
        #[builder(into, default)]
        pub clusterings: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The dataset ID to create the table in.
        /// Changing this forces a new resource to be created.
        #[builder(into)]
        pub dataset_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `=destroy` or `=update` that would delete the instance will fail.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The field description.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how the table should be encrypted.
        /// If left blank, the table will be encrypted with a Google-managed key; that process
        /// is transparent to the user.  Structure is documented below.
        #[builder(into, default)]
        pub encryption_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableEncryptionConfiguration>,
        >,
        /// The time when this table expires, in
        /// milliseconds since the epoch. If not present, the table will persist
        /// indefinitely. Expired tables will be deleted and their storage
        /// reclaimed.
        #[builder(into, default)]
        pub expiration_time: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Describes the data format,
        /// location, and other properties of a table stored outside of BigQuery.
        /// By defining these properties, the data source can then be queried as
        /// if it were a standard BigQuery table. Structure is documented below.
        #[builder(into, default)]
        pub external_data_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableExternalDataConfiguration>,
        >,
        /// A descriptive name for the table.
        #[builder(into, default)]
        pub friendly_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of labels to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// If specified, configures this table as a materialized view.
        /// Structure is documented below.
        #[builder(into, default)]
        pub materialized_view: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableMaterializedView>,
        >,
        /// The maximum staleness of data that could be
        /// returned when the table (or stale MV) is queried. Staleness encoded as a
        /// string encoding of [SQL IntervalValue
        /// type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type).
        #[builder(into, default)]
        pub max_staleness: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If specified, configures range-based
        /// partitioning for this table. Structure is documented below.
        #[builder(into, default)]
        pub range_partitioning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableRangePartitioning>,
        >,
        /// If set to true, queries over this table
        /// require a partition filter that can be used for partition elimination to be
        /// specified.
        #[builder(into, default)]
        pub require_partition_filter: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The tags attached to this table. Tag keys are
        /// globally unique. Tag key is expected to be in the namespaced format, for
        /// example "123456789012/environment" where 123456789012 is the ID of the
        /// parent organization or project resource for this tag key. Tag value is
        /// expected to be the short name, for example "Production".
        #[builder(into, default)]
        pub resource_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON schema for the table.
        #[builder(into, default)]
        pub schema: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the primary key and foreign keys.
        /// Structure is documented below.
        #[builder(into, default)]
        pub table_constraints: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableTableConstraints>,
        >,
        /// A unique ID for the resource.
        /// Changing this forces a new resource to be created.
        #[builder(into)]
        pub table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Replication info of a table created
        /// using "AS REPLICA" DDL like:
        /// `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`.
        /// Structure is documented below.
        #[builder(into, default)]
        pub table_replication_info: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableTableReplicationInfo>,
        >,
        /// If specified, configures time-based
        /// partitioning for this table. Structure is documented below.
        #[builder(into, default)]
        pub time_partitioning: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableTimePartitioning>,
        >,
        /// If specified, configures this table as a view.
        /// Structure is documented below.
        #[builder(into, default)]
        pub view: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigquery::TableView>,
        >,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// Specifies the configuration of a BigLake managed table. Structure is documented below
        pub biglake_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableBiglakeConfiguration>,
        >,
        /// Specifies column names to use for data clustering.
        /// Up to four top-level columns are allowed, and should be specified in
        /// descending priority order.
        pub clusterings: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The time when this table was created, in milliseconds since the epoch.
        pub creation_time: pulumi_gestalt_rust::Output<i32>,
        /// The dataset ID to create the table in.
        /// Changing this forces a new resource to be created.
        pub dataset_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `=destroy` or `=update` that would delete the instance will fail.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The field description.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies how the table should be encrypted.
        /// If left blank, the table will be encrypted with a Google-managed key; that process
        /// is transparent to the user.  Structure is documented below.
        pub encryption_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableEncryptionConfiguration>,
        >,
        /// A hash of the resource.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The time when this table expires, in
        /// milliseconds since the epoch. If not present, the table will persist
        /// indefinitely. Expired tables will be deleted and their storage
        /// reclaimed.
        pub expiration_time: pulumi_gestalt_rust::Output<i32>,
        /// Describes the data format,
        /// location, and other properties of a table stored outside of BigQuery.
        /// By defining these properties, the data source can then be queried as
        /// if it were a standard BigQuery table. Structure is documented below.
        pub external_data_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableExternalDataConfiguration>,
        >,
        /// A descriptive name for the table.
        pub friendly_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of labels to assign to the resource.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The time when this table was last modified, in milliseconds since the epoch.
        pub last_modified_time: pulumi_gestalt_rust::Output<i32>,
        /// The geographic location where the table resides. This value is inherited from the dataset.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// If specified, configures this table as a materialized view.
        /// Structure is documented below.
        pub materialized_view: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableMaterializedView>,
        >,
        /// The maximum staleness of data that could be
        /// returned when the table (or stale MV) is queried. Staleness encoded as a
        /// string encoding of [SQL IntervalValue
        /// type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type).
        pub max_staleness: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of this table in bytes, excluding any data in the streaming buffer.
        pub num_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The number of bytes in the table that are considered "long-term storage".
        pub num_long_term_bytes: pulumi_gestalt_rust::Output<i32>,
        /// The number of rows of data in this table, excluding any data in the streaming buffer.
        pub num_rows: pulumi_gestalt_rust::Output<i32>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// If specified, configures range-based
        /// partitioning for this table. Structure is documented below.
        pub range_partitioning: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableRangePartitioning>,
        >,
        /// If set to true, queries over this table
        /// require a partition filter that can be used for partition elimination to be
        /// specified.
        pub require_partition_filter: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The tags attached to this table. Tag keys are
        /// globally unique. Tag key is expected to be in the namespaced format, for
        /// example "123456789012/environment" where 123456789012 is the ID of the
        /// parent organization or project resource for this tag key. Tag value is
        /// expected to be the short name, for example "Production".
        pub resource_tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A JSON schema for the table.
        pub schema: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
        /// Defines the primary key and foreign keys.
        /// Structure is documented below.
        pub table_constraints: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableTableConstraints>,
        >,
        /// A unique ID for the resource.
        /// Changing this forces a new resource to be created.
        pub table_id: pulumi_gestalt_rust::Output<String>,
        /// Replication info of a table created
        /// using "AS REPLICA" DDL like:
        /// `CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv`.
        /// Structure is documented below.
        pub table_replication_info: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableTableReplicationInfo>,
        >,
        /// If specified, configures time-based
        /// partitioning for this table. Structure is documented below.
        pub time_partitioning: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableTimePartitioning>,
        >,
        /// Describes the table type.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// If specified, configures this table as a view.
        /// Structure is documented below.
        pub view: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigquery::TableView>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: TableArgs,
    ) -> TableResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let biglake_configuration_binding_1 = args
            .biglake_configuration
            .get_output(context);
        let biglake_configuration_binding = biglake_configuration_binding_1.get_inner();
        let clusterings_binding_1 = args.clusterings.get_output(context);
        let clusterings_binding = clusterings_binding_1.get_inner();
        let dataset_id_binding_1 = args.dataset_id.get_output(context);
        let dataset_id_binding = dataset_id_binding_1.get_inner();
        let deletion_protection_binding_1 = args.deletion_protection.get_output(context);
        let deletion_protection_binding = deletion_protection_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let encryption_configuration_binding_1 = args
            .encryption_configuration
            .get_output(context);
        let encryption_configuration_binding = encryption_configuration_binding_1
            .get_inner();
        let expiration_time_binding_1 = args.expiration_time.get_output(context);
        let expiration_time_binding = expiration_time_binding_1.get_inner();
        let external_data_configuration_binding_1 = args
            .external_data_configuration
            .get_output(context);
        let external_data_configuration_binding = external_data_configuration_binding_1
            .get_inner();
        let friendly_name_binding_1 = args.friendly_name.get_output(context);
        let friendly_name_binding = friendly_name_binding_1.get_inner();
        let labels_binding_1 = args.labels.get_output(context);
        let labels_binding = labels_binding_1.get_inner();
        let materialized_view_binding_1 = args.materialized_view.get_output(context);
        let materialized_view_binding = materialized_view_binding_1.get_inner();
        let max_staleness_binding_1 = args.max_staleness.get_output(context);
        let max_staleness_binding = max_staleness_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let range_partitioning_binding_1 = args.range_partitioning.get_output(context);
        let range_partitioning_binding = range_partitioning_binding_1.get_inner();
        let require_partition_filter_binding_1 = args
            .require_partition_filter
            .get_output(context);
        let require_partition_filter_binding = require_partition_filter_binding_1
            .get_inner();
        let resource_tags_binding_1 = args.resource_tags.get_output(context);
        let resource_tags_binding = resource_tags_binding_1.get_inner();
        let schema_binding_1 = args.schema.get_output(context);
        let schema_binding = schema_binding_1.get_inner();
        let table_constraints_binding_1 = args.table_constraints.get_output(context);
        let table_constraints_binding = table_constraints_binding_1.get_inner();
        let table_id_binding_1 = args.table_id.get_output(context);
        let table_id_binding = table_id_binding_1.get_inner();
        let table_replication_info_binding_1 = args
            .table_replication_info
            .get_output(context);
        let table_replication_info_binding = table_replication_info_binding_1
            .get_inner();
        let time_partitioning_binding_1 = args.time_partitioning.get_output(context);
        let time_partitioning_binding = time_partitioning_binding_1.get_inner();
        let view_binding_1 = args.view.get_output(context);
        let view_binding = view_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableResult {
            biglake_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("biglakeConfiguration"),
            ),
            clusterings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterings"),
            ),
            creation_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("creationTime"),
            ),
            dataset_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datasetId"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encryption_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfiguration"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            expiration_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expirationTime"),
            ),
            external_data_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("externalDataConfiguration"),
            ),
            friendly_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("friendlyName"),
            ),
            labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labels"),
            ),
            last_modified_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTime"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            materialized_view: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("materializedView"),
            ),
            max_staleness: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxStaleness"),
            ),
            num_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numBytes"),
            ),
            num_long_term_bytes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numLongTermBytes"),
            ),
            num_rows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("numRows"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            range_partitioning: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("rangePartitioning"),
            ),
            require_partition_filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("requirePartitionFilter"),
            ),
            resource_tags: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceTags"),
            ),
            schema: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("schema"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            table_constraints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableConstraints"),
            ),
            table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableId"),
            ),
            table_replication_info: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tableReplicationInfo"),
            ),
            time_partitioning: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timePartitioning"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            view: pulumi_gestalt_rust::__private::into_domain(o.extract_field("view")),
        }
    }
}
