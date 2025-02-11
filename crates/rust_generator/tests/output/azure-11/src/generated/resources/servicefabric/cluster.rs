/// Manages a Service Fabric Cluster.
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
///     let exampleCluster = cluster::create(
///         "exampleCluster",
///         ClusterArgs::builder()
///             .cluster_code_version("7.1.456.959")
///             .location("${example.location}")
///             .management_endpoint("https://example:80")
///             .name("example-servicefabric")
///             .node_types(
///                 vec![
///                     ClusterNodeType::builder().clientEndpointPort(2020)
///                     .httpEndpointPort(80).instanceCount(3).isPrimary(true).name("first")
///                     .build_struct(),
///                 ],
///             )
///             .reliability_level("Bronze")
///             .resource_group_name("${example.name}")
///             .upgrade_mode("Manual")
///             .vm_image("Windows")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Service Fabric Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:servicefabric/cluster:Cluster cluster1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceFabric/clusters/cluster1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterArgs {
        /// A List of one or more features which should be enabled, such as `DnsService`.
        #[builder(into, default)]
        pub add_on_features: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// An `azure_active_directory` block as defined below.
        #[builder(into, default)]
        pub azure_active_directory: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterAzureActiveDirectory>,
        >,
        /// A `certificate` block as defined below. Conflicts with `certificate_common_names`.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterCertificate>,
        >,
        /// A `certificate_common_names` block as defined below. Conflicts with `certificate`.
        #[builder(into, default)]
        pub certificate_common_names: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterCertificateCommonNames>,
        >,
        /// A `client_certificate_common_name` block as defined below.
        ///
        /// > **NOTE:** If Client Certificates are enabled then at a Certificate must be configured on the cluster.
        #[builder(into, default)]
        pub client_certificate_common_names: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::servicefabric::ClusterClientCertificateCommonName,
                >,
            >,
        >,
        /// One or more `client_certificate_thumbprint` blocks as defined below.
        #[builder(into, default)]
        pub client_certificate_thumbprints: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::servicefabric::ClusterClientCertificateThumbprint,
                >,
            >,
        >,
        /// Required if Upgrade Mode set to `Manual`, Specifies the Version of the Cluster Code of the cluster.
        #[builder(into, default)]
        pub cluster_code_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `diagnostics_config` block as defined below.
        #[builder(into, default)]
        pub diagnostics_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterDiagnosticsConfig>,
        >,
        /// One or more `fabric_settings` blocks as defined below.
        #[builder(into, default)]
        pub fabric_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::servicefabric::ClusterFabricSetting>>,
        >,
        /// Specifies the Azure Region where the Service Fabric Cluster should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Management Endpoint of the cluster such as `http://example.com`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub management_endpoint: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Service Fabric Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `node_type` blocks as defined below.
        #[builder(into)]
        pub node_types: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::servicefabric::ClusterNodeType>,
        >,
        /// Specifies the Reliability Level of the Cluster. Possible values include `None`, `Bronze`, `Silver`, `Gold` and `Platinum`.
        ///
        /// > **NOTE:** The Reliability Level of the Cluster depends on the number of nodes in the Cluster: `Platinum` requires at least 9 VM's, `Gold` requires at least 7 VM's, `Silver` requires at least 5 VM's, `Bronze` requires at least 3 VM's.
        #[builder(into)]
        pub reliability_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group in which the Service Fabric Cluster exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `reverse_proxy_certificate` block as defined below. Conflicts with `reverse_proxy_certificate_common_names`.
        #[builder(into, default)]
        pub reverse_proxy_certificate: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterReverseProxyCertificate>,
        >,
        /// A `reverse_proxy_certificate_common_names` block as defined below. Conflicts with `reverse_proxy_certificate`.
        #[builder(into, default)]
        pub reverse_proxy_certificate_common_names: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::servicefabric::ClusterReverseProxyCertificateCommonNames,
            >,
        >,
        /// Specifies the logical grouping of VMs in upgrade domains. Possible values are `Hierarchical` or `Parallel`.
        #[builder(into, default)]
        pub service_fabric_zonal_upgrade_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Upgrade Mode of the cluster. Possible values are `Automatic` or `Manual`.
        #[builder(into)]
        pub upgrade_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `upgrade_policy` block as defined below.
        #[builder(into, default)]
        pub upgrade_policy: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ClusterUpgradePolicy>,
        >,
        /// Specifies the Image expected for the Service Fabric Cluster, such as `Windows`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub vm_image: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the upgrade mode for the virtual machine scale set updates that happen in all availability zones at once. Possible values are `Hierarchical` or `Parallel`.
        #[builder(into, default)]
        pub vmss_zonal_upgrade_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ClusterResult {
        /// A List of one or more features which should be enabled, such as `DnsService`.
        pub add_on_features: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An `azure_active_directory` block as defined below.
        pub azure_active_directory: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterAzureActiveDirectory>,
        >,
        /// A `certificate` block as defined below. Conflicts with `certificate_common_names`.
        pub certificate: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterCertificate>,
        >,
        /// A `certificate_common_names` block as defined below. Conflicts with `certificate`.
        pub certificate_common_names: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterCertificateCommonNames>,
        >,
        /// A `client_certificate_common_name` block as defined below.
        ///
        /// > **NOTE:** If Client Certificates are enabled then at a Certificate must be configured on the cluster.
        pub client_certificate_common_names: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::servicefabric::ClusterClientCertificateCommonName,
                >,
            >,
        >,
        /// One or more `client_certificate_thumbprint` blocks as defined below.
        pub client_certificate_thumbprints: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::servicefabric::ClusterClientCertificateThumbprint,
                >,
            >,
        >,
        /// Required if Upgrade Mode set to `Manual`, Specifies the Version of the Cluster Code of the cluster.
        pub cluster_code_version: pulumi_gestalt_rust::Output<String>,
        /// The Cluster Endpoint for this Service Fabric Cluster.
        pub cluster_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A `diagnostics_config` block as defined below.
        pub diagnostics_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterDiagnosticsConfig>,
        >,
        /// One or more `fabric_settings` blocks as defined below.
        pub fabric_settings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::servicefabric::ClusterFabricSetting>>,
        >,
        /// Specifies the Azure Region where the Service Fabric Cluster should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Management Endpoint of the cluster such as `http://example.com`. Changing this forces a new resource to be created.
        pub management_endpoint: pulumi_gestalt_rust::Output<String>,
        /// The name of the Service Fabric Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `node_type` blocks as defined below.
        pub node_types: pulumi_gestalt_rust::Output<
            Vec<super::super::types::servicefabric::ClusterNodeType>,
        >,
        /// Specifies the Reliability Level of the Cluster. Possible values include `None`, `Bronze`, `Silver`, `Gold` and `Platinum`.
        ///
        /// > **NOTE:** The Reliability Level of the Cluster depends on the number of nodes in the Cluster: `Platinum` requires at least 9 VM's, `Gold` requires at least 7 VM's, `Silver` requires at least 5 VM's, `Bronze` requires at least 3 VM's.
        pub reliability_level: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group in which the Service Fabric Cluster exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `reverse_proxy_certificate` block as defined below. Conflicts with `reverse_proxy_certificate_common_names`.
        pub reverse_proxy_certificate: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterReverseProxyCertificate>,
        >,
        /// A `reverse_proxy_certificate_common_names` block as defined below. Conflicts with `reverse_proxy_certificate`.
        pub reverse_proxy_certificate_common_names: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::servicefabric::ClusterReverseProxyCertificateCommonNames,
            >,
        >,
        /// Specifies the logical grouping of VMs in upgrade domains. Possible values are `Hierarchical` or `Parallel`.
        pub service_fabric_zonal_upgrade_mode: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Upgrade Mode of the cluster. Possible values are `Automatic` or `Manual`.
        pub upgrade_mode: pulumi_gestalt_rust::Output<String>,
        /// A `upgrade_policy` block as defined below.
        pub upgrade_policy: pulumi_gestalt_rust::Output<
            Option<super::super::types::servicefabric::ClusterUpgradePolicy>,
        >,
        /// Specifies the Image expected for the Service Fabric Cluster, such as `Windows`. Changing this forces a new resource to be created.
        pub vm_image: pulumi_gestalt_rust::Output<String>,
        /// Specifies the upgrade mode for the virtual machine scale set updates that happen in all availability zones at once. Possible values are `Hierarchical` or `Parallel`.
        pub vmss_zonal_upgrade_mode: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let add_on_features_binding = args.add_on_features.get_output(context);
        let azure_active_directory_binding = args
            .azure_active_directory
            .get_output(context);
        let certificate_binding = args.certificate.get_output(context);
        let certificate_common_names_binding = args
            .certificate_common_names
            .get_output(context);
        let client_certificate_common_names_binding = args
            .client_certificate_common_names
            .get_output(context);
        let client_certificate_thumbprints_binding = args
            .client_certificate_thumbprints
            .get_output(context);
        let cluster_code_version_binding = args.cluster_code_version.get_output(context);
        let diagnostics_config_binding = args.diagnostics_config.get_output(context);
        let fabric_settings_binding = args.fabric_settings.get_output(context);
        let location_binding = args.location.get_output(context);
        let management_endpoint_binding = args.management_endpoint.get_output(context);
        let name_binding = args.name.get_output(context);
        let node_types_binding = args.node_types.get_output(context);
        let reliability_level_binding = args.reliability_level.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let reverse_proxy_certificate_binding = args
            .reverse_proxy_certificate
            .get_output(context);
        let reverse_proxy_certificate_common_names_binding = args
            .reverse_proxy_certificate_common_names
            .get_output(context);
        let service_fabric_zonal_upgrade_mode_binding = args
            .service_fabric_zonal_upgrade_mode
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let upgrade_mode_binding = args.upgrade_mode.get_output(context);
        let upgrade_policy_binding = args.upgrade_policy.get_output(context);
        let vm_image_binding = args.vm_image.get_output(context);
        let vmss_zonal_upgrade_mode_binding = args
            .vmss_zonal_upgrade_mode
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:servicefabric/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addOnFeatures".into(),
                    value: &add_on_features_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "azureActiveDirectory".into(),
                    value: &azure_active_directory_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateCommonNames".into(),
                    value: &certificate_common_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateCommonNames".into(),
                    value: &client_certificate_common_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateThumbprints".into(),
                    value: &client_certificate_thumbprints_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterCodeVersion".into(),
                    value: &cluster_code_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "diagnosticsConfig".into(),
                    value: &diagnostics_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fabricSettings".into(),
                    value: &fabric_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managementEndpoint".into(),
                    value: &management_endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nodeTypes".into(),
                    value: &node_types_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reliabilityLevel".into(),
                    value: &reliability_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseProxyCertificate".into(),
                    value: &reverse_proxy_certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseProxyCertificateCommonNames".into(),
                    value: &reverse_proxy_certificate_common_names_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "serviceFabricZonalUpgradeMode".into(),
                    value: &service_fabric_zonal_upgrade_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradeMode".into(),
                    value: &upgrade_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "upgradePolicy".into(),
                    value: &upgrade_policy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmImage".into(),
                    value: &vm_image_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vmssZonalUpgradeMode".into(),
                    value: &vmss_zonal_upgrade_mode_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterResult {
            add_on_features: o.get_field("addOnFeatures"),
            azure_active_directory: o.get_field("azureActiveDirectory"),
            certificate: o.get_field("certificate"),
            certificate_common_names: o.get_field("certificateCommonNames"),
            client_certificate_common_names: o.get_field("clientCertificateCommonNames"),
            client_certificate_thumbprints: o.get_field("clientCertificateThumbprints"),
            cluster_code_version: o.get_field("clusterCodeVersion"),
            cluster_endpoint: o.get_field("clusterEndpoint"),
            diagnostics_config: o.get_field("diagnosticsConfig"),
            fabric_settings: o.get_field("fabricSettings"),
            location: o.get_field("location"),
            management_endpoint: o.get_field("managementEndpoint"),
            name: o.get_field("name"),
            node_types: o.get_field("nodeTypes"),
            reliability_level: o.get_field("reliabilityLevel"),
            resource_group_name: o.get_field("resourceGroupName"),
            reverse_proxy_certificate: o.get_field("reverseProxyCertificate"),
            reverse_proxy_certificate_common_names: o
                .get_field("reverseProxyCertificateCommonNames"),
            service_fabric_zonal_upgrade_mode: o
                .get_field("serviceFabricZonalUpgradeMode"),
            tags: o.get_field("tags"),
            upgrade_mode: o.get_field("upgradeMode"),
            upgrade_policy: o.get_field("upgradePolicy"),
            vm_image: o.get_field("vmImage"),
            vmss_zonal_upgrade_mode: o.get_field("vmssZonalUpgradeMode"),
        }
    }
}
