/// Manages a Stream Analytics Stream Input Blob.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestoracc
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
///   exampleStreamInputBlob:
///     type: azure:streamanalytics:StreamInputBlob
///     name: example
///     properties:
///       name: blob-stream-input
///       streamAnalyticsJobName: ${example.name}
///       resourceGroupName: ${example.resourceGroupName}
///       storageAccountName: ${exampleAccount.name}
///       storageAccountKey: ${exampleAccount.primaryAccessKey}
///       storageContainerName: ${exampleContainer.name}
///       pathPattern: some-random-pattern
///       dateFormat: yyyy/MM/dd
///       timeFormat: HH
///       serialization:
///         type: Json
///         encoding: UTF8
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
/// Stream Analytics Stream Input Blob's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/streamInputBlob:StreamInputBlob example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_input_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamInputBlobArgs {
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        #[builder(into)]
        pub date_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Stream Input Blob. Changing this forces a new resource to be created.
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
            super::super::types::streamanalytics::StreamInputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        #[builder(into)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<String>,
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
    pub struct StreamInputBlobResult {
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        pub date_format: pulumi_gestalt_rust::Output<String>,
        /// The name of the Stream Input Blob. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        pub path_pattern: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::StreamInputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account.
        pub storage_account_key: pulumi_gestalt_rust::Output<String>,
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
        args: StreamInputBlobArgs,
    ) -> StreamInputBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
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
            type_: "azure:streamanalytics/streamInputBlob:StreamInputBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
        StreamInputBlobResult {
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
