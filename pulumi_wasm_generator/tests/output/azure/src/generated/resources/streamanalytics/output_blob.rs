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
pub mod output_blob {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OutputBlobArgs {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum wait time per batch in `hh:mm:ss` e.g. `00:02:00` for two minutes.
        #[builder(into, default)]
        pub batch_max_wait_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum number of rows per batch (must be between `0` and `1000000`).
        #[builder(into, default)]
        pub batch_min_rows: pulumi_wasm_rust::Output<Option<i32>>,
        /// Determines whether blob blocks are either committed automatically or appended. Possible values are `Append` and `Once`. Defaults to `Append`.
        #[builder(into, default)]
        pub blob_write_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        #[builder(into)]
        pub date_format: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        #[builder(into)]
        pub path_pattern: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        #[builder(into)]
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        #[builder(into, default)]
        pub storage_account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account.
        #[builder(into)]
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Container within the Storage Account.
        #[builder(into)]
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        #[builder(into)]
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The time format. Wherever `{time}` appears in `path_pattern`, the value of this property is used as the time format instead.
        #[builder(into)]
        pub time_format: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct OutputBlobResult {
        /// The authentication mode for the Stream Output. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The maximum wait time per batch in `hh:mm:ss` e.g. `00:02:00` for two minutes.
        pub batch_max_wait_time: pulumi_wasm_rust::Output<Option<String>>,
        /// The minimum number of rows per batch (must be between `0` and `1000000`).
        pub batch_min_rows: pulumi_wasm_rust::Output<Option<i32>>,
        /// Determines whether blob blocks are either committed automatically or appended. Possible values are `Append` and `Once`. Defaults to `Append`.
        pub blob_write_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        pub date_format: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Output. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        pub path_pattern: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_wasm_rust::Output<
            super::super::types::streamanalytics::OutputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        pub storage_account_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Storage Account.
        pub storage_account_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Container within the Storage Account.
        pub storage_container_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Stream Analytics Job. Changing this forces a new resource to be created.
        pub stream_analytics_job_name: pulumi_wasm_rust::Output<String>,
        /// The time format. Wherever `{time}` appears in `path_pattern`, the value of this property is used as the time format instead.
        pub time_format: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: OutputBlobArgs) -> OutputBlobResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_mode_binding = args.authentication_mode.get_inner();
        let batch_max_wait_time_binding = args.batch_max_wait_time.get_inner();
        let batch_min_rows_binding = args.batch_min_rows.get_inner();
        let blob_write_mode_binding = args.blob_write_mode.get_inner();
        let date_format_binding = args.date_format.get_inner();
        let name_binding = args.name.get_inner();
        let path_pattern_binding = args.path_pattern.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let serialization_binding = args.serialization.get_inner();
        let storage_account_key_binding = args.storage_account_key.get_inner();
        let storage_account_name_binding = args.storage_account_name.get_inner();
        let storage_container_name_binding = args.storage_container_name.get_inner();
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_inner();
        let time_format_binding = args.time_format.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:streamanalytics/outputBlob:OutputBlob".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "authenticationMode".into(),
                },
                register_interface::ResultField {
                    name: "batchMaxWaitTime".into(),
                },
                register_interface::ResultField {
                    name: "batchMinRows".into(),
                },
                register_interface::ResultField {
                    name: "blobWriteMode".into(),
                },
                register_interface::ResultField {
                    name: "dateFormat".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "pathPattern".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serialization".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountKey".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountName".into(),
                },
                register_interface::ResultField {
                    name: "storageContainerName".into(),
                },
                register_interface::ResultField {
                    name: "streamAnalyticsJobName".into(),
                },
                register_interface::ResultField {
                    name: "timeFormat".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        OutputBlobResult {
            authentication_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationMode").unwrap(),
            ),
            batch_max_wait_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchMaxWaitTime").unwrap(),
            ),
            batch_min_rows: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("batchMinRows").unwrap(),
            ),
            blob_write_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blobWriteMode").unwrap(),
            ),
            date_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dateFormat").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            path_pattern: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pathPattern").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            serialization: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serialization").unwrap(),
            ),
            storage_account_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountKey").unwrap(),
            ),
            storage_account_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountName").unwrap(),
            ),
            storage_container_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageContainerName").unwrap(),
            ),
            stream_analytics_job_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("streamAnalyticsJobName").unwrap(),
            ),
            time_format: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeFormat").unwrap(),
            ),
        }
    }
}