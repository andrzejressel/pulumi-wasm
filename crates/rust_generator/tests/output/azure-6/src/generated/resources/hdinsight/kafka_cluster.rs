/// Manages a HDInsight Kafka Cluster.
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
///     let exampleKafkaCluster = kafka_cluster::create(
///         "exampleKafkaCluster",
///         KafkaClusterArgs::builder()
///             .cluster_version("4.0")
///             .component_version(
///                 KafkaClusterComponentVersion::builder().kafka("2.1").build_struct(),
///             )
///             .gateway(
///                 KafkaClusterGateway::builder()
///                     .password("Password123!")
///                     .username("acctestusrgw")
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("example-hdicluster")
///             .resource_group_name("${example.name}")
///             .roles(
///                 KafkaClusterRoles::builder()
///                     .headNode(
///                         KafkaClusterRolesHeadNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .workerNode(
///                         KafkaClusterRolesWorkerNode::builder()
///                             .numberOfDisksPerNode(3)
///                             .password("AccTestvdSC4daf986!")
///                             .targetInstanceCount(3)
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .zookeeperNode(
///                         KafkaClusterRolesZookeeperNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .storage_accounts(
///                 vec![
///                     KafkaClusterStorageAccount::builder().isDefault(true)
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
/// HDInsight Kafka Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hdinsight/kafkaCluster:KafkaCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.HDInsight/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod kafka_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KafkaClusterArgs {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `component_version` block as defined below.
        #[builder(into)]
        pub component_version: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::KafkaClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        #[builder(into, default)]
        pub compute_isolation: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        #[builder(into, default)]
        pub disk_encryptions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::hdinsight::KafkaClusterDiskEncryption>>,
        >,
        /// Whether encryption in transit is enabled for this HDInsight Kafka Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub encryption_in_transit_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// An `extension` block as defined below.
        #[builder(into, default)]
        pub extension: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        #[builder(into)]
        pub gateway: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::KafkaClusterGateway,
        >,
        /// Specifies the Azure Region which this HDInsight Kafka Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `metastores` block as defined below.
        #[builder(into, default)]
        pub metastores: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        #[builder(into, default)]
        pub monitor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight Kafka Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `network` block as defined below.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        #[builder(into, default)]
        pub private_link_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight Kafka Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `rest_proxy` block as defined below.
        #[builder(into, default)]
        pub rest_proxy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterRestProxy>,
        >,
        /// A `roles` block as defined below.
        #[builder(into)]
        pub roles: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::hdinsight::KafkaClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub security_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterSecurityProfile>,
        >,
        /// A `storage_account_gen2` block as defined below.
        #[builder(into, default)]
        pub storage_account_gen2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::hdinsight::KafkaClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::hdinsight::KafkaClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight Kafka Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight Kafka Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The minimal supported TLS version. Possible values are `1.0`, `1.1` or `1.2`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub tls_min_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct KafkaClusterResult {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        pub cluster_version: pulumi_gestalt_rust::Output<String>,
        /// A `component_version` block as defined below.
        pub component_version: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::KafkaClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        pub compute_isolation: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        pub disk_encryptions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::hdinsight::KafkaClusterDiskEncryption>>,
        >,
        /// Whether encryption in transit is enabled for this HDInsight Kafka Cluster. Changing this forces a new resource to be created.
        pub encryption_in_transit_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `extension` block as defined below.
        pub extension: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        pub gateway: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::KafkaClusterGateway,
        >,
        /// The HTTPS Connectivity Endpoint for this HDInsight Kafka Cluster.
        pub https_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The Kafka Rest Proxy Endpoint for this HDInsight Kafka Cluster.
        pub kafka_rest_proxy_endpoint: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region which this HDInsight Kafka Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `metastores` block as defined below.
        pub metastores: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        pub monitor: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight Kafka Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network` block as defined below.
        pub network: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        pub private_link_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight Kafka Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `rest_proxy` block as defined below.
        pub rest_proxy: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterRestProxy>,
        >,
        /// A `roles` block as defined below.
        pub roles: pulumi_gestalt_rust::Output<
            super::super::types::hdinsight::KafkaClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        pub security_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterSecurityProfile>,
        >,
        /// The SSH Connectivity Endpoint for this HDInsight Kafka Cluster.
        pub ssh_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `storage_account_gen2` block as defined below.
        pub storage_account_gen2: pulumi_gestalt_rust::Output<
            Option<super::super::types::hdinsight::KafkaClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::hdinsight::KafkaClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight Kafka Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight Kafka Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        pub tier: pulumi_gestalt_rust::Output<String>,
        /// The minimal supported TLS version. Possible values are `1.0`, `1.1` or `1.2`. Changing this forces a new resource to be created.
        pub tls_min_version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KafkaClusterArgs,
    ) -> KafkaClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_version_binding = args.cluster_version.get_output(context);
        let component_version_binding = args.component_version.get_output(context);
        let compute_isolation_binding = args.compute_isolation.get_output(context);
        let disk_encryptions_binding = args.disk_encryptions.get_output(context);
        let encryption_in_transit_enabled_binding = args
            .encryption_in_transit_enabled
            .get_output(context);
        let extension_binding = args.extension.get_output(context);
        let gateway_binding = args.gateway.get_output(context);
        let location_binding = args.location.get_output(context);
        let metastores_binding = args.metastores.get_output(context);
        let monitor_binding = args.monitor.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let private_link_configuration_binding = args
            .private_link_configuration
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rest_proxy_binding = args.rest_proxy.get_output(context);
        let roles_binding = args.roles.get_output(context);
        let security_profile_binding = args.security_profile.get_output(context);
        let storage_account_gen2_binding = args.storage_account_gen2.get_output(context);
        let storage_accounts_binding = args.storage_accounts.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let tier_binding = args.tier.get_output(context);
        let tls_min_version_binding = args.tls_min_version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:hdinsight/kafkaCluster:KafkaCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterVersion".into(),
                    value: cluster_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "componentVersion".into(),
                    value: component_version_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeIsolation".into(),
                    value: compute_isolation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diskEncryptions".into(),
                    value: disk_encryptions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "encryptionInTransitEnabled".into(),
                    value: encryption_in_transit_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extension".into(),
                    value: extension_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gateway".into(),
                    value: gateway_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metastores".into(),
                    value: metastores_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "monitor".into(),
                    value: monitor_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateLinkConfiguration".into(),
                    value: private_link_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restProxy".into(),
                    value: rest_proxy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roles".into(),
                    value: roles_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "securityProfile".into(),
                    value: security_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccountGen2".into(),
                    value: storage_account_gen2_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccounts".into(),
                    value: storage_accounts_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tier".into(),
                    value: tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tlsMinVersion".into(),
                    value: tls_min_version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        KafkaClusterResult {
            cluster_version: o.get_field("clusterVersion"),
            component_version: o.get_field("componentVersion"),
            compute_isolation: o.get_field("computeIsolation"),
            disk_encryptions: o.get_field("diskEncryptions"),
            encryption_in_transit_enabled: o.get_field("encryptionInTransitEnabled"),
            extension: o.get_field("extension"),
            gateway: o.get_field("gateway"),
            https_endpoint: o.get_field("httpsEndpoint"),
            kafka_rest_proxy_endpoint: o.get_field("kafkaRestProxyEndpoint"),
            location: o.get_field("location"),
            metastores: o.get_field("metastores"),
            monitor: o.get_field("monitor"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            private_link_configuration: o.get_field("privateLinkConfiguration"),
            resource_group_name: o.get_field("resourceGroupName"),
            rest_proxy: o.get_field("restProxy"),
            roles: o.get_field("roles"),
            security_profile: o.get_field("securityProfile"),
            ssh_endpoint: o.get_field("sshEndpoint"),
            storage_account_gen2: o.get_field("storageAccountGen2"),
            storage_accounts: o.get_field("storageAccounts"),
            tags: o.get_field("tags"),
            tier: o.get_field("tier"),
            tls_min_version: o.get_field("tlsMinVersion"),
        }
    }
}
