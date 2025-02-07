/// Manages a HDInsight Spark Cluster.
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
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("hdinsightstor")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleContainer = container::create(
///         "exampleContainer",
///         ContainerArgs::builder()
///             .container_access_type("private")
///             .name("hdinsight")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let exampleSparkCluster = spark_cluster::create(
///         "exampleSparkCluster",
///         SparkClusterArgs::builder()
///             .cluster_version("3.6")
///             .component_version(
///                 SparkClusterComponentVersion::builder().spark("2.3").build_struct(),
///             )
///             .gateway(
///                 SparkClusterGateway::builder()
///                     .password("Password123!")
///                     .username("acctestusrgw")
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("example-hdicluster")
///             .resource_group_name("${example.name}")
///             .roles(
///                 SparkClusterRoles::builder()
///                     .headNode(
///                         SparkClusterRolesHeadNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Standard_A3")
///                             .build_struct(),
///                     )
///                     .workerNode(
///                         SparkClusterRolesWorkerNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .targetInstanceCount(3)
///                             .username("acctestusrvm")
///                             .vmSize("Standard_A3")
///                             .build_struct(),
///                     )
///                     .zookeeperNode(
///                         SparkClusterRolesZookeeperNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Medium")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .storage_accounts(
///                 vec![
///                     SparkClusterStorageAccount::builder().isDefault(true)
///                     .storageAccountKey("${exampleAccount.primaryAccessKey}")
///                     .storageContainerId("${exampleContainer.id}").build_struct(),
///                 ],
///             )
///             .tier("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HDInsight Spark Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hdinsight/sparkCluster:SparkCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.HDInsight/clusters/cluster1
/// ```
///
pub mod spark_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SparkClusterArgs {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `component_version` block as defined below.
        #[builder(into)]
        pub component_version: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::SparkClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        #[builder(into, default)]
        pub compute_isolation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        #[builder(into, default)]
        pub disk_encryptions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::hdinsight::SparkClusterDiskEncryption>>,
        >,
        /// Whether encryption in transit is enabled for this Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_in_transit_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `extension` block as defined below.
        #[builder(into, default)]
        pub extension: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        #[builder(into)]
        pub gateway: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::SparkClusterGateway,
        >,
        /// Specifies the Azure Region which this HDInsight Spark Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `metastores` block as defined below.
        #[builder(into, default)]
        pub metastores: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        #[builder(into, default)]
        pub monitor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight Spark Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network` block as defined below.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        #[builder(into, default)]
        pub private_link_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight Spark Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `roles` block as defined below.
        #[builder(into)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::SparkClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub security_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterSecurityProfile>,
        >,
        /// A `storage_account_gen2` block as defined below.
        #[builder(into, default)]
        pub storage_account_gen2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::SparkClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::hdinsight::SparkClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight Spark Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight Spark Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The minimal supported TLS version. Possible values are 1.0, 1.1 or 1.2. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        #[builder(into, default)]
        pub tls_min_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SparkClusterResult {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        pub cluster_version: pulumi_gestalt_rust::Output<String>,
        /// A `component_version` block as defined below.
        pub component_version: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::SparkClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        pub compute_isolation: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        pub disk_encryptions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::hdinsight::SparkClusterDiskEncryption>>,
        >,
        /// Whether encryption in transit is enabled for this Cluster. Changing this forces a new resource to be created.
        pub encryption_in_transit_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `extension` block as defined below.
        pub extension: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        pub gateway: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::SparkClusterGateway,
        >,
        /// The HTTPS Connectivity Endpoint for this HDInsight Spark Cluster.
        pub https_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region which this HDInsight Spark Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `metastores` block as defined below.
        pub metastores: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        pub monitor: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight Spark Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network` block as defined below.
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        pub private_link_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight Spark Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `roles` block as defined below.
        pub roles: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::SparkClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        pub security_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterSecurityProfile>,
        >,
        /// The SSH Connectivity Endpoint for this HDInsight Spark Cluster.
        pub ssh_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `storage_account_gen2` block as defined below.
        pub storage_account_gen2: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::SparkClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::hdinsight::SparkClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight Spark Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight Spark Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        pub tier: pulumi_gestalt_rust::Output<String>,
        /// The minimal supported TLS version. Possible values are 1.0, 1.1 or 1.2. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        pub tls_min_version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SparkClusterArgs,
    ) -> SparkClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cluster_version_binding = args
            .cluster_version
            .get_output(context)
            .get_inner();
        let component_version_binding = args
            .component_version
            .get_output(context)
            .get_inner();
        let compute_isolation_binding = args
            .compute_isolation
            .get_output(context)
            .get_inner();
        let disk_encryptions_binding = args
            .disk_encryptions
            .get_output(context)
            .get_inner();
        let encryption_in_transit_enabled_binding = args
            .encryption_in_transit_enabled
            .get_output(context)
            .get_inner();
        let extension_binding = args.extension.get_output(context).get_inner();
        let gateway_binding = args.gateway.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let metastores_binding = args.metastores.get_output(context).get_inner();
        let monitor_binding = args.monitor.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let private_link_configuration_binding = args
            .private_link_configuration
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let roles_binding = args.roles.get_output(context).get_inner();
        let security_profile_binding = args
            .security_profile
            .get_output(context)
            .get_inner();
        let storage_account_gen2_binding = args
            .storage_account_gen2
            .get_output(context)
            .get_inner();
        let storage_accounts_binding = args
            .storage_accounts
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tier_binding = args.tier.get_output(context).get_inner();
        let tls_min_version_binding = args
            .tls_min_version
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hdinsight/sparkCluster:SparkCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterVersion".into(),
                    value: &cluster_version_binding,
                },
                register_interface::ObjectField {
                    name: "componentVersion".into(),
                    value: &component_version_binding,
                },
                register_interface::ObjectField {
                    name: "computeIsolation".into(),
                    value: &compute_isolation_binding,
                },
                register_interface::ObjectField {
                    name: "diskEncryptions".into(),
                    value: &disk_encryptions_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionInTransitEnabled".into(),
                    value: &encryption_in_transit_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "extension".into(),
                    value: &extension_binding,
                },
                register_interface::ObjectField {
                    name: "gateway".into(),
                    value: &gateway_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "metastores".into(),
                    value: &metastores_binding,
                },
                register_interface::ObjectField {
                    name: "monitor".into(),
                    value: &monitor_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "privateLinkConfiguration".into(),
                    value: &private_link_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "roles".into(),
                    value: &roles_binding,
                },
                register_interface::ObjectField {
                    name: "securityProfile".into(),
                    value: &security_profile_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountGen2".into(),
                    value: &storage_account_gen2_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
                register_interface::ObjectField {
                    name: "tlsMinVersion".into(),
                    value: &tls_min_version_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SparkClusterResult {
            cluster_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterVersion"),
            ),
            component_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("componentVersion"),
            ),
            compute_isolation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("computeIsolation"),
            ),
            disk_encryptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diskEncryptions"),
            ),
            encryption_in_transit_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionInTransitEnabled"),
            ),
            extension: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("extension"),
            ),
            gateway: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gateway"),
            ),
            https_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("httpsEndpoint"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metastores: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metastores"),
            ),
            monitor: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitor"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            private_link_configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateLinkConfiguration"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            roles: pulumi_gestalt_rust::__private::into_domain(o.extract_field("roles")),
            security_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityProfile"),
            ),
            ssh_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sshEndpoint"),
            ),
            storage_account_gen2: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccountGen2"),
            ),
            storage_accounts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageAccounts"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tier: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tier")),
            tls_min_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tlsMinVersion"),
            ),
        }
    }
}
