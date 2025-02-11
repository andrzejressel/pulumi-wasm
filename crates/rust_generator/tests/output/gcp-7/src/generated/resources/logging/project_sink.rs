///
///
/// ## Import
///
/// Project-level logging sinks can be imported using their URI, e.g.
///
/// * `projects/{{project_id}}/sinks/{{name}}`
///
/// When using the `pulumi import` command, project-level logging sinks can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/projectSink:ProjectSink default projects/{{project_id}}/sinks/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod project_sink {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectSinkArgs {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        #[builder(into, default)]
        pub bigquery_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::ProjectSinkBigqueryOptions>,
        >,
        /// A user managed service account that will be used to write
        /// the log entries. The format must be `serviceAccount:some@email`. This field can only be specified if you are
        /// routing logs to a destination outside this sink's project. If not specified, a Logging service account
        /// will automatically be generated.
        #[builder(into, default)]
        pub custom_writer_identity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, a BigQuery dataset, a Cloud Logging bucket, or a Google Cloud project. Examples:
        ///
        /// - `storage.googleapis.com/[GCS_BUCKET]`
        /// - `bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]`
        /// - `pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]/locations/global/buckets/[BUCKET_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]`
        ///
        /// The writer associated with the sink must have access to write to the above resource.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If set to True, then this sink is disabled and it does not export any log entries.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Log entries that match any of the exclusion filters will not be exported. If a log entry is matched by both `filter` and one of `exclusions.filter`, it will not be exported.  Can be repeated multiple times for multiple exclusions. Structure is documented below.
        #[builder(into, default)]
        pub exclusions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::logging::ProjectSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the logging sink. Logging automatically creates two sinks: `_Required` and `_Default`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project to create the sink in. If omitted, the project associated with the provider is
        /// used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether or not to create a unique identity associated with this sink. If `false`, then the `writer_identity` used is `serviceAccount:cloud-logs@system.gserviceaccount.com`. If `true` (the default),
        /// then a unique service account is created and used for this sink. If you wish to publish logs across projects or utilize
        /// `bigquery_options`, you must set `unique_writer_identity` to true.
        #[builder(into, default)]
        pub unique_writer_identity: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ProjectSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        pub bigquery_options: pulumi_gestalt_rust::Output<
            super::super::types::logging::ProjectSinkBigqueryOptions,
        >,
        /// A user managed service account that will be used to write
        /// the log entries. The format must be `serviceAccount:some@email`. This field can only be specified if you are
        /// routing logs to a destination outside this sink's project. If not specified, a Logging service account
        /// will automatically be generated.
        pub custom_writer_identity: pulumi_gestalt_rust::Output<Option<String>>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, a BigQuery dataset, a Cloud Logging bucket, or a Google Cloud project. Examples:
        ///
        /// - `storage.googleapis.com/[GCS_BUCKET]`
        /// - `bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]`
        /// - `pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]/locations/global/buckets/[BUCKET_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]`
        ///
        /// The writer associated with the sink must have access to write to the above resource.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// If set to True, then this sink is disabled and it does not export any log entries.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Log entries that match any of the exclusion filters will not be exported. If a log entry is matched by both `filter` and one of `exclusions.filter`, it will not be exported.  Can be repeated multiple times for multiple exclusions. Structure is documented below.
        pub exclusions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::logging::ProjectSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the logging sink. Logging automatically creates two sinks: `_Required` and `_Default`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project to create the sink in. If omitted, the project associated with the provider is
        /// used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Whether or not to create a unique identity associated with this sink. If `false`, then the `writer_identity` used is `serviceAccount:cloud-logs@system.gserviceaccount.com`. If `true` (the default),
        /// then a unique service account is created and used for this sink. If you wish to publish logs across projects or utilize
        /// `bigquery_options`, you must set `unique_writer_identity` to true.
        pub unique_writer_identity: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The identity associated with this sink. This identity must be granted write access to the
        /// configured `destination`.
        pub writer_identity: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProjectSinkArgs,
    ) -> ProjectSinkResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bigquery_options_binding = args.bigquery_options.get_output(context);
        let custom_writer_identity_binding = args
            .custom_writer_identity
            .get_output(context);
        let description_binding = args.description.get_output(context);
        let destination_binding = args.destination.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let exclusions_binding = args.exclusions.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let unique_writer_identity_binding = args
            .unique_writer_identity
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/projectSink:ProjectSink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bigqueryOptions".into(),
                    value: &bigquery_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customWriterIdentity".into(),
                    value: &custom_writer_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: &destination_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "exclusions".into(),
                    value: &exclusions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "uniqueWriterIdentity".into(),
                    value: &unique_writer_identity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProjectSinkResult {
            bigquery_options: o.get_field("bigqueryOptions"),
            custom_writer_identity: o.get_field("customWriterIdentity"),
            description: o.get_field("description"),
            destination: o.get_field("destination"),
            disabled: o.get_field("disabled"),
            exclusions: o.get_field("exclusions"),
            filter: o.get_field("filter"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            unique_writer_identity: o.get_field("uniqueWriterIdentity"),
            writer_identity: o.get_field("writerIdentity"),
        }
    }
}
