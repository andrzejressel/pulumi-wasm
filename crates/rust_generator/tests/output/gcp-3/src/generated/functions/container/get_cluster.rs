#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetClusterArgs {
        /// The location (zone or region) this cluster has been
        /// created in. One of `location`, `region`, `zone`, or a provider-level `zone` must
        /// be specified.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetClusterResult {
        pub addons_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterAddonsConfig>,
        >,
        pub allow_net_admin: pulumi_gestalt_rust::Output<bool>,
        pub authenticator_groups_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterAuthenticatorGroupsConfig,
            >,
        >,
        pub binary_authorizations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterBinaryAuthorization>,
        >,
        pub cluster_autoscalings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterClusterAutoscaling>,
        >,
        pub cluster_ipv4_cidr: pulumi_gestalt_rust::Output<String>,
        pub cluster_telemetries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterClusterTelemetry>,
        >,
        pub confidential_nodes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterConfidentialNode>,
        >,
        pub control_plane_endpoints_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterControlPlaneEndpointsConfig,
            >,
        >,
        pub cost_management_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterCostManagementConfig>,
        >,
        pub database_encryptions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterDatabaseEncryption>,
        >,
        pub datapath_provider: pulumi_gestalt_rust::Output<String>,
        pub default_max_pods_per_node: pulumi_gestalt_rust::Output<i32>,
        pub default_snat_statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterDefaultSnatStatus>,
        >,
        pub deletion_protection: pulumi_gestalt_rust::Output<bool>,
        pub description: pulumi_gestalt_rust::Output<String>,
        pub dns_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterDnsConfig>,
        >,
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub enable_autopilot: pulumi_gestalt_rust::Output<bool>,
        pub enable_cilium_clusterwide_network_policy: pulumi_gestalt_rust::Output<bool>,
        pub enable_fqdn_network_policy: pulumi_gestalt_rust::Output<bool>,
        pub enable_intranode_visibility: pulumi_gestalt_rust::Output<bool>,
        pub enable_k8s_beta_apis: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterEnableK8SBetaApi>,
        >,
        pub enable_kubernetes_alpha: pulumi_gestalt_rust::Output<bool>,
        pub enable_l4_ilb_subsetting: pulumi_gestalt_rust::Output<bool>,
        pub enable_legacy_abac: pulumi_gestalt_rust::Output<bool>,
        pub enable_multi_networking: pulumi_gestalt_rust::Output<bool>,
        pub enable_shielded_nodes: pulumi_gestalt_rust::Output<bool>,
        pub enable_tpu: pulumi_gestalt_rust::Output<bool>,
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        pub enterprise_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterEnterpriseConfig>,
        >,
        pub fleets: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterFleet>,
        >,
        pub gateway_api_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterGatewayApiConfig>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identity_service_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterIdentityServiceConfig>,
        >,
        pub initial_node_count: pulumi_gestalt_rust::Output<i32>,
        pub ip_allocation_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterIpAllocationPolicy>,
        >,
        pub label_fingerprint: pulumi_gestalt_rust::Output<String>,
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        pub logging_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterLoggingConfig>,
        >,
        pub logging_service: pulumi_gestalt_rust::Output<String>,
        pub maintenance_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterMaintenancePolicy>,
        >,
        pub master_authorized_networks_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterMasterAuthorizedNetworksConfig,
            >,
        >,
        pub master_auths: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterMasterAuth>,
        >,
        pub master_version: pulumi_gestalt_rust::Output<String>,
        pub mesh_certificates: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterMeshCertificate>,
        >,
        pub min_master_version: pulumi_gestalt_rust::Output<String>,
        pub monitoring_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterMonitoringConfig>,
        >,
        pub monitoring_service: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network: pulumi_gestalt_rust::Output<String>,
        pub network_policies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNetworkPolicy>,
        >,
        pub networking_mode: pulumi_gestalt_rust::Output<String>,
        pub node_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodeConfig>,
        >,
        pub node_locations: pulumi_gestalt_rust::Output<Vec<String>>,
        pub node_pool_auto_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePoolAutoConfig>,
        >,
        pub node_pool_defaults: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePoolDefault>,
        >,
        pub node_pools: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNodePool>,
        >,
        pub node_version: pulumi_gestalt_rust::Output<String>,
        pub notification_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterNotificationConfig>,
        >,
        pub operation: pulumi_gestalt_rust::Output<String>,
        pub pod_security_policy_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterPodSecurityPolicyConfig>,
        >,
        pub private_cluster_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterPrivateClusterConfig>,
        >,
        pub private_ipv6_google_access: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<Option<String>>,
        pub protect_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterProtectConfig>,
        >,
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub release_channels: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterReleaseChannel>,
        >,
        pub remove_default_node_pool: pulumi_gestalt_rust::Output<bool>,
        pub resource_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub resource_usage_export_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterResourceUsageExportConfig,
            >,
        >,
        pub secret_manager_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterSecretManagerConfig>,
        >,
        pub security_posture_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterSecurityPostureConfig>,
        >,
        pub self_link: pulumi_gestalt_rust::Output<String>,
        pub service_external_ips_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::container::GetClusterServiceExternalIpsConfig,
            >,
        >,
        pub services_ipv4_cidr: pulumi_gestalt_rust::Output<String>,
        pub subnetwork: pulumi_gestalt_rust::Output<String>,
        pub tpu_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterTpuConfig>,
        >,
        pub tpu_ipv4_cidr_block: pulumi_gestalt_rust::Output<String>,
        pub user_managed_keys_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterUserManagedKeysConfig>,
        >,
        pub vertical_pod_autoscalings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterVerticalPodAutoscaling>,
        >,
        pub workload_alts_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterWorkloadAltsConfig>,
        >,
        pub workload_identity_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::container::GetClusterWorkloadIdentityConfig>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetClusterArgs,
    ) -> GetClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetClusterResult {
            addons_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addonsConfigs"),
            ),
            allow_net_admin: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowNetAdmin"),
            ),
            authenticator_groups_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authenticatorGroupsConfigs"),
            ),
            binary_authorizations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("binaryAuthorizations"),
            ),
            cluster_autoscalings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterAutoscalings"),
            ),
            cluster_ipv4_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterIpv4Cidr"),
            ),
            cluster_telemetries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterTelemetries"),
            ),
            confidential_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("confidentialNodes"),
            ),
            control_plane_endpoints_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("controlPlaneEndpointsConfigs"),
            ),
            cost_management_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("costManagementConfigs"),
            ),
            database_encryptions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseEncryptions"),
            ),
            datapath_provider: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("datapathProvider"),
            ),
            default_max_pods_per_node: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultMaxPodsPerNode"),
            ),
            default_snat_statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultSnatStatuses"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            dns_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsConfigs"),
            ),
            effective_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            enable_autopilot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableAutopilot"),
            ),
            enable_cilium_clusterwide_network_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableCiliumClusterwideNetworkPolicy"),
            ),
            enable_fqdn_network_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableFqdnNetworkPolicy"),
            ),
            enable_intranode_visibility: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableIntranodeVisibility"),
            ),
            enable_k8s_beta_apis: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableK8sBetaApis"),
            ),
            enable_kubernetes_alpha: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableKubernetesAlpha"),
            ),
            enable_l4_ilb_subsetting: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableL4IlbSubsetting"),
            ),
            enable_legacy_abac: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableLegacyAbac"),
            ),
            enable_multi_networking: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableMultiNetworking"),
            ),
            enable_shielded_nodes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableShieldedNodes"),
            ),
            enable_tpu: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableTpu"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            enterprise_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enterpriseConfigs"),
            ),
            fleets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fleets"),
            ),
            gateway_api_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayApiConfigs"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identity_service_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identityServiceConfigs"),
            ),
            initial_node_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("initialNodeCount"),
            ),
            ip_allocation_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAllocationPolicies"),
            ),
            label_fingerprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("labelFingerprint"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            logging_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingConfigs"),
            ),
            logging_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("loggingService"),
            ),
            maintenance_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenancePolicies"),
            ),
            master_authorized_networks_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterAuthorizedNetworksConfigs"),
            ),
            master_auths: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterAuths"),
            ),
            master_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterVersion"),
            ),
            mesh_certificates: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("meshCertificates"),
            ),
            min_master_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minMasterVersion"),
            ),
            monitoring_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringConfigs"),
            ),
            monitoring_service: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringService"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            network_policies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkPolicies"),
            ),
            networking_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkingMode"),
            ),
            node_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeConfigs"),
            ),
            node_locations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeLocations"),
            ),
            node_pool_auto_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePoolAutoConfigs"),
            ),
            node_pool_defaults: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePoolDefaults"),
            ),
            node_pools: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodePools"),
            ),
            node_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeVersion"),
            ),
            notification_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("notificationConfigs"),
            ),
            operation: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("operation"),
            ),
            pod_security_policy_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("podSecurityPolicyConfigs"),
            ),
            private_cluster_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateClusterConfigs"),
            ),
            private_ipv6_google_access: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpv6GoogleAccess"),
            ),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            protect_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("protectConfigs"),
            ),
            pulumi_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            release_channels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("releaseChannels"),
            ),
            remove_default_node_pool: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("removeDefaultNodePool"),
            ),
            resource_labels: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceLabels"),
            ),
            resource_usage_export_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceUsageExportConfigs"),
            ),
            secret_manager_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("secretManagerConfigs"),
            ),
            security_posture_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("securityPostureConfigs"),
            ),
            self_link: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            service_external_ips_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceExternalIpsConfigs"),
            ),
            services_ipv4_cidr: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("servicesIpv4Cidr"),
            ),
            subnetwork: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetwork"),
            ),
            tpu_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tpuConfigs"),
            ),
            tpu_ipv4_cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tpuIpv4CidrBlock"),
            ),
            user_managed_keys_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("userManagedKeysConfigs"),
            ),
            vertical_pod_autoscalings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("verticalPodAutoscalings"),
            ),
            workload_alts_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadAltsConfigs"),
            ),
            workload_identity_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workloadIdentityConfigs"),
            ),
        }
    }
}
