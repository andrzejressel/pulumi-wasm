pub mod get_kubernetes_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesClusterArgs {
        /// The name of the managed Kubernetes Cluster.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group in which the managed Kubernetes Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetKubernetesClusterResult {
        /// An `aci_connector_linux` block as documented below.
        pub aci_connector_linuxes: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAciConnectorLinux,
            >,
        >,
        /// An `agent_pool_profile` block as documented below.
        pub agent_pool_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAgentPoolProfile,
            >,
        >,
        /// The IP ranges to whitelist for incoming traffic to the primaries.
        pub api_server_authorized_ip_ranges: pulumi_wasm_rust::Output<Vec<String>>,
        /// An `azure_active_directory_role_based_access_control` block as documented below.
        pub azure_active_directory_role_based_access_controls: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAzureActiveDirectoryRoleBasedAccessControl,
            >,
        >,
        /// Is Azure Policy enabled on this managed Kubernetes Cluster?
        pub azure_policy_enabled: pulumi_wasm_rust::Output<bool>,
        /// Contains the current version of Kubernetes running on the Cluster.
        pub current_kubernetes_version: pulumi_wasm_rust::Output<String>,
        /// The ID of the Disk Encryption Set used for the Nodes and Volumes.
        pub disk_encryption_set_id: pulumi_wasm_rust::Output<String>,
        /// The DNS Prefix of the managed Kubernetes cluster.
        pub dns_prefix: pulumi_wasm_rust::Output<String>,
        /// The FQDN of the Azure Kubernetes Managed Cluster.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Is HTTP Application Routing enabled for this managed Kubernetes Cluster?
        pub http_application_routing_enabled: pulumi_wasm_rust::Output<bool>,
        /// The Zone Name of the HTTP Application Routing.
        pub http_application_routing_zone_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as documented below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterIdentity,
            >,
        >,
        /// An `ingress_application_gateway` block as documented below.
        pub ingress_application_gateways: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterIngressApplicationGateway,
            >,
        >,
        /// A `key_management_service` block as documented below.
        pub key_management_services: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKeyManagementService,
            >,
        >,
        /// A `key_vault_secrets_provider` block as documented below.
        pub key_vault_secrets_providers: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKeyVaultSecretsProvider,
            >,
        >,
        /// Raw Kubernetes config for the admin account to be used by [kubectl](https://kubernetes.io/docs/reference/kubectl/overview/) and other compatible tools. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts are not disabled.
        pub kube_admin_config_raw: pulumi_wasm_rust::Output<String>,
        /// A `kube_admin_config` block as defined below. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts are not disabled.
        pub kube_admin_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeAdminConfig,
            >,
        >,
        /// Base64 encoded Kubernetes configuration.
        pub kube_config_raw: pulumi_wasm_rust::Output<String>,
        /// A `kube_config` block as defined below.
        pub kube_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeConfig,
            >,
        >,
        /// A `kubelet_identity` block as documented below.
        pub kubelet_identities: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeletIdentity,
            >,
        >,
        /// The version of Kubernetes used on the managed Kubernetes Cluster.
        pub kubernetes_version: pulumi_wasm_rust::Output<String>,
        /// A `linux_profile` block as documented below.
        pub linux_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterLinuxProfile,
            >,
        >,
        /// The Azure Region in which the managed Kubernetes Cluster exists.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A `microsoft_defender` block as defined below.
        pub microsoft_defenders: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterMicrosoftDefender,
            >,
        >,
        /// The name assigned to this pool of agents.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_profile` block as documented below.
        pub network_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterNetworkProfile,
            >,
        >,
        /// Auto-generated Resource Group containing AKS Cluster resources.
        pub node_resource_group: pulumi_wasm_rust::Output<String>,
        /// The ID of the Resource Group containing the resources for this Managed Kubernetes Cluster.
        pub node_resource_group_id: pulumi_wasm_rust::Output<String>,
        /// Whether or not the OIDC feature is enabled or disabled.
        pub oidc_issuer_enabled: pulumi_wasm_rust::Output<bool>,
        /// The OIDC issuer URL that is associated with the cluster.
        pub oidc_issuer_url: pulumi_wasm_rust::Output<String>,
        /// An `oms_agent` block as documented below.
        pub oms_agents: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterOmsAgent,
            >,
        >,
        /// Is Open Service Mesh enabled for this managed Kubernetes Cluster?
        pub open_service_mesh_enabled: pulumi_wasm_rust::Output<bool>,
        /// If the cluster has the Kubernetes API only exposed on internal IP addresses.
        pub private_cluster_enabled: pulumi_wasm_rust::Output<bool>,
        /// The FQDN of this Kubernetes Cluster when private link has been enabled. This name is only resolvable inside the Virtual Network where the Azure Kubernetes Service is located
        pub private_fqdn: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Is Role Based Access Control enabled for this managed Kubernetes Cluster?
        pub role_based_access_control_enabled: pulumi_wasm_rust::Output<bool>,
        pub service_mesh_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterServiceMeshProfile,
            >,
        >,
        /// A `service_principal` block as documented below.
        pub service_principals: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterServicePrincipal,
            >,
        >,
        /// A `storage_profile` block as documented below.
        pub storage_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterStorageProfile,
            >,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A `windows_profile` block as documented below.
        pub windows_profiles: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterWindowsProfile,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetKubernetesClusterArgs) -> GetKubernetesClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:containerservice/getKubernetesCluster:getKubernetesCluster"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "aciConnectorLinuxes".into(),
                },
                register_interface::ResultField {
                    name: "agentPoolProfiles".into(),
                },
                register_interface::ResultField {
                    name: "apiServerAuthorizedIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "azureActiveDirectoryRoleBasedAccessControls".into(),
                },
                register_interface::ResultField {
                    name: "azurePolicyEnabled".into(),
                },
                register_interface::ResultField {
                    name: "currentKubernetesVersion".into(),
                },
                register_interface::ResultField {
                    name: "diskEncryptionSetId".into(),
                },
                register_interface::ResultField {
                    name: "dnsPrefix".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "httpApplicationRoutingEnabled".into(),
                },
                register_interface::ResultField {
                    name: "httpApplicationRoutingZoneName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "ingressApplicationGateways".into(),
                },
                register_interface::ResultField {
                    name: "keyManagementServices".into(),
                },
                register_interface::ResultField {
                    name: "keyVaultSecretsProviders".into(),
                },
                register_interface::ResultField {
                    name: "kubeAdminConfigRaw".into(),
                },
                register_interface::ResultField {
                    name: "kubeAdminConfigs".into(),
                },
                register_interface::ResultField {
                    name: "kubeConfigRaw".into(),
                },
                register_interface::ResultField {
                    name: "kubeConfigs".into(),
                },
                register_interface::ResultField {
                    name: "kubeletIdentities".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesVersion".into(),
                },
                register_interface::ResultField {
                    name: "linuxProfiles".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "microsoftDefenders".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkProfiles".into(),
                },
                register_interface::ResultField {
                    name: "nodeResourceGroup".into(),
                },
                register_interface::ResultField {
                    name: "nodeResourceGroupId".into(),
                },
                register_interface::ResultField {
                    name: "oidcIssuerEnabled".into(),
                },
                register_interface::ResultField {
                    name: "oidcIssuerUrl".into(),
                },
                register_interface::ResultField {
                    name: "omsAgents".into(),
                },
                register_interface::ResultField {
                    name: "openServiceMeshEnabled".into(),
                },
                register_interface::ResultField {
                    name: "privateClusterEnabled".into(),
                },
                register_interface::ResultField {
                    name: "privateFqdn".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "roleBasedAccessControlEnabled".into(),
                },
                register_interface::ResultField {
                    name: "serviceMeshProfiles".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipals".into(),
                },
                register_interface::ResultField {
                    name: "storageProfiles".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "windowsProfiles".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetKubernetesClusterResult {
            aci_connector_linuxes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("aciConnectorLinuxes").unwrap(),
            ),
            agent_pool_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("agentPoolProfiles").unwrap(),
            ),
            api_server_authorized_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiServerAuthorizedIpRanges").unwrap(),
            ),
            azure_active_directory_role_based_access_controls: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureActiveDirectoryRoleBasedAccessControls").unwrap(),
            ),
            azure_policy_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azurePolicyEnabled").unwrap(),
            ),
            current_kubernetes_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("currentKubernetesVersion").unwrap(),
            ),
            disk_encryption_set_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskEncryptionSetId").unwrap(),
            ),
            dns_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsPrefix").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            http_application_routing_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpApplicationRoutingEnabled").unwrap(),
            ),
            http_application_routing_zone_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("httpApplicationRoutingZoneName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            ingress_application_gateways: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ingressApplicationGateways").unwrap(),
            ),
            key_management_services: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyManagementServices").unwrap(),
            ),
            key_vault_secrets_providers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyVaultSecretsProviders").unwrap(),
            ),
            kube_admin_config_raw: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeAdminConfigRaw").unwrap(),
            ),
            kube_admin_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeAdminConfigs").unwrap(),
            ),
            kube_config_raw: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeConfigRaw").unwrap(),
            ),
            kube_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeConfigs").unwrap(),
            ),
            kubelet_identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubeletIdentities").unwrap(),
            ),
            kubernetes_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesVersion").unwrap(),
            ),
            linux_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("linuxProfiles").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            microsoft_defenders: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("microsoftDefenders").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkProfiles").unwrap(),
            ),
            node_resource_group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeResourceGroup").unwrap(),
            ),
            node_resource_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("nodeResourceGroupId").unwrap(),
            ),
            oidc_issuer_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oidcIssuerEnabled").unwrap(),
            ),
            oidc_issuer_url: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oidcIssuerUrl").unwrap(),
            ),
            oms_agents: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("omsAgents").unwrap(),
            ),
            open_service_mesh_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("openServiceMeshEnabled").unwrap(),
            ),
            private_cluster_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateClusterEnabled").unwrap(),
            ),
            private_fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateFqdn").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            role_based_access_control_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleBasedAccessControlEnabled").unwrap(),
            ),
            service_mesh_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceMeshProfiles").unwrap(),
            ),
            service_principals: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipals").unwrap(),
            ),
            storage_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageProfiles").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            windows_profiles: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windowsProfiles").unwrap(),
            ),
        }
    }
}
