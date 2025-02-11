/// Manages a Batch Job.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("west europe")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .location("${example.location}")
///             .name("exampleaccount")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleJob = job::create(
///         "exampleJob",
///         JobArgs::builder()
///             .batch_pool_id("${examplePool.id}")
///             .name("examplejob")
///             .build_struct(),
///     );
///     let examplePool = pool::create(
///         "examplePool",
///         PoolArgs::builder()
///             .account_name("${exampleAccount.name}")
///             .fixed_scale(
///                 PoolFixedScale::builder().targetDedicatedNodes(1).build_struct(),
///             )
///             .name("examplepool")
///             .node_agent_sku_id("batch.node.ubuntu 16.04")
///             .resource_group_name("${example.name}")
///             .storage_image_reference(
///                 PoolStorageImageReference::builder()
///                     .offer("0001-com-ubuntu-server-jammy")
///                     .publisher("Canonical")
///                     .sku("22_04-lts")
///                     .version("latest")
///                     .build_struct(),
///             )
///             .vm_size("Standard_A1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Batch Jobs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:batch/job:Job example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Batch/batchAccounts/account1/pools/pool1/jobs/job1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct JobArgs {
        /// The ID of the Batch Pool. Changing this forces a new Batch Job to be created.
        #[builder(into)]
        pub batch_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies a map of common environment settings applied to this Batch Job. Changing this forces a new Batch Job to be created.
        #[builder(into, default)]
        pub common_environment_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The display name of this Batch Job. Changing this forces a new Batch Job to be created.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Batch Job. Changing this forces a new Batch Job to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The priority of this Batch Job, possible values can range from -1000 (lowest) to 1000 (highest). Defaults to `0`.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The number of retries to each Batch Task belongs to this Batch Job. If this is set to `0`, the Batch service does not retry Tasks. If this is set to `-1`, the Batch service retries Batch Tasks without limit.
        #[builder(into, default)]
        pub task_retry_maximum: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct JobResult {
        /// The ID of the Batch Pool. Changing this forces a new Batch Job to be created.
        pub batch_pool_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies a map of common environment settings applied to this Batch Job. Changing this forces a new Batch Job to be created.
        pub common_environment_properties: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The display name of this Batch Job. Changing this forces a new Batch Job to be created.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Batch Job. Changing this forces a new Batch Job to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of this Batch Job, possible values can range from -1000 (lowest) to 1000 (highest). Defaults to `0`.
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The number of retries to each Batch Task belongs to this Batch Job. If this is set to `0`, the Batch service does not retry Tasks. If this is set to `-1`, the Batch service retries Batch Tasks without limit.
        pub task_retry_maximum: pulumi_gestalt_rust::Output<Option<i32>>,
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
        let batch_pool_id_binding = args.batch_pool_id.get_output(context);
        let common_environment_properties_binding = args
            .common_environment_properties
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let task_retry_maximum_binding = args.task_retry_maximum.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:batch/job:Job".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "batchPoolId".into(),
                    value: &batch_pool_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commonEnvironmentProperties".into(),
                    value: &common_environment_properties_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskRetryMaximum".into(),
                    value: &task_retry_maximum_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        JobResult {
            batch_pool_id: o.get_field("batchPoolId"),
            common_environment_properties: o.get_field("commonEnvironmentProperties"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            task_retry_maximum: o.get_field("taskRetryMaximum"),
        }
    }
}
