/// Manages a Mobile Network Sim Policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNetwork:
///     type: azure:mobile:Network
///     name: example
///     properties:
///       name: example-mn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       mobileCountryCode: '001'
///       mobileNetworkCode: '01'
///   exampleNetworkDataNetwork:
///     type: azure:mobile:NetworkDataNetwork
///     name: example
///     properties:
///       name: example-mndn
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///   exampleNetworkService:
///     type: azure:mobile:NetworkService
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       servicePrecedence: 0
///       pccRules:
///         - name: default-rule
///           precedence: 1
///           trafficControlEnabled: true
///           serviceDataFlowTemplates:
///             - direction: Uplink
///               name: IP-to-server
///               ports: []
///               protocols:
///                 - ip
///               remoteIpLists:
///                 - 10.3.4.0/24
///   exampleNetworkSlice:
///     type: azure:mobile:NetworkSlice
///     name: example
///     properties:
///       name: example-mns
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       singleNetworkSliceSelectionAssistanceInformation:
///         sliceServiceType: 1
///   exampleNetworkSimPolicy:
///     type: azure:mobile:NetworkSimPolicy
///     name: example
///     properties:
///       name: example-mnsp
///       mobileNetworkId: ${exampleNetwork.id}
///       location: ${example.location}
///       registrationTimerInSeconds: 3240
///       defaultSliceId: ${exampleNetworkSlice.id}
///       slices:
///         - defaultDataNetworkId: ${exampleNetworkDataNetwork.id}
///           sliceId: ${exampleNetworkSlice.id}
///           dataNetworks:
///             - dataNetworkId: ${exampleNetworkDataNetwork.id}
///               allocationAndRetentionPriorityLevel: 9
///               defaultSessionType: IPv4
///               qosIndicator: 9
///               preemptionCapability: NotPreempt
///               preemptionVulnerability: Preemptable
///               allowedServicesIds:
///                 - ${exampleNetworkService.id}
///               sessionAggregateMaximumBitRate:
///                 downlink: 1 Gbps
///                 uplink: 500 Mbps
///       userEquipmentAggregateMaximumBitRate:
///         downlink: 1 Gbps
///         uplink: 500 Mbps
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Mobile Network Sim Policies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSimPolicy:NetworkSimPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/mobileNetworks/mobileNetwork1/simPolicies/simPolicy1
/// ```
///
pub mod network_sim_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimPolicyArgs {
        /// The ID of default slice to use if the UE does not explicitly specify it. This slice must exist in the `slice` block.
        #[builder(into)]
        pub default_slice_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Mobile Network Sim Policy should exist. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Mobile Network which the Sim Policy belongs to. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim Policies. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        #[builder(into, default)]
        pub rat_frequency_selection_priority_index: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Interval for the user equipment periodic registration update procedure. Defaults to `3240`.
        #[builder(into, default)]
        pub registration_timer_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them. The list must not contain duplicate items and must contain at least one item.
        #[builder(into)]
        pub slices: pulumi_wasm_rust::Output<
            Vec<super::super::types::mobile::NetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        #[builder(into)]
        pub user_equipment_aggregate_maximum_bit_rate: pulumi_wasm_rust::Output<
            super::super::types::mobile::NetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimPolicyResult {
        /// The ID of default slice to use if the UE does not explicitly specify it. This slice must exist in the `slice` block.
        pub default_slice_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Mobile Network Sim Policy should exist. Changing this forces a new Mobile Network Sim Policies to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The ID of the Mobile Network which the Sim Policy belongs to. Changing this forces a new Mobile Network Sim Policies to be created.
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim Policies. Changing this forces a new Mobile Network Sim Policies to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        pub rat_frequency_selection_priority_index: pulumi_wasm_rust::Output<
            Option<i32>,
        >,
        /// Interval for the user equipment periodic registration update procedure. Defaults to `3240`.
        pub registration_timer_in_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them. The list must not contain duplicate items and must contain at least one item.
        pub slices: pulumi_wasm_rust::Output<
            Vec<super::super::types::mobile::NetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        pub user_equipment_aggregate_maximum_bit_rate: pulumi_wasm_rust::Output<
            super::super::types::mobile::NetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: NetworkSimPolicyArgs) -> NetworkSimPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_slice_id_binding = args.default_slice_id.get_inner();
        let location_binding = args.location.get_inner();
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let rat_frequency_selection_priority_index_binding = args
            .rat_frequency_selection_priority_index
            .get_inner();
        let registration_timer_in_seconds_binding = args
            .registration_timer_in_seconds
            .get_inner();
        let slices_binding = args.slices.get_inner();
        let tags_binding = args.tags.get_inner();
        let user_equipment_aggregate_maximum_bit_rate_binding = args
            .user_equipment_aggregate_maximum_bit_rate
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:mobile/networkSimPolicy:NetworkSimPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultSliceId".into(),
                    value: &default_slice_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ratFrequencySelectionPriorityIndex".into(),
                    value: &rat_frequency_selection_priority_index_binding,
                },
                register_interface::ObjectField {
                    name: "registrationTimerInSeconds".into(),
                    value: &registration_timer_in_seconds_binding,
                },
                register_interface::ObjectField {
                    name: "slices".into(),
                    value: &slices_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "userEquipmentAggregateMaximumBitRate".into(),
                    value: &user_equipment_aggregate_maximum_bit_rate_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultSliceId".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mobileNetworkId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ratFrequencySelectionPriorityIndex".into(),
                },
                register_interface::ResultField {
                    name: "registrationTimerInSeconds".into(),
                },
                register_interface::ResultField {
                    name: "slices".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "userEquipmentAggregateMaximumBitRate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NetworkSimPolicyResult {
            default_slice_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSliceId").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mobile_network_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mobileNetworkId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rat_frequency_selection_priority_index: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ratFrequencySelectionPriorityIndex").unwrap(),
            ),
            registration_timer_in_seconds: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("registrationTimerInSeconds").unwrap(),
            ),
            slices: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("slices").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            user_equipment_aggregate_maximum_bit_rate: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEquipmentAggregateMaximumBitRate").unwrap(),
            ),
        }
    }
}
