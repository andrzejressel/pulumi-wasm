pub mod get_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// The location (zone or region) this cluster has been
        /// created in. One of `location`, `region`, `zone`, or a provider-level `zone` must
        /// be specified.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub addons_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterAddonsConfig>,
        >,
        pub allow_net_admin: pulumi_wasm_rust::Output<bool>,
        pub authenticator_groups_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterAuthenticatorGroupsConfig,
            >,
        >,
        pub binary_authorizations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterBinaryAuthorization>,
        >,
        pub cluster_autoscalings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterClusterAutoscaling>,
        >,
        pub cluster_ipv4_cidr: pulumi_wasm_rust::Output<String>,
        pub cluster_telemetries: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterClusterTelemetry>,
        >,
        pub confidential_nodes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterConfidentialNode>,
        >,
        pub control_plane_endpoints_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterControlPlaneEndpointsConfig,
            >,
        >,
        pub cost_management_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterCostManagementConfig>,
        >,
        pub database_encryptions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterDatabaseEncryption>,
        >,
        pub datapath_provider: pulumi_wasm_rust::Output<String>,
        pub default_max_pods_per_node: pulumi_wasm_rust::Output<i32>,
        pub default_snat_statuses: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterDefaultSnatStatus>,
        >,
        pub deletion_protection: pulumi_wasm_rust::Output<bool>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub dns_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterDnsConfig>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_autopilot: pulumi_wasm_rust::Output<bool>,
        pub enable_cilium_clusterwide_network_policy: pulumi_wasm_rust::Output<bool>,
        pub enable_fqdn_network_policy: pulumi_wasm_rust::Output<bool>,
        pub enable_intranode_visibility: pulumi_wasm_rust::Output<bool>,
        pub enable_k8s_beta_apis: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterEnableK8SBetaApi>,
        >,
        pub enable_kubernetes_alpha: pulumi_wasm_rust::Output<bool>,
        pub enable_l4_ilb_subsetting: pulumi_wasm_rust::Output<bool>,
        pub enable_legacy_abac: pulumi_wasm_rust::Output<bool>,
        pub enable_multi_networking: pulumi_wasm_rust::Output<bool>,
        pub enable_shielded_nodes: pulumi_wasm_rust::Output<bool>,
        pub enable_tpu: pulumi_wasm_rust::Output<bool>,
        pub endpoint: pulumi_wasm_rust::Output<String>,
        pub enterprise_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterEnterpriseConfig>,
        >,
        pub fleets: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterFleet>,
        >,
        pub gateway_api_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterGatewayApiConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identity_service_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterIdentityServiceConfig>,
        >,
        pub initial_node_count: pulumi_wasm_rust::Output<i32>,
        pub ip_allocation_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterIpAllocationPolicy>,
        >,
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        pub logging_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterLoggingConfig>,
        >,
        pub logging_service: pulumi_wasm_rust::Output<String>,
        pub maintenance_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterMaintenancePolicy>,
        >,
        pub master_authorized_networks_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterMasterAuthorizedNetworksConfig,
            >,
        >,
        pub master_auths: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterMasterAuth>,
        >,
        pub master_version: pulumi_wasm_rust::Output<String>,
        pub mesh_certificates: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterMeshCertificate>,
        >,
        pub min_master_version: pulumi_wasm_rust::Output<String>,
        pub monitoring_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterMonitoringConfig>,
        >,
        pub monitoring_service: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network: pulumi_wasm_rust::Output<String>,
        pub network_policies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNetworkPolicy>,
        >,
        pub networking_mode: pulumi_wasm_rust::Output<String>,
        pub node_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodeConfig>,
        >,
        pub node_locations: pulumi_wasm_rust::Output<Vec<String>>,
        pub node_pool_auto_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePoolAutoConfig>,
        >,
        pub node_pool_defaults: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePoolDefault>,
        >,
        pub node_pools: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePool>,
        >,
        pub node_version: pulumi_wasm_rust::Output<String>,
        pub notification_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterNotificationConfig>,
        >,
        pub operation: pulumi_wasm_rust::Output<String>,
        pub pod_security_policy_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterPodSecurityPolicyConfig>,
        >,
        pub private_cluster_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterPrivateClusterConfig>,
        >,
        pub private_ipv6_google_access: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub protect_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterProtectConfig>,
        >,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub release_channels: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterReleaseChannel>,
        >,
        pub remove_default_node_pool: pulumi_wasm_rust::Output<bool>,
        pub resource_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_usage_export_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterResourceUsageExportConfig,
            >,
        >,
        pub secret_manager_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterSecretManagerConfig>,
        >,
        pub security_posture_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterSecurityPostureConfig>,
        >,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub service_external_ips_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterServiceExternalIpsConfig,
            >,
        >,
        pub services_ipv4_cidr: pulumi_wasm_rust::Output<String>,
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        pub tpu_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterTpuConfig>,
        >,
        pub tpu_ipv4_cidr_block: pulumi_wasm_rust::Output<String>,
        pub user_managed_keys_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterUserManagedKeysConfig>,
        >,
        pub vertical_pod_autoscalings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterVerticalPodAutoscaling>,
        >,
        pub workload_alts_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterWorkloadAltsConfig>,
        >,
        pub workload_identity_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::container::GetClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:container/getCluster:getCluster".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addonsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "allowNetAdmin".into(),
                },
                register_interface::ResultField {
                    name: "authenticatorGroupsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "binaryAuthorizations".into(),
                },
                register_interface::ResultField {
                    name: "clusterAutoscalings".into(),
                },
                register_interface::ResultField {
                    name: "clusterIpv4Cidr".into(),
                },
                register_interface::ResultField {
                    name: "clusterTelemetries".into(),
                },
                register_interface::ResultField {
                    name: "confidentialNodes".into(),
                },
                register_interface::ResultField {
                    name: "controlPlaneEndpointsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "costManagementConfigs".into(),
                },
                register_interface::ResultField {
                    name: "databaseEncryptions".into(),
                },
                register_interface::ResultField {
                    name: "datapathProvider".into(),
                },
                register_interface::ResultField {
                    name: "defaultMaxPodsPerNode".into(),
                },
                register_interface::ResultField {
                    name: "defaultSnatStatuses".into(),
                },
                register_interface::ResultField {
                    name: "deletionProtection".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "dnsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "enableAutopilot".into(),
                },
                register_interface::ResultField {
                    name: "enableCiliumClusterwideNetworkPolicy".into(),
                },
                register_interface::ResultField {
                    name: "enableFqdnNetworkPolicy".into(),
                },
                register_interface::ResultField {
                    name: "enableIntranodeVisibility".into(),
                },
                register_interface::ResultField {
                    name: "enableK8sBetaApis".into(),
                },
                register_interface::ResultField {
                    name: "enableKubernetesAlpha".into(),
                },
                register_interface::ResultField {
                    name: "enableL4IlbSubsetting".into(),
                },
                register_interface::ResultField {
                    name: "enableLegacyAbac".into(),
                },
                register_interface::ResultField {
                    name: "enableMultiNetworking".into(),
                },
                register_interface::ResultField {
                    name: "enableShieldedNodes".into(),
                },
                register_interface::ResultField {
                    name: "enableTpu".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "enterpriseConfigs".into(),
                },
                register_interface::ResultField {
                    name: "fleets".into(),
                },
                register_interface::ResultField {
                    name: "gatewayApiConfigs".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identityServiceConfigs".into(),
                },
                register_interface::ResultField {
                    name: "initialNodeCount".into(),
                },
                register_interface::ResultField {
                    name: "ipAllocationPolicies".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "loggingConfigs".into(),
                },
                register_interface::ResultField {
                    name: "loggingService".into(),
                },
                register_interface::ResultField {
                    name: "maintenancePolicies".into(),
                },
                register_interface::ResultField {
                    name: "masterAuthorizedNetworksConfigs".into(),
                },
                register_interface::ResultField {
                    name: "masterAuths".into(),
                },
                register_interface::ResultField {
                    name: "masterVersion".into(),
                },
                register_interface::ResultField {
                    name: "meshCertificates".into(),
                },
                register_interface::ResultField {
                    name: "minMasterVersion".into(),
                },
                register_interface::ResultField {
                    name: "monitoringConfigs".into(),
                },
                register_interface::ResultField {
                    name: "monitoringService".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkPolicies".into(),
                },
                register_interface::ResultField {
                    name: "networkingMode".into(),
                },
                register_interface::ResultField {
                    name: "nodeConfigs".into(),
                },
                register_interface::ResultField {
                    name: "nodeLocations".into(),
                },
                register_interface::ResultField {
                    name: "nodePoolAutoConfigs".into(),
                },
                register_interface::ResultField {
                    name: "nodePoolDefaults".into(),
                },
                register_interface::ResultField {
                    name: "nodePools".into(),
                },
                register_interface::ResultField {
                    name: "nodeVersion".into(),
                },
                register_interface::ResultField {
                    name: "notificationConfigs".into(),
                },
                register_interface::ResultField {
                    name: "operation".into(),
                },
                register_interface::ResultField {
                    name: "podSecurityPolicyConfigs".into(),
                },
                register_interface::ResultField {
                    name: "privateClusterConfigs".into(),
                },
                register_interface::ResultField {
                    name: "privateIpv6GoogleAccess".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "protectConfigs".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "releaseChannels".into(),
                },
                register_interface::ResultField {
                    name: "removeDefaultNodePool".into(),
                },
                register_interface::ResultField {
                    name: "resourceLabels".into(),
                },
                register_interface::ResultField {
                    name: "resourceUsageExportConfigs".into(),
                },
                register_interface::ResultField {
                    name: "secretManagerConfigs".into(),
                },
                register_interface::ResultField {
                    name: "securityPostureConfigs".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "serviceExternalIpsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "servicesIpv4Cidr".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "tpuConfigs".into(),
                },
                register_interface::ResultField {
                    name: "tpuIpv4CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "userManagedKeysConfigs".into(),
                },
                register_interface::ResultField {
                    name: "verticalPodAutoscalings".into(),
                },
                register_interface::ResultField {
                    name: "workloadAltsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "workloadIdentityConfigs".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetClusterResult {
            addons_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addonsConfigs").unwrap(),
            ),
            allow_net_admin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowNetAdmin").unwrap(),
            ),
            authenticator_groups_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticatorGroupsConfigs").unwrap(),
            ),
            binary_authorizations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("binaryAuthorizations").unwrap(),
            ),
            cluster_autoscalings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterAutoscalings").unwrap(),
            ),
            cluster_ipv4_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIpv4Cidr").unwrap(),
            ),
            cluster_telemetries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterTelemetries").unwrap(),
            ),
            confidential_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("confidentialNodes").unwrap(),
            ),
            control_plane_endpoints_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlPlaneEndpointsConfigs").unwrap(),
            ),
            cost_management_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("costManagementConfigs").unwrap(),
            ),
            database_encryptions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseEncryptions").unwrap(),
            ),
            datapath_provider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("datapathProvider").unwrap(),
            ),
            default_max_pods_per_node: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultMaxPodsPerNode").unwrap(),
            ),
            default_snat_statuses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSnatStatuses").unwrap(),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionProtection").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            dns_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsConfigs").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            enable_autopilot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableAutopilot").unwrap(),
            ),
            enable_cilium_clusterwide_network_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableCiliumClusterwideNetworkPolicy").unwrap(),
            ),
            enable_fqdn_network_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableFqdnNetworkPolicy").unwrap(),
            ),
            enable_intranode_visibility: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableIntranodeVisibility").unwrap(),
            ),
            enable_k8s_beta_apis: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableK8sBetaApis").unwrap(),
            ),
            enable_kubernetes_alpha: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableKubernetesAlpha").unwrap(),
            ),
            enable_l4_ilb_subsetting: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableL4IlbSubsetting").unwrap(),
            ),
            enable_legacy_abac: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableLegacyAbac").unwrap(),
            ),
            enable_multi_networking: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableMultiNetworking").unwrap(),
            ),
            enable_shielded_nodes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableShieldedNodes").unwrap(),
            ),
            enable_tpu: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enableTpu").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            enterprise_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enterpriseConfigs").unwrap(),
            ),
            fleets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleets").unwrap(),
            ),
            gateway_api_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayApiConfigs").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identity_service_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityServiceConfigs").unwrap(),
            ),
            initial_node_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("initialNodeCount").unwrap(),
            ),
            ip_allocation_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAllocationPolicies").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            logging_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingConfigs").unwrap(),
            ),
            logging_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loggingService").unwrap(),
            ),
            maintenance_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenancePolicies").unwrap(),
            ),
            master_authorized_networks_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAuthorizedNetworksConfigs").unwrap(),
            ),
            master_auths: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterAuths").unwrap(),
            ),
            master_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterVersion").unwrap(),
            ),
            mesh_certificates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("meshCertificates").unwrap(),
            ),
            min_master_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minMasterVersion").unwrap(),
            ),
            monitoring_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringConfigs").unwrap(),
            ),
            monitoring_service: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("monitoringService").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkPolicies").unwrap(),
            ),
            networking_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkingMode").unwrap(),
            ),
            node_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeConfigs").unwrap(),
            ),
            node_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeLocations").unwrap(),
            ),
            node_pool_auto_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodePoolAutoConfigs").unwrap(),
            ),
            node_pool_defaults: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodePoolDefaults").unwrap(),
            ),
            node_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodePools").unwrap(),
            ),
            node_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeVersion").unwrap(),
            ),
            notification_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("notificationConfigs").unwrap(),
            ),
            operation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("operation").unwrap(),
            ),
            pod_security_policy_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("podSecurityPolicyConfigs").unwrap(),
            ),
            private_cluster_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateClusterConfigs").unwrap(),
            ),
            private_ipv6_google_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateIpv6GoogleAccess").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            protect_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("protectConfigs").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            release_channels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("releaseChannels").unwrap(),
            ),
            remove_default_node_pool: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("removeDefaultNodePool").unwrap(),
            ),
            resource_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceLabels").unwrap(),
            ),
            resource_usage_export_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceUsageExportConfigs").unwrap(),
            ),
            secret_manager_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secretManagerConfigs").unwrap(),
            ),
            security_posture_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityPostureConfigs").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            service_external_ips_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceExternalIpsConfigs").unwrap(),
            ),
            services_ipv4_cidr: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicesIpv4Cidr").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            tpu_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tpuConfigs").unwrap(),
            ),
            tpu_ipv4_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tpuIpv4CidrBlock").unwrap(),
            ),
            user_managed_keys_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userManagedKeysConfigs").unwrap(),
            ),
            vertical_pod_autoscalings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("verticalPodAutoscalings").unwrap(),
            ),
            workload_alts_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadAltsConfigs").unwrap(),
            ),
            workload_identity_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workloadIdentityConfigs").unwrap(),
            ),
        }
    }
}
