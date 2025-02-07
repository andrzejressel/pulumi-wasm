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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ClusterArgs,
    ) -> ClusterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let add_on_features_binding = args
            .add_on_features
            .get_output(context)
            .get_inner();
        let azure_active_directory_binding = args
            .azure_active_directory
            .get_output(context)
            .get_inner();
        let certificate_binding = args.certificate.get_output(context).get_inner();
        let certificate_common_names_binding = args
            .certificate_common_names
            .get_output(context)
            .get_inner();
        let client_certificate_common_names_binding = args
            .client_certificate_common_names
            .get_output(context)
            .get_inner();
        let client_certificate_thumbprints_binding = args
            .client_certificate_thumbprints
            .get_output(context)
            .get_inner();
        let cluster_code_version_binding = args
            .cluster_code_version
            .get_output(context)
            .get_inner();
        let diagnostics_config_binding = args
            .diagnostics_config
            .get_output(context)
            .get_inner();
        let fabric_settings_binding = args
            .fabric_settings
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let management_endpoint_binding = args
            .management_endpoint
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_types_binding = args.node_types.get_output(context).get_inner();
        let reliability_level_binding = args
            .reliability_level
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let reverse_proxy_certificate_binding = args
            .reverse_proxy_certificate
            .get_output(context)
            .get_inner();
        let reverse_proxy_certificate_common_names_binding = args
            .reverse_proxy_certificate_common_names
            .get_output(context)
            .get_inner();
        let service_fabric_zonal_upgrade_mode_binding = args
            .service_fabric_zonal_upgrade_mode
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let upgrade_mode_binding = args.upgrade_mode.get_output(context).get_inner();
        let upgrade_policy_binding = args.upgrade_policy.get_output(context).get_inner();
        let vm_image_binding = args.vm_image.get_output(context).get_inner();
        let vmss_zonal_upgrade_mode_binding = args
            .vmss_zonal_upgrade_mode
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicefabric/cluster:Cluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addOnFeatures".into(),
                    value: &add_on_features_binding,
                },
                register_interface::ObjectField {
                    name: "azureActiveDirectory".into(),
                    value: &azure_active_directory_binding,
                },
                register_interface::ObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding,
                },
                register_interface::ObjectField {
                    name: "certificateCommonNames".into(),
                    value: &certificate_common_names_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateCommonNames".into(),
                    value: &client_certificate_common_names_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificateThumbprints".into(),
                    value: &client_certificate_thumbprints_binding,
                },
                register_interface::ObjectField {
                    name: "clusterCodeVersion".into(),
                    value: &cluster_code_version_binding,
                },
                register_interface::ObjectField {
                    name: "diagnosticsConfig".into(),
                    value: &diagnostics_config_binding,
                },
                register_interface::ObjectField {
                    name: "fabricSettings".into(),
                    value: &fabric_settings_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "managementEndpoint".into(),
                    value: &management_endpoint_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nodeTypes".into(),
                    value: &node_types_binding,
                },
                register_interface::ObjectField {
                    name: "reliabilityLevel".into(),
                    value: &reliability_level_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "reverseProxyCertificate".into(),
                    value: &reverse_proxy_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "reverseProxyCertificateCommonNames".into(),
                    value: &reverse_proxy_certificate_common_names_binding,
                },
                register_interface::ObjectField {
                    name: "serviceFabricZonalUpgradeMode".into(),
                    value: &service_fabric_zonal_upgrade_mode_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeMode".into(),
                    value: &upgrade_mode_binding,
                },
                register_interface::ObjectField {
                    name: "upgradePolicy".into(),
                    value: &upgrade_policy_binding,
                },
                register_interface::ObjectField {
                    name: "vmImage".into(),
                    value: &vm_image_binding,
                },
                register_interface::ObjectField {
                    name: "vmssZonalUpgradeMode".into(),
                    value: &vmss_zonal_upgrade_mode_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterResult {
            add_on_features: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addOnFeatures"),
            ),
            azure_active_directory: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureActiveDirectory"),
            ),
            certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificate"),
            ),
            certificate_common_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateCommonNames"),
            ),
            client_certificate_common_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertificateCommonNames"),
            ),
            client_certificate_thumbprints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertificateThumbprints"),
            ),
            cluster_code_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterCodeVersion"),
            ),
            cluster_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clusterEndpoint"),
            ),
            diagnostics_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("diagnosticsConfig"),
            ),
            fabric_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("fabricSettings"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            management_endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("managementEndpoint"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            node_types: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("nodeTypes"),
            ),
            reliability_level: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reliabilityLevel"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            reverse_proxy_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reverseProxyCertificate"),
            ),
            reverse_proxy_certificate_common_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("reverseProxyCertificateCommonNames"),
            ),
            service_fabric_zonal_upgrade_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceFabricZonalUpgradeMode"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            upgrade_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upgradeMode"),
            ),
            upgrade_policy: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upgradePolicy"),
            ),
            vm_image: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmImage"),
            ),
            vmss_zonal_upgrade_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vmssZonalUpgradeMode"),
            ),
        }
    }
}
