/// Manages an orbital contact.
///
/// > **Note:** The `azure.orbital.Contact` resource has been deprecated and will be removed in v5.0 of the AzureRM Provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: rg-example
///       location: West Europe
///   exampleSpacecraft:
///     type: azure:orbital:Spacecraft
///     name: example
///     properties:
///       name: example-spacecraft
///       resourceGroupName: ${example.name}
///       location: westeurope
///       noradId: '12345'
///       links:
///         - bandwidthMhz: 100
///           centerFrequencyMhz: 101
///           direction: Uplink
///           polarization: LHCP
///           name: examplename
///       twoLineElements:
///         - 1 23455U 94089A   97320.90946019  .00000140  00000-0  10191-3 0  2621
///         - 2 23455  99.0090 272.6745 0008546 223.1686 136.8816 14.11711747148495
///       titleLine: AQUA
///       tags:
///         aks-managed-cluster-name: 9a57225d-a405-4d40-aa46-f13d2342abef
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       delegations:
///         - name: orbitalgateway
///           serviceDelegation:
///             name: Microsoft.Orbital/orbitalGateways
///             actions:
///               - Microsoft.Network/publicIPAddresses/join/action
///               - Microsoft.Network/virtualNetworks/subnets/join/action
///               - Microsoft.Network/virtualNetworks/read
///               - Microsoft.Network/publicIPAddresses/read
///   exampleContactProfile:
///     type: azure:orbital:ContactProfile
///     name: example
///     properties:
///       name: example-contactprofile
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       minimumVariableContactDuration: PT1M
///       autoTracking: disabled
///       links:
///         - channels:
///             - name: channelname
///               bandwidthMhz: 100
///               centerFrequencyMhz: 101
///               endPoints:
///                 - endPointName: AQUA_command
///                   ipAddress: 10.0.1.0
///                   port: '49153'
///                   protocol: TCP
///           direction: Uplink
///           name: RHCP_UL
///           polarization: RHCP
///       networkConfigurationSubnetId: ${exampleSubnet.id}
///   exampleContact:
///     type: azure:orbital:Contact
///     name: example
///     properties:
///       name: example-contact
///       spacecraftId: ${exampleSpacecraft.id}
///       reservationStartTime: 2020-07-16T20:35:00.00Z
///       reservationEndTime: 2020-07-16T20:55:00.00Z
///       groundStationName: WESTUS2_0
///       contactProfileId: ${exampleContactProfile.id}
/// ```
///
/// ## Import
///
/// Spacecraft can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:orbital/contact:Contact example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Orbital/spacecrafts/spacecraft1/contacts/contact1
/// ```
///
pub mod contact {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactArgs {
        /// ID of the orbital contact profile. Changing this forces a new resource to be created.
        #[builder(into)]
        pub contact_profile_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Azure ground station. Changing this forces a new resource to be created.
        #[builder(into)]
        pub ground_station_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Contact. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Reservation end time of the Contact. Changing this forces a new resource to be created.
        #[builder(into)]
        pub reservation_end_time: pulumi_wasm_rust::Output<String>,
        /// Reservation start time of the Contact. Changing this forces a new resource to be created.
        #[builder(into)]
        pub reservation_start_time: pulumi_wasm_rust::Output<String>,
        /// The ID of the spacecraft which the contact will be made to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub spacecraft_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ContactResult {
        /// ID of the orbital contact profile. Changing this forces a new resource to be created.
        pub contact_profile_id: pulumi_wasm_rust::Output<String>,
        /// Name of the Azure ground station. Changing this forces a new resource to be created.
        pub ground_station_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Contact. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Reservation end time of the Contact. Changing this forces a new resource to be created.
        pub reservation_end_time: pulumi_wasm_rust::Output<String>,
        /// Reservation start time of the Contact. Changing this forces a new resource to be created.
        pub reservation_start_time: pulumi_wasm_rust::Output<String>,
        /// The ID of the spacecraft which the contact will be made to. Changing this forces a new resource to be created.
        pub spacecraft_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ContactArgs) -> ContactResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_profile_id_binding = args.contact_profile_id.get_inner();
        let ground_station_name_binding = args.ground_station_name.get_inner();
        let name_binding = args.name.get_inner();
        let reservation_end_time_binding = args.reservation_end_time.get_inner();
        let reservation_start_time_binding = args.reservation_start_time.get_inner();
        let spacecraft_id_binding = args.spacecraft_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:orbital/contact:Contact".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactProfileId".into(),
                    value: &contact_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "groundStationName".into(),
                    value: &ground_station_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "reservationEndTime".into(),
                    value: &reservation_end_time_binding,
                },
                register_interface::ObjectField {
                    name: "reservationStartTime".into(),
                    value: &reservation_start_time_binding,
                },
                register_interface::ObjectField {
                    name: "spacecraftId".into(),
                    value: &spacecraft_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "contactProfileId".into(),
                },
                register_interface::ResultField {
                    name: "groundStationName".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "reservationEndTime".into(),
                },
                register_interface::ResultField {
                    name: "reservationStartTime".into(),
                },
                register_interface::ResultField {
                    name: "spacecraftId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ContactResult {
            contact_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("contactProfileId").unwrap(),
            ),
            ground_station_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("groundStationName").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            reservation_end_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationEndTime").unwrap(),
            ),
            reservation_start_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reservationStartTime").unwrap(),
            ),
            spacecraft_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("spacecraftId").unwrap(),
            ),
        }
    }
}