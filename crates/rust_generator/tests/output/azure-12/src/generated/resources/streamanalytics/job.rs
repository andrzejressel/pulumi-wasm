/// Manages a Stream Analytics Job.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleJob:
///     type: azure:streamanalytics:Job
///     name: example
///     properties:
///       name: example-job
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       compatibilityLevel: '1.2'
///       dataLocale: en-GB
///       eventsLateArrivalMaxDelayInSeconds: 60
///       eventsOutOfOrderMaxDelayInSeconds: 50
///       eventsOutOfOrderPolicy: Adjust
///       outputErrorPolicy: Drop
///       streamingUnits: 3
///       skuName: StandardV2
///       tags:
///         environment: Example
///       transformationQuery: |2
///             SELECT *
///             INTO [YourOutputAlias]
///             FROM [YourInputAlias]
/// ```
///
/// ## Import
///
/// Stream Analytics Job's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/job:Job example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// Specifies the compatibility level for this job - which controls certain runtime behaviours of the streaming job. Possible values are `1.0`, `1.1` and `1.2`.
        ///
        /// > **NOTE:** Support for Compatibility Level 1.2 is dependent on a new version of the Stream Analytics API, which [being tracked in this issue](https://github.com/Azure/azure-rest-api-specs/issues/5604).
        #[builder(into, default)]
        pub compatibility_level: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy for storing stream analytics content. Possible values are `JobStorageAccount`, `SystemAccount`. Defaults to `SystemAccount`.
        #[builder(into, default)]
        pub content_storage_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Data Locale of the Job, which [should be a supported .NET Culture](https://msdn.microsoft.com/en-us/library/system.globalization.culturetypes(v=vs.110).aspx). Defaults to `en-US`.
        #[builder(into, default)]
        pub data_locale: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the maximum tolerable delay in seconds where events arriving late could be included. Supported range is `-1` (indefinite) to `1814399` (20d 23h 59m 59s). Default is `5`.
        #[builder(into, default)]
        pub events_late_arrival_max_delay_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the maximum tolerable delay in seconds where out-of-order events can be adjusted to be back in order. Supported range is `0` to `599` (9m 59s). Default is `0`.
        #[builder(into, default)]
        pub events_out_of_order_max_delay_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Specifies the policy which should be applied to events which arrive out of order in the input event stream. Possible values are `Adjust` and `Drop`. Default is `Adjust`.
        #[builder(into, default)]
        pub events_out_of_order_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::streamanalytics::JobIdentity>,
        >,
        /// The details of the job storage account. A `job_storage_account` block as defined below.
        #[builder(into, default)]
        pub job_storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::streamanalytics::JobJobStorageAccount>>,
        >,
        /// The Azure Region in which the Resource Group exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the policy which should be applied to events which arrive at the output and cannot be written to the external storage due to being malformed (such as missing column values, column values of wrong type or size). Possible values are `Drop` and `Stop`. Default is `Drop`.
        #[builder(into, default)]
        pub output_error_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU Name to use for the Stream Analytics Job. Possible values are `Standard`, `StandardV2`. Defaults to `Standard`.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of an existing Stream Analytics Cluster where the Stream Analytics Job should run.
        #[builder(into, default)]
        pub stream_analytics_cluster_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the number of streaming units that the streaming job uses. Supported values are `1`, `3`, `6` and multiples of `6` up to `120`. A conversion table for V2 streaming units can be found [here](https://learn.microsoft.com/azure/stream-analytics/stream-analytics-streaming-unit-consumption#understand-streaming-unit-conversions-and-where-they-apply)
        ///
        /// > **NOTE:** `streaming_units` must be set when `type` is `Cloud`.
        #[builder(into, default)]
        pub streaming_units: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into)]
        pub transformation_query: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the Stream Analytics Job. Possible values are `Cloud` and `Edge`. Defaults to `Cloud`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `Edge` doesn't support `stream_analytics_cluster_id` and `streaming_units`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// Specifies the compatibility level for this job - which controls certain runtime behaviours of the streaming job. Possible values are `1.0`, `1.1` and `1.2`.
        ///
        /// > **NOTE:** Support for Compatibility Level 1.2 is dependent on a new version of the Stream Analytics API, which [being tracked in this issue](https://github.com/Azure/azure-rest-api-specs/issues/5604).
        pub compatibility_level: pulumi_gestalt_rust::Output<String>,
        /// The policy for storing stream analytics content. Possible values are `JobStorageAccount`, `SystemAccount`. Defaults to `SystemAccount`.
        pub content_storage_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Data Locale of the Job, which [should be a supported .NET Culture](https://msdn.microsoft.com/en-us/library/system.globalization.culturetypes(v=vs.110).aspx). Defaults to `en-US`.
        pub data_locale: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the maximum tolerable delay in seconds where events arriving late could be included. Supported range is `-1` (indefinite) to `1814399` (20d 23h 59m 59s). Default is `5`.
        pub events_late_arrival_max_delay_in_seconds: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// Specifies the maximum tolerable delay in seconds where out-of-order events can be adjusted to be back in order. Supported range is `0` to `599` (9m 59s). Default is `0`.
        pub events_out_of_order_max_delay_in_seconds: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// Specifies the policy which should be applied to events which arrive out of order in the input event stream. Possible values are `Adjust` and `Drop`. Default is `Adjust`.
        pub events_out_of_order_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::streamanalytics::JobIdentity>,
        >,
        /// The Job ID assigned by the Stream Analytics Job.
        pub job_id: pulumi_gestalt_rust::Output<String>,
        /// The details of the job storage account. A `job_storage_account` block as defined below.
        pub job_storage_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::streamanalytics::JobJobStorageAccount>>,
        >,
        /// The Azure Region in which the Resource Group exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the policy which should be applied to events which arrive at the output and cannot be written to the external storage due to being malformed (such as missing column values, column values of wrong type or size). Possible values are `Drop` and `Stop`. Default is `Drop`.
        pub output_error_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Stream Analytics Job should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU Name to use for the Stream Analytics Job. Possible values are `Standard`, `StandardV2`. Defaults to `Standard`.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of an existing Stream Analytics Cluster where the Stream Analytics Job should run.
        pub stream_analytics_cluster_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the number of streaming units that the streaming job uses. Supported values are `1`, `3`, `6` and multiples of `6` up to `120`. A conversion table for V2 streaming units can be found [here](https://learn.microsoft.com/azure/stream-analytics/stream-analytics-streaming-unit-consumption#understand-streaming-unit-conversions-and-where-they-apply)
        ///
        /// > **NOTE:** `streaming_units` must be set when `type` is `Cloud`.
        pub streaming_units: pulumi_gestalt_rust::Output<Option<i32>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub transformation_query: pulumi_gestalt_rust::Output<String>,
        /// The type of the Stream Analytics Job. Possible values are `Cloud` and `Edge`. Defaults to `Cloud`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `Edge` doesn't support `stream_analytics_cluster_id` and `streaming_units`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: JobArgs,
    ) -> JobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let compatibility_level_binding = args.compatibility_level.get_output(context);
        let content_storage_policy_binding = args
            .content_storage_policy
            .get_output(context);
        let data_locale_binding = args.data_locale.get_output(context);
        let events_late_arrival_max_delay_in_seconds_binding = args
            .events_late_arrival_max_delay_in_seconds
            .get_output(context);
        let events_out_of_order_max_delay_in_seconds_binding = args
            .events_out_of_order_max_delay_in_seconds
            .get_output(context);
        let events_out_of_order_policy_binding = args
            .events_out_of_order_policy
            .get_output(context);
        let identity_binding = args.identity.get_output(context);
        let job_storage_accounts_binding = args.job_storage_accounts.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let output_error_policy_binding = args.output_error_policy.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let stream_analytics_cluster_id_binding = args
            .stream_analytics_cluster_id
            .get_output(context);
        let streaming_units_binding = args.streaming_units.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let transformation_query_binding = args.transformation_query.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "compatibilityLevel".into(),
                    value: compatibility_level_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "contentStoragePolicy".into(),
                    value: content_storage_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataLocale".into(),
                    value: data_locale_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventsLateArrivalMaxDelayInSeconds".into(),
                    value: events_late_arrival_max_delay_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventsOutOfOrderMaxDelayInSeconds".into(),
                    value: events_out_of_order_max_delay_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "eventsOutOfOrderPolicy".into(),
                    value: events_out_of_order_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jobStorageAccounts".into(),
                    value: job_storage_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputErrorPolicy".into(),
                    value: output_error_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsClusterId".into(),
                    value: stream_analytics_cluster_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingUnits".into(),
                    value: streaming_units_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "transformationQuery".into(),
                    value: transformation_query_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobResult {
            compatibility_level: o.get_field("compatibilityLevel"),
            content_storage_policy: o.get_field("contentStoragePolicy"),
            data_locale: o.get_field("dataLocale"),
            events_late_arrival_max_delay_in_seconds: o
                .get_field("eventsLateArrivalMaxDelayInSeconds"),
            events_out_of_order_max_delay_in_seconds: o
                .get_field("eventsOutOfOrderMaxDelayInSeconds"),
            events_out_of_order_policy: o.get_field("eventsOutOfOrderPolicy"),
            identity: o.get_field("identity"),
            job_id: o.get_field("jobId"),
            job_storage_accounts: o.get_field("jobStorageAccounts"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            output_error_policy: o.get_field("outputErrorPolicy"),
            resource_group_name: o.get_field("resourceGroupName"),
            sku_name: o.get_field("skuName"),
            stream_analytics_cluster_id: o.get_field("streamAnalyticsClusterId"),
            streaming_units: o.get_field("streamingUnits"),
            tags: o.get_field("tags"),
            transformation_query: o.get_field("transformationQuery"),
            type_: o.get_field("type"),
        }
    }
}
