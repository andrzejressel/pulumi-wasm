/// Manages a HDInsight HBase Cluster.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
///     let exampleHBaseCluster = h_base_cluster::create(
///         "exampleHBaseCluster",
///         HBaseClusterArgs::builder()
///             .cluster_version("3.6")
///             .component_version(
///                 HBaseClusterComponentVersion::builder().hbase("1.1").build_struct(),
///             )
///             .gateway(
///                 HBaseClusterGateway::builder()
///                     .password("Password123!")
///                     .username("acctestusrgw")
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("example-hdicluster")
///             .resource_group_name("${example.name}")
///             .roles(
///                 HBaseClusterRoles::builder()
///                     .headNode(
///                         HBaseClusterRolesHeadNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .workerNode(
///                         HBaseClusterRolesWorkerNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .targetInstanceCount(3)
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .zookeeperNode(
///                         HBaseClusterRolesZookeeperNode::builder()
///                             .password("AccTestvdSC4daf986!")
///                             .username("acctestusrvm")
///                             .vmSize("Standard_D3_V2")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .storage_accounts(
///                 vec![
///                     HBaseClusterStorageAccount::builder().isDefault(true)
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
/// HDInsight HBase Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:hdinsight/hBaseCluster:HBaseCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.HDInsight/clusters/cluster1
/// ```
///
pub mod h_base_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HBaseClusterArgs {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_version: pulumi_wasm_rust::Output<String>,
        /// A `component_version` block as defined below.
        #[builder(into)]
        pub component_version: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        #[builder(into, default)]
        pub compute_isolation: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        #[builder(into, default)]
        pub disk_encryptions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::hdinsight::HBaseClusterDiskEncryption>>,
        >,
        /// An `extension` block as defined below.
        #[builder(into, default)]
        pub extension: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        #[builder(into)]
        pub gateway: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterGateway,
        >,
        /// Specifies the Azure Region which this HDInsight HBase Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A `metastores` block as defined below.
        #[builder(into, default)]
        pub metastores: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        #[builder(into, default)]
        pub monitor: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight HBase Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A `network` block as defined below.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        #[builder(into, default)]
        pub private_link_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight HBase Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `roles` block as defined below.
        #[builder(into)]
        pub roles: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub security_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterSecurityProfile>,
        >,
        /// A `storage_account_gen2` block as defined below.
        #[builder(into, default)]
        pub storage_account_gen2: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::hdinsight::HBaseClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight HBase Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight HBase Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The minimal supported TLS version. Possible values are 1.0, 1.1 or 1.2. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        #[builder(into, default)]
        pub tls_min_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HBaseClusterResult {
        /// Specifies the Version of HDInsights which should be used for this Cluster. Changing this forces a new resource to be created.
        pub cluster_version: pulumi_wasm_rust::Output<String>,
        /// A `component_version` block as defined below.
        pub component_version: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterComponentVersion,
        >,
        /// A `compute_isolation` block as defined below.
        pub compute_isolation: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterComputeIsolation>,
        >,
        /// One or more `disk_encryption` block as defined below.
        pub disk_encryptions: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::hdinsight::HBaseClusterDiskEncryption>>,
        >,
        /// An `extension` block as defined below.
        pub extension: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterExtension>,
        >,
        /// A `gateway` block as defined below.
        pub gateway: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterGateway,
        >,
        /// The HTTPS Connectivity Endpoint for this HDInsight HBase Cluster.
        pub https_endpoint: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region which this HDInsight HBase Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `metastores` block as defined below.
        pub metastores: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterMetastores>,
        >,
        /// A `monitor` block as defined below.
        pub monitor: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterMonitor>,
        >,
        /// Specifies the name for this HDInsight HBase Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network` block as defined below.
        pub network: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterNetwork>,
        >,
        /// A `private_link_configuration` block as defined below.
        pub private_link_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterPrivateLinkConfiguration>,
        >,
        /// Specifies the name of the Resource Group in which this HDInsight HBase Cluster should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `roles` block as defined below.
        pub roles: pulumi_wasm_rust::Output<
            super::super::types::hdinsight::HBaseClusterRoles,
        >,
        /// A `security_profile` block as defined below. Changing this forces a new resource to be created.
        pub security_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterSecurityProfile>,
        >,
        /// The SSH Connectivity Endpoint for this HDInsight HBase Cluster.
        pub ssh_endpoint: pulumi_wasm_rust::Output<String>,
        /// A `storage_account_gen2` block as defined below.
        pub storage_account_gen2: pulumi_wasm_rust::Output<
            Option<super::super::types::hdinsight::HBaseClusterStorageAccountGen2>,
        >,
        /// One or more `storage_account` block as defined below.
        pub storage_accounts: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::hdinsight::HBaseClusterStorageAccount>>,
        >,
        /// A map of Tags which should be assigned to this HDInsight HBase Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Tier which should be used for this HDInsight HBase Cluster. Possible values are `Standard` or `Premium`. Changing this forces a new resource to be created.
        pub tier: pulumi_wasm_rust::Output<String>,
        /// The minimal supported TLS version. Possible values are 1.0, 1.1 or 1.2. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** Starting on June 30, 2020, Azure HDInsight will enforce TLS 1.2 or later versions for all HTTPS connections. For more information, see [Azure HDInsight TLS 1.2 Enforcement](https://azure.microsoft.com/en-us/updates/azure-hdinsight-tls-12-enforcement/).
        pub tls_min_version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HBaseClusterArgs) -> HBaseClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_version_binding = args.cluster_version.get_inner();
        let component_version_binding = args.component_version.get_inner();
        let compute_isolation_binding = args.compute_isolation.get_inner();
        let disk_encryptions_binding = args.disk_encryptions.get_inner();
        let extension_binding = args.extension.get_inner();
        let gateway_binding = args.gateway.get_inner();
        let location_binding = args.location.get_inner();
        let metastores_binding = args.metastores.get_inner();
        let monitor_binding = args.monitor.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let private_link_configuration_binding = args
            .private_link_configuration
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let roles_binding = args.roles.get_inner();
        let security_profile_binding = args.security_profile.get_inner();
        let storage_account_gen2_binding = args.storage_account_gen2.get_inner();
        let storage_accounts_binding = args.storage_accounts.get_inner();
        let tags_binding = args.tags.get_inner();
        let tier_binding = args.tier.get_inner();
        let tls_min_version_binding = args.tls_min_version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:hdinsight/hBaseCluster:HBaseCluster".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "clusterVersion".into(),
                },
                register_interface::ResultField {
                    name: "componentVersion".into(),
                },
                register_interface::ResultField {
                    name: "computeIsolation".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryptions".into(),
                },
                register_interface::ResultField {
                    name: "extension".into(),
                },
                register_interface::ResultField {
                    name: "gateway".into(),
                },
                register_interface::ResultField {
                    name: "httpsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metastores".into(),
                },
                register_interface::ResultField {
                    name: "monitor".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "privateLinkConfiguration".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "roles".into(),
                },
                register_interface::ResultField {
                    name: "securityProfile".into(),
                },
                register_interface::ResultField {
                    name: "sshEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountGen2".into(),
                },
                register_interface::ResultField {
                    name: "storageAccounts".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
                register_interface::ResultField {
                    name: "tlsMinVersion".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HBaseClusterResult {
            cluster_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterVersion").unwrap(),
            ),
            component_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("componentVersion").unwrap(),
            ),
            compute_isolation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeIsolation").unwrap(),
            ),
            disk_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptions").unwrap(),
            ),
            extension: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extension").unwrap(),
            ),
            gateway: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gateway").unwrap(),
            ),
            https_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpsEndpoint").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metastores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metastores").unwrap(),
            ),
            monitor: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitor").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            private_link_configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateLinkConfiguration").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            roles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roles").unwrap(),
            ),
            security_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityProfile").unwrap(),
            ),
            ssh_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sshEndpoint").unwrap(),
            ),
            storage_account_gen2: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountGen2").unwrap(),
            ),
            storage_accounts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccounts").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
            tls_min_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tlsMinVersion").unwrap(),
            ),
        }
    }
}
