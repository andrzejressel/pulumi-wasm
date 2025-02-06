/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/billingAccounts.sinks)
/// * How-to Guides
///     * [Exporting Logs](https://cloud.google.com/logging/docs/export)
///
/// > **Note** You must have the "Logs Configuration Writer" IAM role (`roles/logging.configWriter`)
/// [granted on the billing account](https://cloud.google.com/billing/reference/rest/v1/billingAccounts/getIamPolicy) to
/// the credentials used with this provider. [IAM roles granted on a billing account](https://cloud.google.com/billing/docs/how-to/billing-access) are separate from the
/// typical IAM roles granted on a project.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   my-sink:
///     type: gcp:logging:BillingAccountSink
///     properties:
///       name: my-sink
///       description: some explanation on what this is
///       billingAccount: ABCDEF-012345-GHIJKL
///       destination: storage.googleapis.com/${["log-bucket"].name}
///   log-bucket:
///     type: gcp:storage:Bucket
///     properties:
///       name: billing-logging-bucket
///       location: US
///   log-writer:
///     type: gcp:projects:IAMBinding
///     properties:
///       project: your-project-id
///       role: roles/storage.objectCreator
///       members:
///         - ${["my-sink"].writerIdentity}
/// ```
///
/// ## Import
///
/// Billing account logging sinks can be imported using this format:
///
/// * `billingAccounts/{{billing_account_id}}/sinks/{{sink_id}}`
///
/// When using the `pulumi import` command, billing account logging sinks can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/billingAccountSink:BillingAccountSink default billingAccounts/{{billing_account_id}}/sinks/{{sink_id}}
/// ```
///
pub mod billing_account_sink {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BillingAccountSinkArgs {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        #[builder(into, default)]
        pub bigquery_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::BillingAccountSinkBigqueryOptions>,
        >,
        /// The billing account exported to the sink.
        #[builder(into)]
        pub billing_account: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The destination of the sink (or, in other words, where logs are written to). Can be a
        /// Cloud Storage bucket, a PubSub topic, a BigQuery dataset or a Cloud Logging bucket. Examples:
        ///
        /// - `storage.googleapis.com/[GCS_BUCKET]`
        /// - `bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]`
        /// - `pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]]/locations/global/buckets/[BUCKET_ID]`
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
            Option<Vec<super::super::types::logging::BillingAccountSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the logging sink.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BillingAccountSinkResult {
        /// Options that affect sinks exporting data to BigQuery. Structure documented below.
        pub bigquery_options: pulumi_gestalt_rust::Output<
            super::super::types::logging::BillingAccountSinkBigqueryOptions,
        >,
        /// The billing account exported to the sink.
        pub billing_account: pulumi_gestalt_rust::Output<String>,
        /// A description of this sink. The maximum length of the description is 8000 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The destination of the sink (or, in other words, where logs are written to). Can be a
        /// Cloud Storage bucket, a PubSub topic, a BigQuery dataset or a Cloud Logging bucket. Examples:
        ///
        /// - `storage.googleapis.com/[GCS_BUCKET]`
        /// - `bigquery.googleapis.com/projects/[PROJECT_ID]/datasets/[DATASET]`
        /// - `pubsub.googleapis.com/projects/[PROJECT_ID]/topics/[TOPIC_ID]`
        /// - `logging.googleapis.com/projects/[PROJECT_ID]]/locations/global/buckets/[BUCKET_ID]`
        ///
        /// The writer associated with the sink must have access to write to the above resource.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// If set to True, then this sink is disabled and it does not export any log entries.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Log entries that match any of the exclusion filters will not be exported. If a log entry is matched by both `filter` and one of `exclusions.filter`, it will not be exported.  Can be repeated multiple times for multiple exclusions. Structure is documented below.
        pub exclusions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::logging::BillingAccountSinkExclusion>>,
        >,
        /// The filter to apply when exporting logs. Only log entries that match the filter are exported.
        /// See [Advanced Log Filters](https://cloud.google.com/logging/docs/view/advanced_filters) for information on how to
        /// write a filter.
        pub filter: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the logging sink.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The identity associated with this sink. This identity must be granted write access to the
        /// configured `destination`.
        pub writer_identity: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: BillingAccountSinkArgs,
    ) -> BillingAccountSinkResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let bigquery_options_binding = args
            .bigquery_options
            .get_output(context)
            .get_inner();
        let billing_account_binding = args
            .billing_account
            .get_output(context)
            .get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let destination_binding = args.destination.get_output(context).get_inner();
        let disabled_binding = args.disabled.get_output(context).get_inner();
        let exclusions_binding = args.exclusions.get_output(context).get_inner();
        let filter_binding = args.filter.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:logging/billingAccountSink:BillingAccountSink".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigqueryOptions".into(),
                    value: &bigquery_options_binding,
                },
                register_interface::ObjectField {
                    name: "billingAccount".into(),
                    value: &billing_account_binding,
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
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BillingAccountSinkResult {
            bigquery_options: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bigqueryOptions"),
            ),
            billing_account: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("billingAccount"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destination"),
            ),
            disabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("disabled"),
            ),
            exclusions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("exclusions"),
            ),
            filter: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filter"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            writer_identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("writerIdentity"),
            ),
        }
    }
}
