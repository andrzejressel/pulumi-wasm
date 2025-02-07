/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: random:RandomString
///     properties:
///       length: 12
///       special: true
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: ${prefix}-resources
///       location: ${location}
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: ${prefix}-vnet
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       addressSpaces:
///         - 10.6.0.0/16
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: ${prefix}-delegated-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.6.2.0/24
///       delegations:
///         - name: testdelegation
///           serviceDelegation:
///             name: Microsoft.Netapp/volumes
///             actions:
///               - Microsoft.Network/networkinterfaces/*
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///   example1:
///     type: azure:network:Subnet
///     properties:
///       name: ${prefix}-hosts-subnet
///       resourceGroupName: ${exampleResourceGroup.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.6.1.0/24
///   examplePlacementGroup:
///     type: azure:proximity:PlacementGroup
///     name: example
///     properties:
///       name: ${prefix}-ppg
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleAvailabilitySet:
///     type: azure:compute:AvailabilitySet
///     name: example
///     properties:
///       name: ${prefix}-avset
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       proximityPlacementGroupId: ${examplePlacementGroup.id}
///   exampleNetworkInterface:
///     type: azure:network:NetworkInterface
///     name: example
///     properties:
///       name: ${prefix}-nic
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       ipConfigurations:
///         - name: internal
///           subnetId: ${example1.id}
///           privateIpAddressAllocation: Dynamic
///   exampleLinuxVirtualMachine:
///     type: azure:compute:LinuxVirtualMachine
///     name: example
///     properties:
///       name: ${prefix}-vm
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       size: Standard_M8ms
///       adminUsername: ${adminUsername}
///       adminPassword: ${adminPassword}
///       disablePasswordAuthentication: false
///       proximityPlacementGroupId: ${examplePlacementGroup.id}
///       availabilitySetId: ${exampleAvailabilitySet.id}
///       networkInterfaceIds:
///         - ${exampleNetworkInterface.id}
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///       osDisk:
///         storageAccountType: Standard_LRS
///         caching: ReadWrite
///   exampleAccount:
///     type: azure:netapp:Account
///     name: example
///     properties:
///       name: ${prefix}-netapp-account
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///     options:
///       dependsOn:
///         - ${exampleSubnet}
///         - ${example1}
///   examplePool:
///     type: azure:netapp:Pool
///     name: example
///     properties:
///       name: ${prefix}-netapp-pool
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountName: ${exampleAccount.name}
///       serviceLevel: Standard
///       sizeInTb: 8
///       qosType: Manual
///   exampleVolumeGroupSapHana:
///     type: azure:netapp:VolumeGroupSapHana
///     name: example
///     properties:
///       name: ${prefix}-netapp-volumegroup
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       accountName: ${exampleAccount.name}
///       groupDescription: Test volume group
///       applicationIdentifier: TST
///       volumes:
///         - name: ${prefix}-netapp-volume-1
///           volumePath: my-unique-file-path-1
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: data
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///           tags:
///             foo: bar
///         - name: ${prefix}-netapp-volume-2
///           volumePath: my-unique-file-path-2
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: log
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///           tags:
///             foo: bar
///         - name: ${prefix}-netapp-volume-3
///           volumePath: my-unique-file-path-3
///           serviceLevel: Standard
///           capacityPoolId: ${examplePool.id}
///           subnetId: ${exampleSubnet.id}
///           proximityPlacementGroupId: ${examplePlacementGroup.id}
///           volumeSpecName: shared
///           storageQuotaInGb: 1024
///           throughputInMibps: 24
///           protocols: NFSv4.1
///           securityStyle: unix
///           snapshotDirectoryVisible: false
///           exportPolicyRules:
///             - ruleIndex: 1
///               allowedClients: 0.0.0.0/0
///               nfsv3Enabled: false
///               nfsv41Enabled: true
///               unixReadOnly: false
///               unixReadWrite: true
///               rootAccessEnabled: false
///     options:
///       dependsOn:
///         - ${exampleLinuxVirtualMachine}
///         - ${examplePlacementGroup}
/// variables:
///   adminUsername: exampleadmin
///   adminPassword: ${example.result}
/// ```
///
/// ## Import
///
/// Application Volume Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:netapp/volumeGroupSapHana:VolumeGroupSapHana example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mytest-rg/providers/Microsoft.NetApp/netAppAccounts/netapp-account-test/volumeGroups/netapp-volumegroup-test
/// ```
///
pub mod volume_group_sap_hana {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VolumeGroupSapHanaArgs {
        /// Name of the account where the application volume group belong to. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub account_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SAP System ID, maximum 3 characters, e.g. `SH9`. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub application_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Volume group description. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub group_description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Azure Region where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for this Application Volume Group. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// One or more `volume` blocks as defined below.
        #[builder(into)]
        pub volumes: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::netapp::VolumeGroupSapHanaVolume>,
        >,
    }
    #[allow(dead_code)]
    pub struct VolumeGroupSapHanaResult {
        /// Name of the account where the application volume group belong to. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub account_name: pulumi_gestalt_rust::Output<String>,
        /// The SAP System ID, maximum 3 characters, e.g. `SH9`. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub application_identifier: pulumi_gestalt_rust::Output<String>,
        /// Volume group description. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub group_description: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Application Volume Group. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Application Volume Group should exist. Changing this forces a new Application Volume Group to be created and data will be lost.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// One or more `volume` blocks as defined below.
        pub volumes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::netapp::VolumeGroupSapHanaVolume>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VolumeGroupSapHanaArgs,
    ) -> VolumeGroupSapHanaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let account_name_binding = args.account_name.get_output(context).get_inner();
        let application_identifier_binding = args
            .application_identifier
            .get_output(context)
            .get_inner();
        let group_description_binding = args
            .group_description
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let volumes_binding = args.volumes.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:netapp/volumeGroupSapHana:VolumeGroupSapHana".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountName".into(),
                    value: &account_name_binding,
                },
                register_interface::ObjectField {
                    name: "applicationIdentifier".into(),
                    value: &application_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "groupDescription".into(),
                    value: &group_description_binding,
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
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "volumes".into(),
                    value: &volumes_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VolumeGroupSapHanaResult {
            account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accountName"),
            ),
            application_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationIdentifier"),
            ),
            group_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("groupDescription"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            volumes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumes"),
            ),
        }
    }
}
