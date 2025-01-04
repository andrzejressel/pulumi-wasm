/// Manages a organization-level logging sink. For more information see:
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/organizations.sinks)
/// * How-to Guides
///     * [Exporting Logs](https://cloud.google.com/logging/docs/export)
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-sink:
///     type: gcp:logging:OrganizationSink
///     properties:
///       name: my-sink
///       description: some explanation on what this is
///       orgId: '123456789'
///       destination: storage.googleapis.com/${["log-bucket"].name}
///       filter: resource.type = gce_instance AND severity >= WARNING
///   log-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: organization-logging-bucket
///       location: US
///   log-writer:
///     type: gcp:projects:IAMMember
///     properties:
///       project: your-project-id
///       role: roles/storage.objectCreator
///       member: ${["my-sink"].writerIdentity}
/// ```
///
/// ## Import
///
/// Organization-level logging sinks can be imported using this format:
///
/// * `organizations/{{organization_id}}/sinks/{{sink_id}}`
///
/// When using the `pulumi import` command, organization-level logging sinks can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/organizationSink:OrganizationSink default organizations/{{organization_id}}/sinks/{{sink_id}}
/// ```
///
pub mod organization_sink {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSinkArgs {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        #[builder(into, default)]
        pub bigquery_options: pulumi_wasm_rust::Output<
            Option<super::super::types::logging::OrganizationSinkBigqueryOptions>,
        >,
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
            Option<Vec<super::super::types::logging::OrganizationSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not to include children organizations in the sink export. If true, logs
        /// associated with child projects are also exported; otherwise only logs relating to the provided organization are included.
        #[builder(into, default)]
        pub include_children: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether or not to intercept logs from child projects. If true, matching logs will not
        /// match with sinks in child resources, except _Required sinks. This sink will be visible to child resources when listing sinks.
        #[builder(into, default)]
        pub intercept_children: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the logging sink.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The numeric ID of the organization to be exported to the sink.
        #[builder(into)]
        pub org_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        pub bigquery_options: pulumi_wasm_rust::Output<
            super::super::types::logging::OrganizationSinkBigqueryOptions,
        >,
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
            Option<Vec<super::super::types::logging::OrganizationSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether or not to include children organizations in the sink export. If true, logs
        /// associated with child projects are also exported; otherwise only logs relating to the provided organization are included.
        pub include_children: pulumi_wasm_rust::Output<Option<bool>>,
        /// Whether or not to intercept logs from child projects. If true, matching logs will not
        /// match with sinks in child resources, except _Required sinks. This sink will be visible to child resources when listing sinks.
        pub intercept_children: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the logging sink.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The numeric ID of the organization to be exported to the sink.
        pub org_id: pulumi_wasm_rust::Output<String>,
        /// The identity associated with this sink. This identity must be granted write access to the
        /// configured `destination`.
        pub writer_identity: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OrganizationSinkArgs) -> OrganizationSinkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bigquery_options_binding = args.bigquery_options.get_inner();
        let description_binding = args.description.get_inner();
        let destination_binding = args.destination.get_inner();
        let disabled_binding = args.disabled.get_inner();
        let exclusions_binding = args.exclusions.get_inner();
        let filter_binding = args.filter.get_inner();
        let include_children_binding = args.include_children.get_inner();
        let intercept_children_binding = args.intercept_children.get_inner();
        let name_binding = args.name.get_inner();
        let org_id_binding = args.org_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/organizationSink:OrganizationSink".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigqueryOptions".into(),
                    value: &bigquery_options_binding,
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
                    name: "includeChildren".into(),
                    value: &include_children_binding,
                },
                register_interface::ObjectField {
                    name: "interceptChildren".into(),
                    value: &intercept_children_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigqueryOptions".into(),
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
                    name: "includeChildren".into(),
                },
                register_interface::ResultField {
                    name: "interceptChildren".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "orgId".into(),
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
        OrganizationSinkResult {
            bigquery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryOptions").unwrap(),
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
            include_children: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeChildren").unwrap(),
            ),
            intercept_children: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("interceptChildren").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            org_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("orgId").unwrap(),
            ),
            writer_identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("writerIdentity").unwrap(),
            ),
        }
    }
}
