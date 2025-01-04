/// A Cloud Security Command Center (Cloud SCC) Big Query Export Config.
/// It represents exporting Security Command Center data, including assets, findings, and security marks
/// using gcloud scc bqexports
/// > **Note:** In order to use Cloud SCC resources, your organization must be enrolled
/// in [SCC Standard/Premium](https://cloud.google.com/security-command-center/docs/quickstart-security-command-center).
/// Without doing so, you may run into errors during resource creation.
///
///
/// To get more information about FolderSccBigQueryExport, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v2/folders.locations.bigQueryExports)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs/how-to-analyze-findings-in-big-query)
///
/// ## Example Usage
///
/// ### Scc V2 Folder Big Query Export Config Basic
///
///
/// ```yaml
/// resources:
///   folder:
///     type: gcp:organizations:Folder
///     properties:
///       parent: organizations/123456789
///       displayName: folder-name
///       deletionProtection: false
///   default:
///     type: gcp:bigquery:Dataset
///     properties:
///       datasetId: my_dataset_id
///       friendlyName: test
///       description: This is a test description
///       location: US
///       defaultTableExpirationMs: 3.6e+06
///       defaultPartitionExpirationMs: null
///       labels:
///         env: default
///   customBigQueryExportConfig:
///     type: gcp:securitycenter:V2FolderSccBigQueryExport
///     name: custom_big_query_export_config
///     properties:
///       bigQueryExportId: my-export
///       folder: ${folder.folderId}
///       dataset: ${default.id}
///       location: global
///       description: Cloud Security Command Center Findings Big Query Export Config
///       filter: state="ACTIVE" AND NOT mute="MUTED"
/// ```
///
/// ## Import
///
/// FolderSccBigQueryExport can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/locations/{{location}}/bigQueryExports/{{big_query_export_id}}`
///
/// * `{{folder}}/{{location}}/{{big_query_export_id}}`
///
/// When using the `pulumi import` command, FolderSccBigQueryExport can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2FolderSccBigQueryExport:V2FolderSccBigQueryExport default folders/{{folder}}/locations/{{location}}/bigQueryExports/{{big_query_export_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/v2FolderSccBigQueryExport:V2FolderSccBigQueryExport default {{folder}}/{{location}}/{{big_query_export_id}}
/// ```
///
pub mod v_2_folder_scc_big_query_export {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct V2FolderSccBigQueryExportArgs {
        /// This must be unique within the organization.  It must consist of only lowercase letters,
        /// numbers, and hyphens, must start with a letter, must end with either a letter or a number,
        /// and must be 63 characters or less.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub big_query_export_id: pulumi_wasm_rust::Output<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        #[builder(into, default)]
        pub dataset: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the notification config (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Expression that defines the filter to apply across create/update
        /// events of findings. The
        /// expression is a list of zero or more restrictions combined via
        /// logical operators AND and OR. Parentheses are supported, and OR
        /// has higher precedence than AND.
        /// Restrictions have the form <field> <operator> <value> and may have
        /// a - character in front of them to indicate negation. The fields
        /// map to those defined in the corresponding resource.
        /// The supported operators are:
        /// * = for all value types.
        /// * >, <, >=, <= for integer values.
        /// * :, meaning substring matching, for strings.
        /// The supported value types are:
        /// * string literals in quotes.
        /// * integer literals without quotes.
        /// * boolean literals true and false without quotes.
        /// See
        /// [Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)
        /// for information on how to write a filter.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        #[builder(into)]
        pub folder: pulumi_wasm_rust::Output<String>,
        /// The BigQuery export configuration is stored in this location. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2FolderSccBigQueryExportResult {
        /// This must be unique within the organization.  It must consist of only lowercase letters,
        /// numbers, and hyphens, must start with a letter, must end with either a letter or a number,
        /// and must be 63 characters or less.
        ///
        ///
        /// - - -
        pub big_query_export_id: pulumi_wasm_rust::Output<String>,
        /// The time at which the BigQuery export was created. This field is set by the server and will be ignored if provided on export on creation.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        pub dataset: pulumi_wasm_rust::Output<Option<String>>,
        /// The description of the notification config (max of 1024 characters).
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Expression that defines the filter to apply across create/update
        /// events of findings. The
        /// expression is a list of zero or more restrictions combined via
        /// logical operators AND and OR. Parentheses are supported, and OR
        /// has higher precedence than AND.
        /// Restrictions have the form <field> <operator> <value> and may have
        /// a - character in front of them to indicate negation. The fields
        /// map to those defined in the corresponding resource.
        /// The supported operators are:
        /// * = for all value types.
        /// * >, <, >=, <= for integer values.
        /// * :, meaning substring matching, for strings.
        /// The supported value types are:
        /// * string literals in quotes.
        /// * integer literals without quotes.
        /// * boolean literals true and false without quotes.
        /// See
        /// [Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)
        /// for information on how to write a filter.
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        pub folder: pulumi_wasm_rust::Output<String>,
        /// The BigQuery export configuration is stored in this location. If not provided, Use global as default.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Email address of the user who last edited the BigQuery export.
        /// This field is set by the server and will be ignored if provided on export creation or update.
        pub most_recent_editor: pulumi_wasm_rust::Output<String>,
        /// The resource name of this export, in the format
        /// `folders/{{folder}}/locations/{{location}}/bigQueryExports/{{big_query_export_id}}`.
        /// This field is provided in responses, and is ignored when provided in create requests.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The service account that needs permission to create table and upload data to the BigQuery dataset.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// The most recent time at which the BigQuery export was updated. This field is set by the server and will be ignored if provided on export creation or update.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: V2FolderSccBigQueryExportArgs,
    ) -> V2FolderSccBigQueryExportResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let big_query_export_id_binding = args.big_query_export_id.get_inner();
        let dataset_binding = args.dataset.get_inner();
        let description_binding = args.description.get_inner();
        let filter_binding = args.filter.get_inner();
        let folder_binding = args.folder.get_inner();
        let location_binding = args.location.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2FolderSccBigQueryExport:V2FolderSccBigQueryExport"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigQueryExportId".into(),
                    value: &big_query_export_id_binding,
                },
                register_interface::ObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "folder".into(),
                    value: &folder_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigQueryExportId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dataset".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "folder".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mostRecentEditor".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
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
        V2FolderSccBigQueryExportResult {
            big_query_export_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigQueryExportId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dataset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dataset").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            folder: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("folder").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            most_recent_editor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecentEditor").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
