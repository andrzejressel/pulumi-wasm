/// Databases are containers of tables.
///
///
/// To get more information about Database, see:
///
/// * [API documentation](https://cloud.google.com/bigquery/docs/reference/biglake/rest/v1/projects.locations.catalogs.databases)
/// * How-to Guides
///     * [Manage open source metadata with BigLake Metastore](https://cloud.google.com/bigquery/docs/manage-open-source-metadata#create_databases)
///
/// ## Example Usage
///
/// ### Biglake Database
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
///   database:
///     type: gcp:biglake:Database
///     properties:
///       name: my_database
///       catalog: ${catalog.id}
///       type: HIVE
///       hiveOptions:
///         locationUri: gs://${bucket.name}/${metadataFolder.name}
///         parameters:
///           owner: John Doe
/// ```
///
/// ## Import
///
/// Database can be imported using any of these accepted formats:
///
/// * `{{catalog}}/databases/{{name}}`
///
/// When using the `pulumi import` command, Database can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:biglake/database:Database default {{catalog}}/databases/{{name}}
/// ```
///
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The parent catalog.
        #[builder(into)]
        pub catalog: pulumi_wasm_rust::Output<String>,
        /// Options of a Hive database.
        /// Structure is documented below.
        #[builder(into)]
        pub hive_options: pulumi_wasm_rust::Output<
            super::super::types::biglake::DatabaseHiveOptions,
        >,
        /// The name of the database.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The database type.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The parent catalog.
        pub catalog: pulumi_wasm_rust::Output<String>,
        /// Output only. The creation time of the database. A timestamp in RFC3339
        /// UTC "Zulu" format, with nanosecond resolution and up to nine fractional
        /// digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The deletion time of the database. Only set after the
        /// database is deleted. A timestamp in RFC3339 UTC "Zulu" format, with
        /// nanosecond resolution and up to nine fractional digits. Examples:
        /// "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub delete_time: pulumi_wasm_rust::Output<String>,
        /// Output only. The time when this database is considered expired. Only set
        /// after the database is deleted. A timestamp in RFC3339 UTC "Zulu" format,
        /// with nanosecond resolution and up to nine fractional digits. Examples:
        /// "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub expire_time: pulumi_wasm_rust::Output<String>,
        /// Options of a Hive database.
        /// Structure is documented below.
        pub hive_options: pulumi_wasm_rust::Output<
            super::super::types::biglake::DatabaseHiveOptions,
        >,
        /// The name of the database.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The database type.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Output only. The last modification time of the database. A timestamp in
        /// RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine
        /// fractional digits. Examples: "2014-10-02T15:01:23Z" and
        /// "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: DatabaseArgs) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let catalog_binding = args.catalog.get_inner();
        let hive_options_binding = args.hive_options.get_inner();
        let name_binding = args.name.get_inner();
        let type__binding = args.type_.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:biglake/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "catalog".into(),
                    value: &catalog_binding,
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "catalog".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deleteTime".into(),
                },
                register_interface::ResultField {
                    name: "expireTime".into(),
                },
                register_interface::ResultField {
                    name: "hiveOptions".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseResult {
            catalog: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("catalog").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            delete_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deleteTime").unwrap(),
            ),
            expire_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("expireTime").unwrap(),
            ),
            hive_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hiveOptions").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
