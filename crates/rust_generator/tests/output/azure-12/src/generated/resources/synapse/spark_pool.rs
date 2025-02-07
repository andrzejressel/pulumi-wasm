/// Manages a Synapse Spark Pool.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageacc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///       accountKind: StorageV2
///       isHnsEnabled: 'true'
///   exampleDataLakeGen2Filesystem:
///     type: azure:storage:DataLakeGen2Filesystem
///     name: example
///     properties:
///       name: example
///       storageAccountId: ${exampleAccount.id}
///   exampleWorkspace:
///     type: azure:synapse:Workspace
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       storageDataLakeGen2FilesystemId: ${exampleDataLakeGen2Filesystem.id}
///       sqlAdministratorLogin: sqladminuser
///       sqlAdministratorLoginPassword: H@Sh1CoR3!
///       identity:
///         type: SystemAssigned
///   exampleSparkPool:
///     type: azure:synapse:SparkPool
///     name: example
///     properties:
///       name: example
///       synapseWorkspaceId: ${exampleWorkspace.id}
///       nodeSizeFamily: MemoryOptimized
///       nodeSize: Small
///       cacheSize: 100
///       autoScale:
///         maxNodeCount: 50
///         minNodeCount: 3
///       autoPause:
///         delayInMinutes: 15
///       libraryRequirement:
///         content: |
///           appnope==0.1.0
///           beautifulsoup4==4.6.3
///         filename: requirements.txt
///       sparkConfig:
///         content: |
///           spark.shuffle.spill                true
///         filename: config.txt
///       tags:
///         ENV: Production
/// ```
///
/// ## Import
///
/// Synapse Spark Pool can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:synapse/sparkPool:SparkPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Synapse/workspaces/workspace1/bigDataPools/sparkPool1
/// ```
///
pub mod spark_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SparkPoolArgs {
        /// An `auto_pause` block as defined below.
        #[builder(into, default)]
        pub auto_pause: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::SparkPoolAutoPause>,
        >,
        /// An `auto_scale` block as defined below. Exactly one of `node_count` or `auto_scale` must be specified.
        #[builder(into, default)]
        pub auto_scale: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::SparkPoolAutoScale>,
        >,
        /// The cache size in the Spark Pool.
        #[builder(into, default)]
        pub cache_size: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates whether compute isolation is enabled or not. Defaults to `false`.
        #[builder(into, default)]
        pub compute_isolation_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub dynamic_executor_allocation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub library_requirement: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::SparkPoolLibraryRequirement>,
        >,
        #[builder(into, default)]
        pub max_executors: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub min_executors: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The name which should be used for this Synapse Spark Pool. Changing this forces a new Synapse Spark Pool to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of nodes in the Spark Pool. Exactly one of `node_count` or `auto_scale` must be specified.
        #[builder(into, default)]
        pub node_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The level of node in the Spark Pool. Possible values are `Small`, `Medium`, `Large`, `None`, `XLarge`, `XXLarge` and `XXXLarge`.
        #[builder(into)]
        pub node_size: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The kind of nodes that the Spark Pool provides. Possible values are `HardwareAcceleratedFPGA`, `HardwareAcceleratedGPU`, `MemoryOptimized`, and `None`.
        #[builder(into)]
        pub node_size_family: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub session_level_packages_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        #[builder(into, default)]
        pub spark_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::synapse::SparkPoolSparkConfig>,
        >,
        #[builder(into, default)]
        pub spark_events_folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub spark_log_folder: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub spark_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Synapse Workspace where the Synapse Spark Pool should exist. Changing this forces a new Synapse Spark Pool to be created.
        #[builder(into)]
        pub synapse_workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct SparkPoolResult {
        /// An `auto_pause` block as defined below.
        pub auto_pause: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::SparkPoolAutoPause>,
        >,
        /// An `auto_scale` block as defined below. Exactly one of `node_count` or `auto_scale` must be specified.
        pub auto_scale: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::SparkPoolAutoScale>,
        >,
        /// The cache size in the Spark Pool.
        pub cache_size: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Indicates whether compute isolation is enabled or not. Defaults to `false`.
        pub compute_isolation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub dynamic_executor_allocation_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        pub library_requirement: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::SparkPoolLibraryRequirement>,
        >,
        pub max_executors: pulumi_gestalt_rust::Output<Option<i32>>,
        pub min_executors: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The name which should be used for this Synapse Spark Pool. Changing this forces a new Synapse Spark Pool to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of nodes in the Spark Pool. Exactly one of `node_count` or `auto_scale` must be specified.
        pub node_count: pulumi_gestalt_rust::Output<i32>,
        /// The level of node in the Spark Pool. Possible values are `Small`, `Medium`, `Large`, `None`, `XLarge`, `XXLarge` and `XXXLarge`.
        pub node_size: pulumi_gestalt_rust::Output<String>,
        /// The kind of nodes that the Spark Pool provides. Possible values are `HardwareAcceleratedFPGA`, `HardwareAcceleratedGPU`, `MemoryOptimized`, and `None`.
        pub node_size_family: pulumi_gestalt_rust::Output<String>,
        pub session_level_packages_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        pub spark_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::synapse::SparkPoolSparkConfig>,
        >,
        pub spark_events_folder: pulumi_gestalt_rust::Output<Option<String>>,
        pub spark_log_folder: pulumi_gestalt_rust::Output<Option<String>>,
        pub spark_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Synapse Workspace where the Synapse Spark Pool should exist. Changing this forces a new Synapse Spark Pool to be created.
        pub synapse_workspace_id: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SparkPoolArgs,
    ) -> SparkPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_pause_binding = args.auto_pause.get_output(context).get_inner();
        let auto_scale_binding = args.auto_scale.get_output(context).get_inner();
        let cache_size_binding = args.cache_size.get_output(context).get_inner();
        let compute_isolation_enabled_binding = args
            .compute_isolation_enabled
            .get_output(context)
            .get_inner();
        let dynamic_executor_allocation_enabled_binding = args
            .dynamic_executor_allocation_enabled
            .get_output(context)
            .get_inner();
        let library_requirement_binding = args
            .library_requirement
            .get_output(context)
            .get_inner();
        let max_executors_binding = args.max_executors.get_output(context).get_inner();
        let min_executors_binding = args.min_executors.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_count_binding = args.node_count.get_output(context).get_inner();
        let node_size_binding = args.node_size.get_output(context).get_inner();
        let node_size_family_binding = args
            .node_size_family
            .get_output(context)
            .get_inner();
        let session_level_packages_enabled_binding = args
            .session_level_packages_enabled
            .get_output(context)
            .get_inner();
        let spark_config_binding = args.spark_config.get_output(context).get_inner();
        let spark_events_folder_binding = args
            .spark_events_folder
            .get_output(context)
            .get_inner();
        let spark_log_folder_binding = args
            .spark_log_folder
            .get_output(context)
            .get_inner();
        let spark_version_binding = args.spark_version.get_output(context).get_inner();
        let synapse_workspace_id_binding = args
            .synapse_workspace_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:synapse/sparkPool:SparkPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoPause".into(),
                    value: &auto_pause_binding,
                },
                register_interface::ObjectField {
                    name: "autoScale".into(),
                    value: &auto_scale_binding,
                },
                register_interface::ObjectField {
                    name: "cacheSize".into(),
                    value: &cache_size_binding,
                },
                register_interface::ObjectField {
                    name: "computeIsolationEnabled".into(),
                    value: &compute_isolation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "dynamicExecutorAllocationEnabled".into(),
                    value: &dynamic_executor_allocation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "libraryRequirement".into(),
                    value: &library_requirement_binding,
                },
                register_interface::ObjectField {
                    name: "maxExecutors".into(),
                    value: &max_executors_binding,
                },
                register_interface::ObjectField {
                    name: "minExecutors".into(),
                    value: &min_executors_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeCount".into(),
                    value: &node_count_binding,
                },
                register_interface::ObjectField {
                    name: "nodeSize".into(),
                    value: &node_size_binding,
                },
                register_interface::ObjectField {
                    name: "nodeSizeFamily".into(),
                    value: &node_size_family_binding,
                },
                register_interface::ObjectField {
                    name: "sessionLevelPackagesEnabled".into(),
                    value: &session_level_packages_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sparkConfig".into(),
                    value: &spark_config_binding,
                },
                register_interface::ObjectField {
                    name: "sparkEventsFolder".into(),
                    value: &spark_events_folder_binding,
                },
                register_interface::ObjectField {
                    name: "sparkLogFolder".into(),
                    value: &spark_log_folder_binding,
                },
                register_interface::ObjectField {
                    name: "sparkVersion".into(),
                    value: &spark_version_binding,
                },
                register_interface::ObjectField {
                    name: "synapseWorkspaceId".into(),
                    value: &synapse_workspace_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SparkPoolResult {
            auto_pause: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoPause"),
            ),
            auto_scale: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScale"),
            ),
            cache_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheSize"),
            ),
            compute_isolation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeIsolationEnabled"),
            ),
            dynamic_executor_allocation_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dynamicExecutorAllocationEnabled"),
            ),
            library_requirement: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("libraryRequirement"),
            ),
            max_executors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxExecutors"),
            ),
            min_executors: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minExecutors"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            node_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeCount"),
            ),
            node_size: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeSize"),
            ),
            node_size_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeSizeFamily"),
            ),
            session_level_packages_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sessionLevelPackagesEnabled"),
            ),
            spark_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkConfig"),
            ),
            spark_events_folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkEventsFolder"),
            ),
            spark_log_folder: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkLogFolder"),
            ),
            spark_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sparkVersion"),
            ),
            synapse_workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("synapseWorkspaceId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
