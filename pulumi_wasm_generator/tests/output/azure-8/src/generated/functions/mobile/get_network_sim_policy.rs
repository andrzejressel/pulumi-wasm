pub mod get_network_sim_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSimPolicyArgs {
        /// The ID of the Mobile Network which the Sim Policy belongs to.
        #[builder(into)]
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim Policies.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSimPolicyResult {
        /// The ID of default slice to use if the UE does not explicitly specify it.
        pub default_slice_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The Azure Region where the Mobile Network Sim Policy should exist.
        pub location: pulumi_wasm_rust::Output<String>,
        pub mobile_network_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        pub rat_frequency_selection_priority_index: pulumi_wasm_rust::Output<i32>,
        /// Interval for the UE periodic registration update procedure.
        pub registration_timer_in_seconds: pulumi_wasm_rust::Output<i32>,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them.
        pub slices: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        pub user_equipment_aggregate_maximum_bit_rates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetNetworkSimPolicyArgs) -> GetNetworkSimPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let mobile_network_id_binding = args.mobile_network_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:mobile/getNetworkSimPolicy:getNetworkSimPolicy".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "mobileNetworkId".into(),
                    value: &mobile_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "defaultSliceId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
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
                    name: "userEquipmentAggregateMaximumBitRates".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetNetworkSimPolicyResult {
            default_slice_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultSliceId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
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
            user_equipment_aggregate_maximum_bit_rates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("userEquipmentAggregateMaximumBitRates").unwrap(),
            ),
        }
    }
}
