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
pub mod project_sink {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectSinkArgs {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        #[builder(into, default)]
        pub bigquery_options: pulumi_wasm_rust::Output<
            Option<super::super::types::logging::ProjectSinkBigqueryOptions>,
        >,
        /// A user managed service account that will be used to write
        /// the log entries. The format must be `serviceAccount:some@email`. This field can only be specified if you are
        /// routing logs to a destination outside this sink's project. If not specified, a Logging service account
        /// will automatically be generated.
        #[builder(into, default)]
        pub custom_writer_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
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
        pub destination: pulumi_wasm_rust::Output<String>,
        /// If set to True, then this sink is disabled and it does not export any log entries.
        #[builder(into, default)]
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Log entries that match any of the exclusion filters will not be exported. If a log entry is matched by both `filter` and one of `exclusions.filter`, it will not be exported.  Can be repeated multiple times for multiple exclusions. Structure is documented below.
        #[builder(into, default)]
        pub exclusions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::logging::ProjectSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the logging sink. Logging automatically creates two sinks: `_Required` and `_Default`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project to create the sink in. If omitted, the project associated with the provider is
        /// used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not to create a unique identity associated with this sink. If `false`, then the `writer_identity` used is `serviceAccount:cloud-logs@system.gserviceaccount.com`. If `true` (the default),
        /// then a unique service account is created and used for this sink. If you wish to publish logs across projects or utilize
        /// `bigquery_options`, you must set `unique_writer_identity` to true.
        #[builder(into, default)]
        pub unique_writer_identity: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ProjectSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        pub bigquery_options: pulumi_wasm_rust::Output<
            super::super::types::logging::ProjectSinkBigqueryOptions,
        >,
        /// A user managed service account that will be used to write
        /// the log entries. The format must be `serviceAccount:some@email`. This field can only be specified if you are
        /// routing logs to a destination outside this sink's project. If not specified, a Logging service account
        /// will automatically be generated.
        pub custom_writer_identity: pulumi_wasm_rust::Output<Option<String>>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination of the sink (or, in other words, where logs are written to). Can be a Cloud Storage bucket, a PubSub topic, a BigQuery dataset, a Cloud Logging bucket, or a Google Cloud project. Examples:
        ///
        /// - `storage.googleapis.com/[GCS_BUCKET]`
        /// - `bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]`
        /// - `pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]/locations/global/buckets/[BUCKET_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]`
        ///
        /// The writer associated with the sink must have access to write to the above resource.
        pub destination: pulumi_wasm_rust::Output<String>,
        /// If set to True, then this sink is disabled and it does not export any log entries.
        pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Log entries that match any of the exclusion filters will not be exported. If a log entry is matched by both `filter` and one of `exclusions.filter`, it will not be exported.  Can be repeated multiple times for multiple exclusions. Structure is documented below.
        pub exclusions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::logging::ProjectSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the logging sink. Logging automatically creates two sinks: `_Required` and `_Default`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project to create the sink in. If omitted, the project associated with the provider is
        /// used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// Whether or not to create a unique identity associated with this sink. If `false`, then the `writer_identity` used is `serviceAccount:cloud-logs@system.gserviceaccount.com`. If `true` (the default),
        /// then a unique service account is created and used for this sink. If you wish to publish logs across projects or utilize
        /// `bigquery_options`, you must set `unique_writer_identity` to true.
        pub unique_writer_identity: pulumi_wasm_rust::Output<Option<bool>>,
        /// The identity associated with this sink. This identity must be granted write access to the
        /// configured `destination`.
        pub writer_identity: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ProjectSinkArgs) -> ProjectSinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bigquery_options_binding = args.bigquery_options.get_inner();
        let custom_writer_identity_binding = args.custom_writer_identity.get_inner();
        let description_binding = args.description.get_inner();
        let destination_binding = args.destination.get_inner();
        let disabled_binding = args.disabled.get_inner();
        let exclusions_binding = args.exclusions.get_inner();
        let filter_binding = args.filter.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let unique_writer_identity_binding = args.unique_writer_identity.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/projectSink:ProjectSink".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigqueryOptions".into(),
                    value: &bigquery_options_binding,
                },
                register_interface::ObjectField {
                    name: "customWriterIdentity".into(),
                    value: &custom_writer_identity_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "destination".into(),
                    value: &destination_binding,
                },
                register_interface::ObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding,
                },
                register_interface::ObjectField {
                    name: "exclusions".into(),
                    value: &exclusions_binding,
                },
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "uniqueWriterIdentity".into(),
                    value: &unique_writer_identity_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigqueryOptions".into(),
                },
                register_interface::ResultField {
                    name: "customWriterIdentity".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destination".into(),
                },
                register_interface::ResultField {
                    name: "disabled".into(),
                },
                register_interface::ResultField {
                    name: "exclusions".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "uniqueWriterIdentity".into(),
                },
                register_interface::ResultField {
                    name: "writerIdentity".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ProjectSinkResult {
            bigquery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryOptions").unwrap(),
            ),
            custom_writer_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customWriterIdentity").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destination").unwrap(),
            ),
            disabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabled").unwrap(),
            ),
            exclusions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exclusions").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            unique_writer_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uniqueWriterIdentity").unwrap(),
            ),
            writer_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writerIdentity").unwrap(),
            ),
        }
    }
}
