/// Manages a Security Alert Policy for an MS SQL Managed Instance.
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
///   exampleManagedInstanceSecurityAlertPolicy:
///     type: azure:mssql:ManagedInstanceSecurityAlertPolicy
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       managedInstanceName: ${exampleManagedInstance.name}
///       enabled: true
///       storageEndpoint: ${exampleAzurermStorageAccount.primaryBlobEndpoint}
///       storageAccountAccessKey: ${exampleAzurermStorageAccount.primaryAccessKey}
///       disabledAlerts:
///         - Sql_Injection
///         - Data_Exfiltration
///       retentionDays: 20
/// ```
///
/// ## Import
///
/// MS SQL Managed Instance Security Alert Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/managedInstanceSecurityAlertPolicy:ManagedInstanceSecurityAlertPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/acceptanceTestResourceGroup1/providers/Microsoft.Sql/managedInstances/instance1/securityAlertPolicies/Default
/// ```
///
pub mod managed_instance_security_alert_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedInstanceSecurityAlertPolicyArgs {
        /// Specifies an array of alerts that are disabled. Possible values are `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action` and `Brute_Force`.
        #[builder(into, default)]
        pub disabled_alerts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        #[builder(into, default)]
        pub email_account_admins_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        #[builder(into, default)]
        pub email_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the state of the Security Alert Policy, whether it is enabled or disabled. Possible values are `true`, `false`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the MS SQL Managed Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub managed_instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group that contains the MS SQL Managed Instance. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        #[builder(into, default)]
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the identifier key of the Threat Detection audit storage account. This is mandatory when you use `storage_endpoint` to specify a storage account blob endpoint.
        ///
        /// > **NOTE:**  Please note that storage accounts configured with `shared_access_key_enabled = false` cannot be used to configure `azure.mssql.ManagedInstanceSecurityAlertPolicy` with `storage_endpoint` for now.
        #[builder(into, default)]
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. https://example.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs.
        #[builder(into, default)]
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ManagedInstanceSecurityAlertPolicyResult {
        /// Specifies an array of alerts that are disabled. Possible values are `Sql_Injection`, `Sql_Injection_Vulnerability`, `Access_Anomaly`, `Data_Exfiltration`, `Unsafe_Action` and `Brute_Force`.
        pub disabled_alerts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Boolean flag which specifies if the alert is sent to the account administrators or not. Defaults to `false`.
        pub email_account_admins_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies an array of email addresses to which the alert is sent.
        pub email_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the state of the Security Alert Policy, whether it is enabled or disabled. Possible values are `true`, `false`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the MS SQL Managed Instance. Changing this forces a new resource to be created.
        pub managed_instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group that contains the MS SQL Managed Instance. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the number of days to keep in the Threat Detection audit logs. Defaults to `0`.
        pub retention_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// Specifies the identifier key of the Threat Detection audit storage account. This is mandatory when you use `storage_endpoint` to specify a storage account blob endpoint.
        ///
        /// > **NOTE:**  Please note that storage accounts configured with `shared_access_key_enabled = false` cannot be used to configure `azure.mssql.ManagedInstanceSecurityAlertPolicy` with `storage_endpoint` for now.
        pub storage_account_access_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the blob storage endpoint (e.g. https://example.blob.core.windows.net). This blob storage will hold all Threat Detection audit logs.
        pub storage_endpoint: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ManagedInstanceSecurityAlertPolicyArgs,
    ) -> ManagedInstanceSecurityAlertPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let disabled_alerts_binding = args.disabled_alerts.get_inner();
        let email_account_admins_enabled_binding = args
            .email_account_admins_enabled
            .get_inner();
        let email_addresses_binding = args.email_addresses.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let managed_instance_name_binding = args.managed_instance_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let retention_days_binding = args.retention_days.get_inner();
        let storage_account_access_key_binding = args
            .storage_account_access_key
            .get_inner();
        let storage_endpoint_binding = args.storage_endpoint.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mssql/managedInstanceSecurityAlertPolicy:ManagedInstanceSecurityAlertPolicy"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "disabledAlerts".into(),
                    value: &disabled_alerts_binding,
                },
                register_interface::ObjectField {
                    name: "emailAccountAdminsEnabled".into(),
                    value: &email_account_admins_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "emailAddresses".into(),
                    value: &email_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "managedInstanceName".into(),
                    value: &managed_instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "retentionDays".into(),
                    value: &retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "storageAccountAccessKey".into(),
                    value: &storage_account_access_key_binding,
                },
                register_interface::ObjectField {
                    name: "storageEndpoint".into(),
                    value: &storage_endpoint_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "disabledAlerts".into(),
                },
                register_interface::ResultField {
                    name: "emailAccountAdminsEnabled".into(),
                },
                register_interface::ResultField {
                    name: "emailAddresses".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "managedInstanceName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "retentionDays".into(),
                },
                register_interface::ResultField {
                    name: "storageAccountAccessKey".into(),
                },
                register_interface::ResultField {
                    name: "storageEndpoint".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedInstanceSecurityAlertPolicyResult {
            disabled_alerts: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("disabledAlerts").unwrap(),
            ),
            email_account_admins_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAccountAdminsEnabled").unwrap(),
            ),
            email_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emailAddresses").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            managed_instance_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedInstanceName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("retentionDays").unwrap(),
            ),
            storage_account_access_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageAccountAccessKey").unwrap(),
            ),
            storage_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageEndpoint").unwrap(),
            ),
        }
    }
}