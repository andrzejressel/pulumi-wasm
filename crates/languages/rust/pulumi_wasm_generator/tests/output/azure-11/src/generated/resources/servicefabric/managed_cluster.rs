/// Manages a Resource Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = managed_cluster::create(
///         "example",
///         ManagedClusterArgs::builder()
///             .client_connection_port(12345)
///             .http_gateway_port(4567)
///             .lb_rules(
///                 vec![
///                     ManagedClusterLbRule::builder().backendPort(38080).frontendPort(80)
///                     .probeProtocol("http").probeRequestPath("/test").protocol("tcp")
///                     .build_struct(),
///                 ],
///             )
///             .location("West Europe")
///             .name("example")
///             .node_types(
///                 vec![
///                     ManagedClusterNodeType::builder().applicationPortRange("30000-49000")
///                     .dataDiskSizeGb(130).ephemeralPortRange("10000-20000").name("test1")
///                     .primary(true).vmImageOffer("WindowsServer")
///                     .vmImagePublisher("MicrosoftWindowsServer")
///                     .vmImageSku("2019-Datacenter-with-Containers")
///                     .vmImageVersion("latest").vmInstanceCount(5)
///                     .vmSize("Standard_DS1_v2").build_struct(),
///                 ],
///             )
///             .resource_group_name("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Resource Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:servicefabric/managedCluster:ManagedCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.ServiceFabric/managedClusters/clusterName1
/// ```
///
pub mod managed_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedClusterArgs {
        /// Controls how connections to the cluster are authenticated. A `authentication` block as defined below.
        #[builder(into, default)]
        pub authentication: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::servicefabric::ManagedClusterAuthentication>,
        >,
        /// If true, backup service is enabled.
        #[builder(into, default)]
        pub backup_service_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Port to use when connecting to the cluster.
        #[builder(into)]
        pub client_connection_port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// One or more `custom_fabric_setting` blocks as defined below.
        #[builder(into, default)]
        pub custom_fabric_settings: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::servicefabric::ManagedClusterCustomFabricSetting,
                >,
            >,
        >,
        /// Hostname for the cluster. If unset the cluster's name will be used..
        #[builder(into, default)]
        pub dns_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// If true, DNS service is enabled.
        #[builder(into, default)]
        pub dns_service_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Port that should be used by the Service Fabric Explorer to visualize applications and cluster status.
        #[builder(into)]
        pub http_gateway_port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// One or more `lb_rule` blocks as defined below.
        #[builder(into)]
        pub lb_rules: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::servicefabric::ManagedClusterLbRule>,
        >,
        /// The Azure Region where the Resource Group should exist. Changing this forces a new Resource Group to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Resource Group. Changing this forces a new Resource Group to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `node_type` blocks as defined below.
        #[builder(into, default)]
        pub node_types: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::servicefabric::ManagedClusterNodeType>>,
        >,
        /// Administrator password for the VMs that will be created as part of this cluster.
        #[builder(into, default)]
        pub password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Resource Group should exist. Changing this forces a new Resource Group to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// SKU for this cluster. Changing this forces a new resource to be created. Default is `Basic`, allowed values are either `Basic` or `Standard`.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Resource Group.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Upgrade wave for the fabric runtime. Default is `Wave0`, allowed value must be one of `Wave0`, `Wave1`, or `Wave2`.
        #[builder(into, default)]
        pub upgrade_wave: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Administrator password for the VMs that will be created as part of this cluster.
        #[builder(into, default)]
        pub username: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedClusterResult {
        /// Controls how connections to the cluster are authenticated. A `authentication` block as defined below.
        pub authentication: pulumi_wasm_rust::Output<
            Option<super::super::types::servicefabric::ManagedClusterAuthentication>,
        >,
        /// If true, backup service is enabled.
        pub backup_service_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Port to use when connecting to the cluster.
        pub client_connection_port: pulumi_wasm_rust::Output<i32>,
        /// One or more `custom_fabric_setting` blocks as defined below.
        pub custom_fabric_settings: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::types::servicefabric::ManagedClusterCustomFabricSetting,
                >,
            >,
        >,
        /// Hostname for the cluster. If unset the cluster's name will be used..
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// If true, DNS service is enabled.
        pub dns_service_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Port that should be used by the Service Fabric Explorer to visualize applications and cluster status.
        pub http_gateway_port: pulumi_wasm_rust::Output<i32>,
        /// One or more `lb_rule` blocks as defined below.
        pub lb_rules: pulumi_wasm_rust::Output<
            Vec<super::super::types::servicefabric::ManagedClusterLbRule>,
        >,
        /// The Azure Region where the Resource Group should exist. Changing this forces a new Resource Group to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Resource Group. Changing this forces a new Resource Group to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `node_type` blocks as defined below.
        pub node_types: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::servicefabric::ManagedClusterNodeType>>,
        >,
        /// Administrator password for the VMs that will be created as part of this cluster.
        pub password: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Resource Group should exist. Changing this forces a new Resource Group to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// SKU for this cluster. Changing this forces a new resource to be created. Default is `Basic`, allowed values are either `Basic` or `Standard`.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Resource Group.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Upgrade wave for the fabric runtime. Default is `Wave0`, allowed value must be one of `Wave0`, `Wave1`, or `Wave2`.
        pub upgrade_wave: pulumi_wasm_rust::Output<Option<String>>,
        /// Administrator password for the VMs that will be created as part of this cluster.
        pub username: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ManagedClusterArgs,
    ) -> ManagedClusterResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_binding = args.authentication.get_output(context).get_inner();
        let backup_service_enabled_binding = args
            .backup_service_enabled
            .get_output(context)
            .get_inner();
        let client_connection_port_binding = args
            .client_connection_port
            .get_output(context)
            .get_inner();
        let custom_fabric_settings_binding = args
            .custom_fabric_settings
            .get_output(context)
            .get_inner();
        let dns_name_binding = args.dns_name.get_output(context).get_inner();
        let dns_service_enabled_binding = args
            .dns_service_enabled
            .get_output(context)
            .get_inner();
        let http_gateway_port_binding = args
            .http_gateway_port
            .get_output(context)
            .get_inner();
        let lb_rules_binding = args.lb_rules.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let node_types_binding = args.node_types.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let upgrade_wave_binding = args.upgrade_wave.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:servicefabric/managedCluster:ManagedCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "backupServiceEnabled".into(),
                    value: &backup_service_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "clientConnectionPort".into(),
                    value: &client_connection_port_binding,
                },
                register_interface::ObjectField {
                    name: "customFabricSettings".into(),
                    value: &custom_fabric_settings_binding,
                },
                register_interface::ObjectField {
                    name: "dnsName".into(),
                    value: &dns_name_binding,
                },
                register_interface::ObjectField {
                    name: "dnsServiceEnabled".into(),
                    value: &dns_service_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "httpGatewayPort".into(),
                    value: &http_gateway_port_binding,
                },
                register_interface::ObjectField {
                    name: "lbRules".into(),
                    value: &lb_rules_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
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
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeWave".into(),
                    value: &upgrade_wave_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedClusterResult {
            authentication: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            backup_service_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("backupServiceEnabled"),
            ),
            client_connection_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clientConnectionPort"),
            ),
            custom_fabric_settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("customFabricSettings"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            dns_service_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsServiceEnabled"),
            ),
            http_gateway_port: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("httpGatewayPort"),
            ),
            lb_rules: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lbRules"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            node_types: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nodeTypes"),
            ),
            password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            upgrade_wave: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("upgradeWave"),
            ),
            username: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
