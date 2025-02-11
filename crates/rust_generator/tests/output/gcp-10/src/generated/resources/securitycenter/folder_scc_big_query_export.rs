/// A Cloud Security Command Center (Cloud SCC) Big Query Export Config.
/// It represents exporting Security Command Center data, including assets, findings, and security marks
/// to a BigQuery instance.
///
/// > **Note:** In order to use Cloud SCC resources, your organization must be enrolled
/// in [SCC Standard/Premium](https://cloud.google.com/security-command-center/docs/quickstart-security-command-center).
/// Without doing so, you may run into errors during resource creation.
///
///
/// To get more information about FolderSccBigQueryExport, see:
///
/// * [API documentation](https://cloud.google.com/security-command-center/docs/reference/rest/v1/folders.bigQueryExports)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/security-command-center/docs/how-to-analyze-findings-in-big-query)
///
/// ## Example Usage
///
/// ### Scc Folder Big Query Export Config Basic
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
///     type: gcp:securitycenter:FolderSccBigQueryExport
///     name: custom_big_query_export_config
///     properties:
///       bigQueryExportId: my-export
///       folder: ${folder.folderId}
///       dataset: ${default.id}
///       description: Cloud Security Command Center Findings Big Query Export Config
///       filter: state="ACTIVE" AND NOT mute="MUTED"
/// ```
///
/// ## Import
///
/// FolderSccBigQueryExport can be imported using any of these accepted formats:
///
/// * `folders/{{folder}}/bigQueryExports/{{big_query_export_id}}`
///
/// * `{{folder}}/{{big_query_export_id}}`
///
/// When using the `pulumi import` command, FolderSccBigQueryExport can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:securitycenter/folderSccBigQueryExport:FolderSccBigQueryExport default folders/{{folder}}/bigQueryExports/{{big_query_export_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:securitycenter/folderSccBigQueryExport:FolderSccBigQueryExport default {{folder}}/{{big_query_export_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod folder_scc_big_query_export {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FolderSccBigQueryExportArgs {
        /// This must be unique within the organization.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub big_query_export_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        #[builder(into)]
        pub dataset: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the export (max of 1024 characters).
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
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
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        #[builder(into)]
        pub folder: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FolderSccBigQueryExportResult {
        /// This must be unique within the organization.
        ///
        ///
        /// - - -
        pub big_query_export_id: pulumi_gestalt_rust::Output<String>,
        /// The time at which the BigQuery export was created.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The dataset to write findings' updates to.
        /// Its format is "projects/[projectId]/datasets/[bigquery_dataset_id]".
        /// BigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
        pub dataset: pulumi_gestalt_rust::Output<String>,
        /// The description of the export (max of 1024 characters).
        pub description: pulumi_gestalt_rust::Output<String>,
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
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// The folder where Cloud Security Command Center Big Query Export
        /// Config lives in.
        pub folder: pulumi_gestalt_rust::Output<String>,
        /// Email address of the user who last edited the BigQuery export.
        pub most_recent_editor: pulumi_gestalt_rust::Output<String>,
        /// The resource name of this export, in the format
        /// `projects/{{project}}/bigQueryExports/{{big_query_export_id}}`.
        /// This field is provided in responses, and is ignored when provided in create requests.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The service account that needs permission to create table and upload data to the BigQuery dataset.
        pub principal: pulumi_gestalt_rust::Output<String>,
        /// The most recent time at which the BigQuery export was updated.
        /// A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits.
        /// Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".
        pub update_time: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FolderSccBigQueryExportArgs,
    ) -> FolderSccBigQueryExportResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let big_query_export_id_binding = args.big_query_export_id.get_output(context);
        let dataset_binding = args.dataset.get_output(context);
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let folder_binding = args.folder.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:securitycenter/folderSccBigQueryExport:FolderSccBigQueryExport"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bigQueryExportId".into(),
                    value: &big_query_export_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataset".into(),
                    value: &dataset_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "folder".into(),
                    value: &folder_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FolderSccBigQueryExportResult {
            big_query_export_id: o.get_field("bigQueryExportId"),
            create_time: o.get_field("createTime"),
            dataset: o.get_field("dataset"),
            description: o.get_field("description"),
            filter: o.get_field("filter"),
            folder: o.get_field("folder"),
            most_recent_editor: o.get_field("mostRecentEditor"),
            name: o.get_field("name"),
            principal: o.get_field("principal"),
            update_time: o.get_field("updateTime"),
        }
    }
}
