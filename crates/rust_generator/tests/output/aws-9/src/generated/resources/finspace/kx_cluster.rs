/// Resource for managing an AWS FinSpace Kx Cluster.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:finspace:KxCluster
///     properties:
///       name: my-tf-kx-cluster
///       environmentId: ${exampleAwsFinspaceKxEnvironment.id}
///       type: HDB
///       releaseLabel: '1.0'
///       azMode: SINGLE
///       availabilityZoneId: use1-az2
///       capacityConfiguration:
///         nodeType: kx.s.2xlarge
///         nodeCount: 2
///       vpcConfiguration:
///         vpcId: ${test.id}
///         securityGroupIds:
///           - ${exampleAwsSecurityGroup.id}
///         subnetIds:
///           - ${exampleAwsSubnet.id}
///         ipAddressType: IP_V4
///       cacheStorageConfigurations:
///         - type: CACHE_1000
///           size: 1200
///       databases:
///         - databaseName: ${exampleAwsFinspaceKxDatabase.name}
///           cacheConfiguration:
///             - cacheType: CACHE_1000
///               dbPaths: /
///       code:
///         s3Bucket: ${testAwsS3Bucket.id}
///         s3Key: ${object.key}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an AWS FinSpace Kx Cluster using the `id` (environment ID and cluster name, comma-delimited). For example:
///
/// ```sh
/// $ pulumi import aws:finspace/kxCluster:KxCluster example n3ceo7wqxoxcti5tujqwzs,my-tf-kx-cluster
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kx_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KxClusterArgs {
        /// Configuration based on which FinSpace will scale in or scale out nodes in your cluster. See auto_scaling_configuration.
        #[builder(into, default)]
        pub auto_scaling_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::finspace::KxClusterAutoScalingConfiguration>,
        >,
        /// The availability zone identifiers for the requested regions. Required when `az_mode` is set to SINGLE.
        #[builder(into, default)]
        pub availability_zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * SINGLE - Assigns one availability zone per cluster.
        /// * MULTI - Assigns all the availability zones per cluster.
        #[builder(into)]
        pub az_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configurations for a read only cache storage associated with a cluster. This cache will be stored as an FSx Lustre that reads from the S3 store. See cache_storage_configuration.
        #[builder(into, default)]
        pub cache_storage_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::finspace::KxClusterCacheStorageConfiguration>,
            >,
        >,
        /// Structure for the metadata of a cluster. Includes information like the CPUs needed, memory of instances, and number of instances. See capacity_configuration.
        #[builder(into, default)]
        pub capacity_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::finspace::KxClusterCapacityConfiguration>,
        >,
        /// Details of the custom code that you want to use inside a cluster when analyzing data. Consists of the S3 source bucket, location, object version, and the relative path from where the custom code is loaded into the cluster. See code.
        #[builder(into, default)]
        pub code: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::finspace::KxClusterCode>,
        >,
        /// List of key-value pairs to make available inside the cluster.
        #[builder(into, default)]
        pub command_line_arguments: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// KX database that will be available for querying. Defined below.
        #[builder(into, default)]
        pub databases: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::finspace::KxClusterDatabase>>,
        >,
        /// Description of the cluster.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique identifier for the KX environment.
        #[builder(into)]
        pub environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An IAM role that defines a set of permissions associated with a cluster. These permissions are assumed when a cluster attempts to access another cluster.
        #[builder(into, default)]
        pub execution_role: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Path to Q program that will be run at launch of a cluster. This is a relative path within .zip file that contains the custom code, which will be loaded on the cluster. It must include the file name itself. For example, somedir/init.q.
        #[builder(into, default)]
        pub initialization_script: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Unique name for the cluster that you want to create.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Version of FinSpace Managed kdb to run.
        #[builder(into)]
        pub release_label: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Size and type of the temporary storage that is used to hold data during the savedown process. This parameter is required when you choose `type` as RDB. All the data written to this storage space is lost when the cluster node is restarted. See savedown_storage_configuration.
        #[builder(into, default)]
        pub savedown_storage_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::finspace::KxClusterSavedownStorageConfiguration>,
        >,
        /// The structure that stores the configuration details of a scaling group.
        #[builder(into, default)]
        pub scaling_group_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::finspace::KxClusterScalingGroupConfiguration>,
        >,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A configuration to store Tickerplant logs. It consists of a list of volumes that will be mounted to your cluster. For the cluster type Tickerplant , the location of the TP volume on the cluster will be available by using the global variable .aws.tp_log_path.
        #[builder(into, default)]
        pub tickerplant_log_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::finspace::KxClusterTickerplantLogConfiguration>,
            >,
        >,
        /// Type of KDB database. The following types are available:
        /// * HDB - Historical Database. The data is only accessible with read-only permissions from one of the FinSpace managed KX databases mounted to the cluster.
        /// * RDB - Realtime Database. This type of database captures all the data from a ticker plant and stores it in memory until the end of day, after which it writes all of its data to a disk and reloads the HDB. This cluster type requires local storage for temporary storage of data during the savedown process. If you specify this field in your request, you must provide the `savedownStorageConfiguration` parameter.
        /// * GATEWAY - A gateway cluster allows you to access data across processes in kdb systems. It allows you to create your own routing logic using the initialization scripts and custom code. This type of cluster does not require a  writable local storage.
        /// * GP - A general purpose cluster allows you to quickly iterate on code during development by granting greater access to system commands and enabling a fast reload of custom code. This cluster type can optionally mount databases including cache and savedown storage. For this cluster type, the node count is fixed at 1. It does not support autoscaling and supports only `SINGLE` AZ mode.
        /// * Tickerplant – A tickerplant cluster allows you to subscribe to feed handlers based on IAM permissions. It can publish to RDBs, other Tickerplants, and real-time subscribers (RTS). Tickerplants can persist messages to log, which is readable by any RDB environment. It supports only single-node that is only one kdb process.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration details about the network where the Privatelink endpoint of the cluster resides. See vpc_configuration.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub vpc_configuration: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::finspace::KxClusterVpcConfiguration,
        >,
    }
    #[allow(dead_code)]
    pub struct KxClusterResult {
        /// Amazon Resource Name (ARN) identifier of the KX cluster.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration based on which FinSpace will scale in or scale out nodes in your cluster. See auto_scaling_configuration.
        pub auto_scaling_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::finspace::KxClusterAutoScalingConfiguration>,
        >,
        /// The availability zone identifiers for the requested regions. Required when `az_mode` is set to SINGLE.
        pub availability_zone_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The number of availability zones you want to assign per cluster. This can be one of the following:
        /// * SINGLE - Assigns one availability zone per cluster.
        /// * MULTI - Assigns all the availability zones per cluster.
        pub az_mode: pulumi_gestalt_rust::Output<String>,
        /// Configurations for a read only cache storage associated with a cluster. This cache will be stored as an FSx Lustre that reads from the S3 store. See cache_storage_configuration.
        pub cache_storage_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::finspace::KxClusterCacheStorageConfiguration>,
            >,
        >,
        /// Structure for the metadata of a cluster. Includes information like the CPUs needed, memory of instances, and number of instances. See capacity_configuration.
        pub capacity_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::finspace::KxClusterCapacityConfiguration>,
        >,
        /// Details of the custom code that you want to use inside a cluster when analyzing data. Consists of the S3 source bucket, location, object version, and the relative path from where the custom code is loaded into the cluster. See code.
        pub code: pulumi_gestalt_rust::Output<
            Option<super::super::types::finspace::KxClusterCode>,
        >,
        /// List of key-value pairs to make available inside the cluster.
        pub command_line_arguments: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Timestamp at which the cluster is created in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub created_timestamp: pulumi_gestalt_rust::Output<String>,
        /// KX database that will be available for querying. Defined below.
        pub databases: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::finspace::KxClusterDatabase>>,
        >,
        /// Description of the cluster.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Unique identifier for the KX environment.
        pub environment_id: pulumi_gestalt_rust::Output<String>,
        /// An IAM role that defines a set of permissions associated with a cluster. These permissions are assumed when a cluster attempts to access another cluster.
        pub execution_role: pulumi_gestalt_rust::Output<Option<String>>,
        /// Path to Q program that will be run at launch of a cluster. This is a relative path within .zip file that contains the custom code, which will be loaded on the cluster. It must include the file name itself. For example, somedir/init.q.
        pub initialization_script: pulumi_gestalt_rust::Output<Option<String>>,
        /// Last timestamp at which the cluster was updated in FinSpace. Value determined as epoch time in seconds. For example, the value for Monday, November 1, 2021 12:00:00 PM UTC is specified as 1635768000.
        pub last_modified_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Unique name for the cluster that you want to create.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Version of FinSpace Managed kdb to run.
        pub release_label: pulumi_gestalt_rust::Output<String>,
        /// Size and type of the temporary storage that is used to hold data during the savedown process. This parameter is required when you choose `type` as RDB. All the data written to this storage space is lost when the cluster node is restarted. See savedown_storage_configuration.
        pub savedown_storage_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::finspace::KxClusterSavedownStorageConfiguration>,
        >,
        /// The structure that stores the configuration details of a scaling group.
        pub scaling_group_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::finspace::KxClusterScalingGroupConfiguration>,
        >,
        pub status: pulumi_gestalt_rust::Output<String>,
        pub status_reason: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A configuration to store Tickerplant logs. It consists of a list of volumes that will be mounted to your cluster. For the cluster type Tickerplant , the location of the TP volume on the cluster will be available by using the global variable .aws.tp_log_path.
        pub tickerplant_log_configurations: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::finspace::KxClusterTickerplantLogConfiguration>,
            >,
        >,
        /// Type of KDB database. The following types are available:
        /// * HDB - Historical Database. The data is only accessible with read-only permissions from one of the FinSpace managed KX databases mounted to the cluster.
        /// * RDB - Realtime Database. This type of database captures all the data from a ticker plant and stores it in memory until the end of day, after which it writes all of its data to a disk and reloads the HDB. This cluster type requires local storage for temporary storage of data during the savedown process. If you specify this field in your request, you must provide the `savedownStorageConfiguration` parameter.
        /// * GATEWAY - A gateway cluster allows you to access data across processes in kdb systems. It allows you to create your own routing logic using the initialization scripts and custom code. This type of cluster does not require a  writable local storage.
        /// * GP - A general purpose cluster allows you to quickly iterate on code during development by granting greater access to system commands and enabling a fast reload of custom code. This cluster type can optionally mount databases including cache and savedown storage. For this cluster type, the node count is fixed at 1. It does not support autoscaling and supports only `SINGLE` AZ mode.
        /// * Tickerplant – A tickerplant cluster allows you to subscribe to feed handlers based on IAM permissions. It can publish to RDBs, other Tickerplants, and real-time subscribers (RTS). Tickerplants can persist messages to log, which is readable by any RDB environment. It supports only single-node that is only one kdb process.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Configuration details about the network where the Privatelink endpoint of the cluster resides. See vpc_configuration.
        ///
        /// The following arguments are optional:
        pub vpc_configuration: pulumi_gestalt_rust::Output<
            super::super::types::finspace::KxClusterVpcConfiguration,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: KxClusterArgs,
    ) -> KxClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_scaling_configuration_binding = args
            .auto_scaling_configuration
            .get_output(context)
            .get_inner();
        let availability_zone_id_binding = args
            .availability_zone_id
            .get_output(context)
            .get_inner();
        let az_mode_binding = args.az_mode.get_output(context).get_inner();
        let cache_storage_configurations_binding = args
            .cache_storage_configurations
            .get_output(context)
            .get_inner();
        let capacity_configuration_binding = args
            .capacity_configuration
            .get_output(context)
            .get_inner();
        let code_binding = args.code.get_output(context).get_inner();
        let command_line_arguments_binding = args
            .command_line_arguments
            .get_output(context)
            .get_inner();
        let databases_binding = args.databases.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let environment_id_binding = args.environment_id.get_output(context).get_inner();
        let execution_role_binding = args.execution_role.get_output(context).get_inner();
        let initialization_script_binding = args
            .initialization_script
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let release_label_binding = args.release_label.get_output(context).get_inner();
        let savedown_storage_configuration_binding = args
            .savedown_storage_configuration
            .get_output(context)
            .get_inner();
        let scaling_group_configuration_binding = args
            .scaling_group_configuration
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tickerplant_log_configurations_binding = args
            .tickerplant_log_configurations
            .get_output(context)
            .get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let vpc_configuration_binding = args
            .vpc_configuration
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:finspace/kxCluster:KxCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoScalingConfiguration".into(),
                    value: &auto_scaling_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZoneId".into(),
                    value: &availability_zone_id_binding,
                },
                register_interface::ObjectField {
                    name: "azMode".into(),
                    value: &az_mode_binding,
                },
                register_interface::ObjectField {
                    name: "cacheStorageConfigurations".into(),
                    value: &cache_storage_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "capacityConfiguration".into(),
                    value: &capacity_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "code".into(),
                    value: &code_binding,
                },
                register_interface::ObjectField {
                    name: "commandLineArguments".into(),
                    value: &command_line_arguments_binding,
                },
                register_interface::ObjectField {
                    name: "databases".into(),
                    value: &databases_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "environmentId".into(),
                    value: &environment_id_binding,
                },
                register_interface::ObjectField {
                    name: "executionRole".into(),
                    value: &execution_role_binding,
                },
                register_interface::ObjectField {
                    name: "initializationScript".into(),
                    value: &initialization_script_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "releaseLabel".into(),
                    value: &release_label_binding,
                },
                register_interface::ObjectField {
                    name: "savedownStorageConfiguration".into(),
                    value: &savedown_storage_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "scalingGroupConfiguration".into(),
                    value: &scaling_group_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tickerplantLogConfigurations".into(),
                    value: &tickerplant_log_configurations_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "vpcConfiguration".into(),
                    value: &vpc_configuration_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        KxClusterResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_scaling_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoScalingConfiguration"),
            ),
            availability_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZoneId"),
            ),
            az_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azMode"),
            ),
            cache_storage_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("cacheStorageConfigurations"),
            ),
            capacity_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("capacityConfiguration"),
            ),
            code: pulumi_gestalt_rust::__private::into_domain(o.extract_field("code")),
            command_line_arguments: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("commandLineArguments"),
            ),
            created_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTimestamp"),
            ),
            databases: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databases"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            environment_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("environmentId"),
            ),
            execution_role: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("executionRole"),
            ),
            initialization_script: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("initializationScript"),
            ),
            last_modified_timestamp: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastModifiedTimestamp"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            release_label: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseLabel"),
            ),
            savedown_storage_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("savedownStorageConfiguration"),
            ),
            scaling_group_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("scalingGroupConfiguration"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            status_reason: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statusReason"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            tickerplant_log_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tickerplantLogConfigurations"),
            ),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
            vpc_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcConfiguration"),
            ),
        }
    }
}
