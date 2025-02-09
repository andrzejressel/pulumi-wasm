/// Manages a Stream Analytics Output to Blob Storage.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplesa
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleOutputBlob:
///     type: azure:streamanalytics:OutputBlob
///     name: example
///     properties:
///       name: output-to-blob-storage
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountKey: ${exampleAccount.primaryAccessKey}
///       storageContainerName: ${exampleContainer.name}
///       pathPattern: some-pattern
///       dateFormat: yyyy-MM-dd
///       timeFormat: HH
///       serialization:
///         type: Csv
///         encoding: UTF8
///         fieldDelimiter: ','
/// variables:
///   example:
///     fn::invoke:
///       function: azure:streamanalytics:getJob
///       arguments:
///         name: example-job
///         resourceGroupName: ${exampleResourceGroup.name}
/// ```
///
/// ## Import
///
/// Stream Analytics Outputs to Blob Storage can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/outputBlob:OutputBlob example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/outputs/output1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod output_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputBlobArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum wait time per batch in `hh:mm:ss` e.g. `00:02:00` for two minutes.
        #[builder(into, default)]
        pub batch_max_wait_time: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The minimum number of rows per batch (must be between `0` and `1000000`).
        #[builder(into, default)]
        pub batch_min_rows: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Determines whether blob blocks are either committed automatically or appended. Possible values are `Append` and `Once`. Defaults to `Append`.
        #[builder(into, default)]
        pub blob_write_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        #[builder(into)]
        pub date_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        #[builder(into)]
        pub path_pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::streamanalytics::OutputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        #[builder(into, default)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Container within the Storage Account.
        #[builder(into)]
        pub storage_container_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The time format. Wherever `{time}` appears in `path_pattern`, the value of this property is used as the time format instead.
        #[builder(into)]
        pub time_format: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OutputBlobResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum wait time per batch in `hh:mm:ss` e.g. `00:02:00` for two minutes.
        pub batch_max_wait_time: pulumi_gestalt_rust::Output<Option<String>>,
        /// The minimum number of rows per batch (must be between `0` and `1000000`).
        pub batch_min_rows: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Determines whether blob blocks are either committed automatically or appended. Possible values are `Append` and `Once`. Defaults to `Append`.
        pub blob_write_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        pub date_format: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        pub path_pattern: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::OutputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        pub storage_account_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Storage Account.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Container within the Storage Account.
        pub storage_container_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_gestalt_rust::Output<String>,
        /// The time format. Wherever `{time}` appears in `path_pattern`, the value of this property is used as the time format instead.
        pub time_format: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: OutputBlobArgs,
    ) -> OutputBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding_1 = args.authentication_mode.get_output(context);
        let authentication_mode_binding = authentication_mode_binding_1.get_inner();
        let batch_max_wait_time_binding_1 = args.batch_max_wait_time.get_output(context);
        let batch_max_wait_time_binding = batch_max_wait_time_binding_1.get_inner();
        let batch_min_rows_binding_1 = args.batch_min_rows.get_output(context);
        let batch_min_rows_binding = batch_min_rows_binding_1.get_inner();
        let blob_write_mode_binding_1 = args.blob_write_mode.get_output(context);
        let blob_write_mode_binding = blob_write_mode_binding_1.get_inner();
        let date_format_binding_1 = args.date_format.get_output(context);
        let date_format_binding = date_format_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let path_pattern_binding_1 = args.path_pattern.get_output(context);
        let path_pattern_binding = path_pattern_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let serialization_binding_1 = args.serialization.get_output(context);
        let serialization_binding = serialization_binding_1.get_inner();
        let storage_account_key_binding_1 = args.storage_account_key.get_output(context);
        let storage_account_key_binding = storage_account_key_binding_1.get_inner();
        let storage_account_name_binding_1 = args
            .storage_account_name
            .get_output(context);
        let storage_account_name_binding = storage_account_name_binding_1.get_inner();
        let storage_container_name_binding_1 = args
            .storage_container_name
            .get_output(context);
        let storage_container_name_binding = storage_container_name_binding_1
            .get_inner();
        let stream_analytics_job_name_binding_1 = args
            .stream_analytics_job_name
            .get_output(context);
        let stream_analytics_job_name_binding = stream_analytics_job_name_binding_1
            .get_inner();
        let time_format_binding_1 = args.time_format.get_output(context);
        let time_format_binding = time_format_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputBlob:OutputBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationMode".into(),
                    value: &authentication_mode_binding,
                },
                register_interface::ObjectField {
                    name: "batchMaxWaitTime".into(),
                    value: &batch_max_wait_time_binding,
                },
                register_interface::ObjectField {
                    name: "batchMinRows".into(),
                    value: &batch_min_rows_binding,
                },
                register_interface::ObjectField {
                    name: "blobWriteMode".into(),
                    value: &blob_write_mode_binding,
                },
                register_interface::ObjectField {
                    name: "dateFormat".into(),
                    value: &date_format_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "pathPattern".into(),
                    value: &path_pattern_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serialization".into(),
                    value: &serialization_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountKey".into(),
                    value: &storage_account_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountName".into(),
                    value: &storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageContainerName".into(),
                    value: &storage_container_name_binding,
                },
                register_interface::ObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: &stream_analytics_job_name_binding,
                },
                register_interface::ObjectField {
                    name: "timeFormat".into(),
                    value: &time_format_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        OutputBlobResult {
            authentication_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticationMode"),
            ),
            batch_max_wait_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchMaxWaitTime"),
            ),
            batch_min_rows: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("batchMinRows"),
            ),
            blob_write_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blobWriteMode"),
            ),
            date_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dateFormat"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            path_pattern: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pathPattern"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            serialization: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serialization"),
            ),
            storage_account_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountKey"),
            ),
            storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountName"),
            ),
            storage_container_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageContainerName"),
            ),
            stream_analytics_job_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("streamAnalyticsJobName"),
            ),
            time_format: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timeFormat"),
            ),
        }
    }
}
