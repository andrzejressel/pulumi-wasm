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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod v_2_folder_scc_big_query_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
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
        pub big_query_export_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        #[builder(into, default)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The description of the notification config (max of 1024 characters).
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The BigQuery export configuration is stored in this location. If not provided, Use global as default.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct V2FolderSccBigQueryExportResult {
        /// This must be unique within the organization.  It must consist of only lowercase letters,
        /// numbers, and hyphens, must start with a letter, must end with either a letter or a number,
        /// and must be 63 characters or less.
        ///
        ///
        /// - - -
        pub big_query_export_id: pulumi_gestalt_rust::Output<String>,
        /// The time at which the BigQuery export was created. This field is set by the server and will be ignored if provided on export on creation.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        pub dataset: pulumi_gestalt_rust::Output<Option<String>>,
        /// The description of the notification config (max of 1024 characters).
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
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
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// The BigQuery export configuration is stored in this location. If not provided, Use global as default.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Email address of the user who last edited the BigQuery export.
        /// This field is set by the server and will be ignored if provided on export creation or update.
        pub most_recent_editor: pulumi_gestalt_rust::Output<String>,
        /// The resource name of this export, in the format
        /// `folders/{{folder}}/locations/{{location}}/bigQueryExports/{{big_query_export_id}}`.
        /// This field is provided in responses, and is ignored when provided in create requests.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The service account that needs permission to create table and upload data to the BigQuery dataset.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The most recent time at which the BigQuery export was updated. This field is set by the server and will be ignored if provided on export creation or update.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: V2FolderSccBigQueryExportArgs,
    ) -> V2FolderSccBigQueryExportResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let big_query_export_id_binding_1 = args.big_query_export_id.get_output(context);
        let big_query_export_id_binding = big_query_export_id_binding_1.get_inner();
        let dataset_binding_1 = args.dataset.get_output(context);
        let dataset_binding = dataset_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let filter_binding_1 = args.filter.get_output(context);
        let filter_binding = filter_binding_1.get_inner();
        let folder_binding_1 = args.folder.get_output(context);
        let folder_binding = folder_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:securitycenter/v2FolderSccBigQueryExport:V2FolderSccBigQueryExport"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        V2FolderSccBigQueryExportResult {
            big_query_export_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bigQueryExportId"),
            ),
            create_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            dataset: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dataset"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("folder"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            most_recent_editor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mostRecentEditor"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            principal: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("principal"),
            ),
            update_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
