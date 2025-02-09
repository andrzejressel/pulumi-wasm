/// Manages a Stream Analytics Reference Input Blob. Reference data (also known as a lookup table) is a finite data set that is static or slowly changing in nature, used to perform a lookup or to correlate with your data stream. Learn more [here](https://docs.microsoft.com/azure/stream-analytics/stream-analytics-use-reference-data#azure-blob-storage).
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
///   test:
///     type: azure:streamanalytics:ReferenceInputBlob
///     properties:
///       name: blob-reference-input
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
/// Stream Analytics Reference Input Blob's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:streamanalytics/referenceInputBlob:ReferenceInputBlob example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.StreamAnalytics/streamingJobs/job1/inputs/input1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod reference_input_blob {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ReferenceInputBlobArgs {
        /// The authentication mode for the Stream Analytics Reference Input. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        #[builder(into, default)]
        pub authentication_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        #[builder(into)]
        pub date_format: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Reference Input Blob. Changing this forces a new resource to be created.
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
            super::super::types::streamanalytics::ReferenceInputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account. Required if `authentication_mode` is `ConnectionString`.
        #[builder(into, default)]
        pub storage_account_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Storage Account that has the blob container with reference data.
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
    pub struct ReferenceInputBlobResult {
        /// The authentication mode for the Stream Analytics Reference Input. Possible values are `Msi` and `ConnectionString`. Defaults to `ConnectionString`.
        pub authentication_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The date format. Wherever `{date}` appears in `path_pattern`, the value of this property is used as the date format instead.
        pub date_format: pulumi_gestalt_rust::Output<String>,
        /// The name of the Reference Input Blob. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The blob path pattern. Not a regular expression. It represents a pattern against which blob names will be matched to determine whether or not they should be included as input or output to the job.
        pub path_pattern: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Stream Analytics Job exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `serialization` block as defined below.
        pub serialization: pulumi_gestalt_rust::Output<
            super::super::types::streamanalytics::ReferenceInputBlobSerialization,
        >,
        /// The Access Key which should be used to connect to this Storage Account. Required if `authentication_mode` is `ConnectionString`.
        pub storage_account_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Storage Account that has the blob container with reference data.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ReferenceInputBlobArgs,
    ) -> ReferenceInputBlobResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_mode_binding = args.authentication_mode.get_output(context);
        let date_format_binding = args.date_format.get_output(context);
        let name_binding = args.name.get_output(context);
        let path_pattern_binding = args.path_pattern.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let serialization_binding = args.serialization.get_output(context);
        let storage_account_key_binding = args.storage_account_key.get_output(context);
        let storage_account_name_binding = args.storage_account_name.get_output(context);
        let storage_container_name_binding = args
            .storage_container_name
            .get_output(context);
        let stream_analytics_job_name_binding = args
            .stream_analytics_job_name
            .get_output(context);
        let time_format_binding = args.time_format.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:streamanalytics/referenceInputBlob:ReferenceInputBlob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationMode".into(),
                    value: authentication_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dateFormat".into(),
                    value: date_format_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pathPattern".into(),
                    value: path_pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serialization".into(),
                    value: serialization_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountKey".into(),
                    value: storage_account_key_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountName".into(),
                    value: storage_account_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageContainerName".into(),
                    value: storage_container_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamAnalyticsJobName".into(),
                    value: stream_analytics_job_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeFormat".into(),
                    value: time_format_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ReferenceInputBlobResult {
            authentication_mode: o.get_field("authenticationMode"),
            date_format: o.get_field("dateFormat"),
            name: o.get_field("name"),
            path_pattern: o.get_field("pathPattern"),
            resource_group_name: o.get_field("resourceGroupName"),
            serialization: o.get_field("serialization"),
            storage_account_key: o.get_field("storageAccountKey"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_container_name: o.get_field("storageContainerName"),
            stream_analytics_job_name: o.get_field("streamAnalyticsJobName"),
            time_format: o.get_field("timeFormat"),
        }
    }
}
