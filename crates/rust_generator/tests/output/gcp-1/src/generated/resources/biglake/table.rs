/// Represents a table.
///
///
/// To get more information about Table, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/biglake/rest/v1/projects.locations.catalogs.databases.tables)
/// * How-to Guides
///     * [Manage open source metadata with BigLake Metastore](https://cloud.google.com/bigquery/docs/manage-open-source-metadata#create_tables)
///
/// ## Example Usage
///
/// ### Biglake Table
///
///
/// ```yaml
/// resources:
///   catalog:
///     type: gcp:biglake:Catalog
///     properties:
///       name: my_catalog
///       location: US
///   bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: my_bucket
///       location: US
///       forceDestroy: true
///       uniformBucketLevelAccess: true
///   metadataFolder:
///     type: gcp:storage:BucketObject
///     name: metadata_folder
///     properties:
///       name: metadata/
///       content: ' '
///       bucket: ${bucket.name}
///   dataFolder:
///     type: gcp:storage:BucketObject
///     name: data_folder
///     properties:
///       name: data/
///       content: ' '
///       bucket: ${bucket.name}
///   database:
///     type: gcp:biglake:Database
///     properties:
///       name: my_database
///       catalog: ${catalog.id}
///       type: HIVE
///       hiveOptions:
///         locationUri: gs://${bucket.name}/${metadataFolder.name}
///         parameters:
///           owner: Alex
///   table:
///     type: gcp:biglake:Table
///     properties:
///       name: my_table
///       database: ${database.id}
///       type: HIVE
///       hiveOptions:
///         tableType: MANAGED_TABLE
///         storageDescriptor:
///           locationUri: gs://${bucket.name}/${dataFolder.name}
///           inputFormat: org.apache.hadoop.mapred.SequenceFileInputFormat
///           outputFormat: org.apache.hadoop.hive.ql.io.HiveSequenceFileOutputFormat
///         parameters:
///           spark.sql.create.version: 3.1.3
///           spark.sql.sources.schema.numParts: '1'
///           transient_lastDdlTime: '1680894197'
///           spark.sql.partitionProvider: catalog
///           owner: John Doe
///           spark.sql.sources.schema.part.0: '{"type":"struct","fields":[{"name":"id","type":"integer","nullable":true,"metadata":{}},{"name":"name","type":"string","nullable":true,"metadata":{}},{"name":"age","type":"integer","nullable":true,"metadata":{}}]}'
///           spark.sql.sources.provider: iceberg
///           provider: iceberg
/// ```
///
/// ## Import
///
/// Table can be imported using any of these accepted formats:
///
/// * `{{database}}/tables/{{name}}`
///
/// When using the `pulumi import` command, Table can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:biglake/table:Table default {{database}}/tables/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TableArgs {
        /// The id of the parent database.
        #[builder(into, default)]
        pub database: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options of a Hive table.
        /// Structure is documented below.
        #[builder(into, default)]
        pub hive_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::biglake::TableHiveOptions>,
        >,
        /// Output only. The name of the Table. Format:
        /// projects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}/databases/{databaseId}/tables/{tableId}
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The database type.
        /// Possible values are: `HIVE`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TableResult {
        /// Output only. The creation time of the table. A timestamp in RFC3339 UTC
        /// "Zulu" format, with nanosecond resolution and up to nine fractional
        /// digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The id of the parent database.
        pub database: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. The deletion time of the table. Only set after the
        /// table is deleted. A timestamp in RFC3339 UTC "Zulu" format, with
        /// nanosecond resolution and up to nine fractional digits. Examples:
        /// "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub delete_time: pulumi_gestalt_rust::Output<String>,
        /// The checksum of a table object computed by the server based on the value
        /// of other fields. It may be sent on update requests to ensure the client
        /// has an up-to-date value before proceeding. It is only checked for update
        /// table operations.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time when this table is considered expired. Only set
        /// after the table is deleted. A timestamp in RFC3339 UTC "Zulu" format,
        /// with nanosecond resolution and up to nine fractional digits. Examples:
        /// "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub expire_time: pulumi_gestalt_rust::Output<String>,
        /// Options of a Hive table.
        /// Structure is documented below.
        pub hive_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::biglake::TableHiveOptions>,
        >,
        /// Output only. The name of the Table. Format:
        /// projects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}/databases/{databaseId}/tables/{tableId}
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The database type.
        /// Possible values are: `HIVE`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// Output only. The last modification time of the table. A timestamp in
        /// RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
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
        let database_binding = args.database.get_output(context).get_inner();
        let hive_options_binding = args.hive_options.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:biglake/table:Table".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "database".into(),
                    value: &database_binding,
                },
                register_interface::ObjectField {
                    name: "hiveOptions".into(),
                    value: &hive_options_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        TableResult {
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            database: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("database"),
            ),
            delete_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteTime"),
            ),
            etag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("etag")),
            expire_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("expireTime"),
            ),
            hive_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hiveOptions"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
