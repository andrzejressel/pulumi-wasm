#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_kubernetes_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKubernetesClusterArgs {
        /// The name of the managed Kubernetes Cluster.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the managed Kubernetes Cluster exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetKubernetesClusterResult {
        /// An `aci_connector_linux` block as documented below.
        pub aci_connector_linuxes: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAciConnectorLinux,
            >,
        >,
        /// An `agent_pool_profile` block as documented below.
        pub agent_pool_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAgentPoolProfile,
            >,
        >,
        /// The IP ranges to whitelist for incoming traffic to the primaries.
        pub api_server_authorized_ip_ranges: pulumi_gestalt_rust::Output<Vec<String>>,
        /// An `azure_active_directory_role_based_access_control` block as documented below.
        pub azure_active_directory_role_based_access_controls: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterAzureActiveDirectoryRoleBasedAccessControl,
            >,
        >,
        /// Is Azure Policy enabled on this managed Kubernetes Cluster?
        pub azure_policy_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Contains the current version of Kubernetes running on the Cluster.
        pub current_kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Disk Encryption Set used for the Nodes and Volumes.
        pub disk_encryption_set_id: pulumi_gestalt_rust::Output<String>,
        /// The DNS Prefix of the managed Kubernetes cluster.
        pub dns_prefix: pulumi_gestalt_rust::Output<String>,
        /// The FQDN of the Azure Kubernetes Managed Cluster.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// Is HTTP Application Routing enabled for this managed Kubernetes Cluster?
        pub http_application_routing_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Zone Name of the HTTP Application Routing.
        pub http_application_routing_zone_name: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as documented below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterIdentity,
            >,
        >,
        /// An `ingress_application_gateway` block as documented below.
        pub ingress_application_gateways: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterIngressApplicationGateway,
            >,
        >,
        /// A `key_management_service` block as documented below.
        pub key_management_services: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKeyManagementService,
            >,
        >,
        /// A `key_vault_secrets_provider` block as documented below.
        pub key_vault_secrets_providers: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKeyVaultSecretsProvider,
            >,
        >,
        /// Raw Kubernetes config for the admin account to be used by [kubectl](https://kubernetes.io/docs/reference/kubectl/overview/) and other compatible tools. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts are not disabled.
        pub kube_admin_config_raw: pulumi_gestalt_rust::Output<String>,
        /// A `kube_admin_config` block as defined below. This is only available when Role Based Access Control with Azure Active Directory is enabled and local accounts are not disabled.
        pub kube_admin_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeAdminConfig,
            >,
        >,
        /// Base64 encoded Kubernetes configuration.
        pub kube_config_raw: pulumi_gestalt_rust::Output<String>,
        /// A `kube_config` block as defined below.
        pub kube_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeConfig,
            >,
        >,
        /// A `kubelet_identity` block as documented below.
        pub kubelet_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterKubeletIdentity,
            >,
        >,
        /// The version of Kubernetes used on the managed Kubernetes Cluster.
        pub kubernetes_version: pulumi_gestalt_rust::Output<String>,
        /// A `linux_profile` block as documented below.
        pub linux_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterLinuxProfile,
            >,
        >,
        /// The Azure Region in which the managed Kubernetes Cluster exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `microsoft_defender` block as defined below.
        pub microsoft_defenders: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterMicrosoftDefender,
            >,
        >,
        /// The name assigned to this pool of agents.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `network_profile` block as documented below.
        pub network_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterNetworkProfile,
            >,
        >,
        /// Auto-generated Resource Group containing AKS Cluster resources.
        pub node_resource_group: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Resource Group containing the resources for this Managed Kubernetes Cluster.
        pub node_resource_group_id: pulumi_gestalt_rust::Output<String>,
        /// Whether or not the OIDC feature is enabled or disabled.
        pub oidc_issuer_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The OIDC issuer URL that is associated with the cluster.
        pub oidc_issuer_url: pulumi_gestalt_rust::Output<String>,
        /// An `oms_agent` block as documented below.
        pub oms_agents: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterOmsAgent,
            >,
        >,
        /// Is Open Service Mesh enabled for this managed Kubernetes Cluster?
        pub open_service_mesh_enabled: pulumi_gestalt_rust::Output<bool>,
        /// If the cluster has the Kubernetes API only exposed on internal IP addresses.
        pub private_cluster_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The FQDN of this Kubernetes Cluster when private link has been enabled. This name is only resolvable inside the Virtual Network where the Azure Kubernetes Service is located
        pub private_fqdn: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Is Role Based Access Control enabled for this managed Kubernetes Cluster?
        pub role_based_access_control_enabled: pulumi_gestalt_rust::Output<bool>,
        pub service_mesh_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterServiceMeshProfile,
            >,
        >,
        /// A `service_principal` block as documented below.
        pub service_principals: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterServicePrincipal,
            >,
        >,
        /// A `storage_profile` block as documented below.
        pub storage_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterStorageProfile,
            >,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A `windows_profile` block as documented below.
        pub windows_profiles: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::containerservice::GetKubernetesClusterWindowsProfile,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKubernetesClusterArgs,
    ) -> GetKubernetesClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerservice/getKubernetesCluster:getKubernetesCluster"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKubernetesClusterResult {
            aci_connector_linuxes: o.get_field("aciConnectorLinuxes"),
            agent_pool_profiles: o.get_field("agentPoolProfiles"),
            api_server_authorized_ip_ranges: o.get_field("apiServerAuthorizedIpRanges"),
            azure_active_directory_role_based_access_controls: o
                .get_field("azureActiveDirectoryRoleBasedAccessControls"),
            azure_policy_enabled: o.get_field("azurePolicyEnabled"),
            current_kubernetes_version: o.get_field("currentKubernetesVersion"),
            disk_encryption_set_id: o.get_field("diskEncryptionSetId"),
            dns_prefix: o.get_field("dnsPrefix"),
            fqdn: o.get_field("fqdn"),
            http_application_routing_enabled: o
                .get_field("httpApplicationRoutingEnabled"),
            http_application_routing_zone_name: o
                .get_field("httpApplicationRoutingZoneName"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            ingress_application_gateways: o.get_field("ingressApplicationGateways"),
            key_management_services: o.get_field("keyManagementServices"),
            key_vault_secrets_providers: o.get_field("keyVaultSecretsProviders"),
            kube_admin_config_raw: o.get_field("kubeAdminConfigRaw"),
            kube_admin_configs: o.get_field("kubeAdminConfigs"),
            kube_config_raw: o.get_field("kubeConfigRaw"),
            kube_configs: o.get_field("kubeConfigs"),
            kubelet_identities: o.get_field("kubeletIdentities"),
            kubernetes_version: o.get_field("kubernetesVersion"),
            linux_profiles: o.get_field("linuxProfiles"),
            location: o.get_field("location"),
            microsoft_defenders: o.get_field("microsoftDefenders"),
            name: o.get_field("name"),
            network_profiles: o.get_field("networkProfiles"),
            node_resource_group: o.get_field("nodeResourceGroup"),
            node_resource_group_id: o.get_field("nodeResourceGroupId"),
            oidc_issuer_enabled: o.get_field("oidcIssuerEnabled"),
            oidc_issuer_url: o.get_field("oidcIssuerUrl"),
            oms_agents: o.get_field("omsAgents"),
            open_service_mesh_enabled: o.get_field("openServiceMeshEnabled"),
            private_cluster_enabled: o.get_field("privateClusterEnabled"),
            private_fqdn: o.get_field("privateFqdn"),
            resource_group_name: o.get_field("resourceGroupName"),
            role_based_access_control_enabled: o
                .get_field("roleBasedAccessControlEnabled"),
            service_mesh_profiles: o.get_field("serviceMeshProfiles"),
            service_principals: o.get_field("servicePrincipals"),
            storage_profiles: o.get_field("storageProfiles"),
            tags: o.get_field("tags"),
            windows_profiles: o.get_field("windowsProfiles"),
        }
    }
}
