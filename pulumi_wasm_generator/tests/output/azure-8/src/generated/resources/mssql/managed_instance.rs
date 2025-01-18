/// Manages a Microsoft SQL Azure Managed Instance.
///
/// > **Note:** All arguments including the administrator login and password will be stored in the raw state as plain-text. [Read more about sensitive data in state](https://www.terraform.io/docs/state/sensitive-data.html).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: database-rg
///       location: West Europe
///   exampleNetworkSecurityGroup:
///     type: azure:network:NetworkSecurityGroup
///     name: example
///     properties:
///       name: mi-security-group
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   allowManagementInbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_management_inbound
///     properties:
///       name: allow_management_inbound
///       priority: 106
///       direction: Inbound
///       access: Allow
///       protocol: Tcp
///       sourcePortRange: '*'
///       destinationPortRanges:
///         - '9000'
///         - '9003'
///         - '1438'
///         - '1440'
///         - '1452'
///       sourceAddressPrefix: '*'
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   allowMisubnetInbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_misubnet_inbound
///     properties:
///       name: allow_misubnet_inbound
///       priority: 200
///       direction: Inbound
///       access: Allow
///       protocol: '*'
///       sourcePortRange: '*'
///       destinationPortRange: '*'
///       sourceAddressPrefix: 10.0.0.0/24
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   allowHealthProbeInbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_health_probe_inbound
///     properties:
///       name: allow_health_probe_inbound
///       priority: 300
///       direction: Inbound
///       access: Allow
///       protocol: '*'
///       sourcePortRange: '*'
///       destinationPortRange: '*'
///       sourceAddressPrefix: AzureLoadBalancer
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   allowTdsInbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_tds_inbound
///     properties:
///       name: allow_tds_inbound
///       priority: 1000
///       direction: Inbound
///       access: Allow
///       protocol: Tcp
///       sourcePortRange: '*'
///       destinationPortRange: '1433'
///       sourceAddressPrefix: VirtualNetwork
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   denyAllInbound:
///     type: azure:network:NetworkSecurityRule
///     name: deny_all_inbound
///     properties:
///       name: deny_all_inbound
///       priority: 4096
///       direction: Inbound
///       access: Deny
///       protocol: '*'
///       sourcePortRange: '*'
///       destinationPortRange: '*'
///       sourceAddressPrefix: '*'
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   allowManagementOutbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_management_outbound
///     properties:
///       name: allow_management_outbound
///       priority: 102
///       direction: Outbound
///       access: Allow
///       protocol: Tcp
///       sourcePortRange: '*'
///       destinationPortRanges:
///         - '80'
///         - '443'
///         - '12000'
///       sourceAddressPrefix: '*'
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   allowMisubnetOutbound:
///     type: azure:network:NetworkSecurityRule
///     name: allow_misubnet_outbound
///     properties:
///       name: allow_misubnet_outbound
///       priority: 200
///       direction: Outbound
///       access: Allow
///       protocol: '*'
///       sourcePortRange: '*'
///       destinationPortRange: '*'
///       sourceAddressPrefix: 10.0.0.0/24
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   denyAllOutbound:
///     type: azure:network:NetworkSecurityRule
///     name: deny_all_outbound
///     properties:
///       name: deny_all_outbound
///       priority: 4096
///       direction: Outbound
///       access: Deny
///       protocol: '*'
///       sourcePortRange: '*'
///       destinationPortRange: '*'
///       sourceAddressPrefix: '*'
///       destinationAddressPrefix: '*'
///       resourceGroupName: ${example.name}
///       networkSecurityGroupName: ${exampleNetworkSecurityGroup.name}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: vnet-mi
///       resourceGroupName: ${example.name}
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: subnet-mi
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.0.0/24
///       delegations:
///         - name: managedinstancedelegation
///           serviceDelegation:
///             name: Microsoft.Sql/managedInstances
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///               - Microsoft.Network/virtualNetworks/subnets/prepareNetworkPolicies/action
///               - Microsoft.Network/virtualNetworks/subnets/unprepareNetworkPolicies/action
///   exampleSubnetNetworkSecurityGroupAssociation:
///     type: azure:network:SubnetNetworkSecurityGroupAssociation
///     name: example
///     properties:
///       subnetId: ${exampleSubnet.id}
///       networkSecurityGroupId: ${exampleNetworkSecurityGroup.id}
///   exampleRouteTable:
///     type: azure:network:RouteTable
///     name: example
///     properties:
///       name: routetable-mi
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       disableBgpRoutePropagation: false
///     options:
///       dependsOn:
///         - ${exampleSubnet}
///   exampleSubnetRouteTableAssociation:
///     type: azure:network:SubnetRouteTableAssociation
///     name: example
///     properties:
///       subnetId: ${exampleSubnet.id}
///       routeTableId: ${exampleRouteTable.id}
///   exampleManagedInstance:
///     type: azure:mssql:ManagedInstance
///     name: example
///     properties:
///       name: managedsqlinstance
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       licenseType: BasePrice
///       skuName: GP_Gen5
///       storageSizeInGb: 32
///       subnetId: ${exampleSubnet.id}
///       vcores: 4
///       administratorLogin: mradministrator
///       administratorLoginPassword: thisIsDog11
///     options:
///       dependsOn:
///         - ${exampleSubnetNetworkSecurityGroupAssociation}
///         - ${exampleSubnetRouteTableAssociation}
/// ```
///
/// ## Import
///
/// Microsoft SQL Managed Instances can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/managedInstance:ManagedInstance example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myresourcegroup/providers/Microsoft.Sql/managedInstances/myserver
/// ```
///
pub mod managed_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedInstanceArgs {
        /// The administrator login name for the new SQL Managed Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The password associated with the `administrator_login` user. Needs to comply with Azure's [Password Policy](https://msdn.microsoft.com/library/ms161959.aspx)
        #[builder(into)]
        pub administrator_login_password: pulumi_wasm_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be collated. Default value is `SQL_Latin1_General_CP1_CI_AS`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub collation: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the SQL Managed Instance which will share the DNS zone. This is a prerequisite for creating an `azurerm_sql_managed_instance_failover_group`. Setting this after creation forces a new resource to be created.
        #[builder(into, default)]
        pub dns_zone_partner_id: pulumi_wasm_rust::Output<Option<String>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ManagedInstanceIdentity>,
        >,
        /// What type of license the Managed Instance will use. Possible values are `LicenseIncluded` and `BasePrice`.
        #[builder(into)]
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Public Maintenance Configuration window to apply to the SQL Managed Instance. Valid values include `SQL_Default` or an Azure Location in the format `SQL_{Location}_MI_{Size}`(for example `SQL_EastUS_MI_1`). Defaults to `SQL_Default`.
        #[builder(into, default)]
        pub maintenance_configuration_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Minimum TLS Version. Default value is `1.2` Valid values include `1.0`, `1.1`, `1.2`.
        #[builder(into, default)]
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the SQL Managed Instance. This needs to be globally unique within Azure. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies how the SQL Managed Instance will be accessed. Default value is `Default`. Valid values include `Default`, `Proxy`, and `Redirect`.
        #[builder(into, default)]
        pub proxy_override: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the public data endpoint enabled? Default value is `false`.
        #[builder(into, default)]
        pub public_data_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the SQL Managed Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The service principal type. The only possible value is `SystemAssigned`.
        #[builder(into, default)]
        pub service_principal_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU Name for the SQL Managed Instance. Valid values include `GP_Gen4`, `GP_Gen5`, `GP_Gen8IM`, `GP_Gen8IH`, `BC_Gen4`, `BC_Gen5`, `BC_Gen8IM` or `BC_Gen8IH`.
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account type used to store backups for this database. Possible values are `GRS`, `GZRS`, `LRS`, and `ZRS`. Defaults to `GRS`.
        #[builder(into, default)]
        pub storage_account_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum storage space for the SQL Managed instance. This should be a multiple of 32 (GB).
        #[builder(into)]
        pub storage_size_in_gb: pulumi_wasm_rust::Output<i32>,
        /// The subnet resource id that the SQL Managed Instance will be associated with. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The TimeZone ID that the SQL Managed Instance will be operating in. Default value is `UTC`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub timezone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of cores that should be assigned to the SQL Managed Instance. Values can be `8`, `16`, or `24` for Gen4 SKUs, or `4`, `6`, `8`, `10`, `12`, `16`, `20`, `24`, `32`, `40`, `48`, `56`, `64`, `80`, `96` or `128` for Gen5 SKUs.
        #[builder(into)]
        pub vcores: pulumi_wasm_rust::Output<i32>,
        /// Specifies whether or not the SQL Managed Instance is zone redundant. Defaults to `false`.
        #[builder(into, default)]
        pub zone_redundant_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ManagedInstanceResult {
        /// The administrator login name for the new SQL Managed Instance. Changing this forces a new resource to be created.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The password associated with the `administrator_login` user. Needs to comply with Azure's [Password Policy](https://msdn.microsoft.com/library/ms161959.aspx)
        pub administrator_login_password: pulumi_wasm_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be collated. Default value is `SQL_Latin1_General_CP1_CI_AS`. Changing this forces a new resource to be created.
        pub collation: pulumi_wasm_rust::Output<Option<String>>,
        /// The Dns Zone where the SQL Managed Instance is located.
        pub dns_zone: pulumi_wasm_rust::Output<String>,
        /// The ID of the SQL Managed Instance which will share the DNS zone. This is a prerequisite for creating an `azurerm_sql_managed_instance_failover_group`. Setting this after creation forces a new resource to be created.
        pub dns_zone_partner_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The fully qualified domain name of the Azure Managed SQL Instance
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::mssql::ManagedInstanceIdentity>,
        >,
        /// What type of license the Managed Instance will use. Possible values are `LicenseIncluded` and `BasePrice`.
        pub license_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the Public Maintenance Configuration window to apply to the SQL Managed Instance. Valid values include `SQL_Default` or an Azure Location in the format `SQL_{Location}_MI_{Size}`(for example `SQL_EastUS_MI_1`). Defaults to `SQL_Default`.
        pub maintenance_configuration_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The Minimum TLS Version. Default value is `1.2` Valid values include `1.0`, `1.1`, `1.2`.
        pub minimum_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the SQL Managed Instance. This needs to be globally unique within Azure. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies how the SQL Managed Instance will be accessed. Default value is `Default`. Valid values include `Default`, `Proxy`, and `Redirect`.
        pub proxy_override: pulumi_wasm_rust::Output<Option<String>>,
        /// Is the public data endpoint enabled? Default value is `false`.
        pub public_data_endpoint_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the SQL Managed Instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The service principal type. The only possible value is `SystemAssigned`.
        pub service_principal_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU Name for the SQL Managed Instance. Valid values include `GP_Gen4`, `GP_Gen5`, `GP_Gen8IM`, `GP_Gen8IH`, `BC_Gen4`, `BC_Gen5`, `BC_Gen8IM` or `BC_Gen8IH`.
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the storage account type used to store backups for this database. Possible values are `GRS`, `GZRS`, `LRS`, and `ZRS`. Defaults to `GRS`.
        pub storage_account_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Maximum storage space for the SQL Managed instance. This should be a multiple of 32 (GB).
        pub storage_size_in_gb: pulumi_wasm_rust::Output<i32>,
        /// The subnet resource id that the SQL Managed Instance will be associated with. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The TimeZone ID that the SQL Managed Instance will be operating in. Default value is `UTC`. Changing this forces a new resource to be created.
        pub timezone_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Number of cores that should be assigned to the SQL Managed Instance. Values can be `8`, `16`, or `24` for Gen4 SKUs, or `4`, `6`, `8`, `10`, `12`, `16`, `20`, `24`, `32`, `40`, `48`, `56`, `64`, `80`, `96` or `128` for Gen5 SKUs.
        pub vcores: pulumi_wasm_rust::Output<i32>,
        /// Specifies whether or not the SQL Managed Instance is zone redundant. Defaults to `false`.
        pub zone_redundant_enabled: pulumi_wasm_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ManagedInstanceArgs) -> ManagedInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_login_binding = args.administrator_login.get_inner();
        let administrator_login_password_binding = args
            .administrator_login_password
            .get_inner();
        let collation_binding = args.collation.get_inner();
        let dns_zone_partner_id_binding = args.dns_zone_partner_id.get_inner();
        let identity_binding = args.identity.get_inner();
        let license_type_binding = args.license_type.get_inner();
        let location_binding = args.location.get_inner();
        let maintenance_configuration_name_binding = args
            .maintenance_configuration_name
            .get_inner();
        let minimum_tls_version_binding = args.minimum_tls_version.get_inner();
        let name_binding = args.name.get_inner();
        let proxy_override_binding = args.proxy_override.get_inner();
        let public_data_endpoint_enabled_binding = args
            .public_data_endpoint_enabled
            .get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_principal_type_binding = args.service_principal_type.get_inner();
        let sku_name_binding = args.sku_name.get_inner();
        let storage_account_type_binding = args.storage_account_type.get_inner();
        let storage_size_in_gb_binding = args.storage_size_in_gb.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let timezone_id_binding = args.timezone_id.get_inner();
        let vcores_binding = args.vcores.get_inner();
        let zone_redundant_enabled_binding = args.zone_redundant_enabled.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/managedInstance:ManagedInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorLogin".into(),
                    value: &administrator_login_binding,
                },
                register_interface::ObjectField {
                    name: "administratorLoginPassword".into(),
                    value: &administrator_login_password_binding,
                },
                register_interface::ObjectField {
                    name: "collation".into(),
                    value: &collation_binding,
                },
                register_interface::ObjectField {
                    name: "dnsZonePartnerId".into(),
                    value: &dns_zone_partner_id_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceConfigurationName".into(),
                    value: &maintenance_configuration_name_binding,
                },
                register_interface::ObjectField {
                    name: "minimumTlsVersion".into(),
                    value: &minimum_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "proxyOverride".into(),
                    value: &proxy_override_binding,
                },
                register_interface::ObjectField {
                    name: "publicDataEndpointEnabled".into(),
                    value: &public_data_endpoint_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "servicePrincipalType".into(),
                    value: &service_principal_type_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountType".into(),
                    value: &storage_account_type_binding,
                },
                register_interface::ObjectField {
                    name: "storageSizeInGb".into(),
                    value: &storage_size_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timezoneId".into(),
                    value: &timezone_id_binding,
                },
                register_interface::ObjectField {
                    name: "vcores".into(),
                    value: &vcores_binding,
                },
                register_interface::ObjectField {
                    name: "zoneRedundantEnabled".into(),
                    value: &zone_redundant_enabled_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "administratorLoginPassword".into(),
                },
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "dnsZone".into(),
                },
                register_interface::ResultField {
                    name: "dnsZonePartnerId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "licenseType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maintenanceConfigurationName".into(),
                },
                register_interface::ResultField {
                    name: "minimumTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "proxyOverride".into(),
                },
                register_interface::ResultField {
                    name: "publicDataEndpointEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "servicePrincipalType".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountType".into(),
                },
                register_interface::ResultField {
                    name: "storageSizeInGb".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timezoneId".into(),
                },
                register_interface::ResultField {
                    name: "vcores".into(),
                },
                register_interface::ResultField {
                    name: "zoneRedundantEnabled".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedInstanceResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            administrator_login_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLoginPassword").unwrap(),
            ),
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            dns_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsZone").unwrap(),
            ),
            dns_zone_partner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsZonePartnerId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            license_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenseType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maintenance_configuration_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maintenanceConfigurationName").unwrap(),
            ),
            minimum_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minimumTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            proxy_override: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("proxyOverride").unwrap(),
            ),
            public_data_endpoint_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicDataEndpointEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_principal_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("servicePrincipalType").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            storage_account_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountType").unwrap(),
            ),
            storage_size_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageSizeInGb").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            timezone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timezoneId").unwrap(),
            ),
            vcores: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vcores").unwrap(),
            ),
            zone_redundant_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneRedundantEnabled").unwrap(),
            ),
        }
    }
}
