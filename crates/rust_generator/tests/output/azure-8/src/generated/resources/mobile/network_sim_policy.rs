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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_sim_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimPolicyArgs {
        /// The ID of default slice to use if the UE does not explicitly specify it. This slice must exist in the `slice` block.
        #[builder(into)]
        pub default_slice_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the Azure Region where the Mobile Network Sim Policy should exist. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Mobile Network which the Sim Policy belongs to. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim Policies. Changing this forces a new Mobile Network Sim Policies to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        #[builder(into, default)]
        pub rat_frequency_selection_priority_index: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Interval for the user equipment periodic registration update procedure. Defaults to `3240`.
        #[builder(into, default)]
        pub registration_timer_in_seconds: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them. The list must not contain duplicate items and must contain at least one item.
        #[builder(into)]
        pub slices: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::mobile::NetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        #[builder(into)]
        pub user_equipment_aggregate_maximum_bit_rate: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mobile::NetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimPolicyResult {
        /// The ID of default slice to use if the UE does not explicitly specify it. This slice must exist in the `slice` block.
        pub default_slice_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the Azure Region where the Mobile Network Sim Policy should exist. Changing this forces a new Mobile Network Sim Policies to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Mobile Network which the Sim Policy belongs to. Changing this forces a new Mobile Network Sim Policies to be created.
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim Policies. Changing this forces a new Mobile Network Sim Policies to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        pub rat_frequency_selection_priority_index: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// Interval for the user equipment periodic registration update procedure. Defaults to `3240`.
        pub registration_timer_in_seconds: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them. The list must not contain duplicate items and must contain at least one item.
        pub slices: pulumi_gestalt_rust::Output<
            Vec<super::super::types::mobile::NetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        pub user_equipment_aggregate_maximum_bit_rate: pulumi_gestalt_rust::Output<
            super::super::types::mobile::NetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkSimPolicyArgs,
    ) -> NetworkSimPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let default_slice_id_binding = args.default_slice_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let mobile_network_id_binding = args.mobile_network_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let rat_frequency_selection_priority_index_binding = args
            .rat_frequency_selection_priority_index
            .get_output(context);
        let registration_timer_in_seconds_binding = args
            .registration_timer_in_seconds
            .get_output(context);
        let slices_binding = args.slices.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let user_equipment_aggregate_maximum_bit_rate_binding = args
            .user_equipment_aggregate_maximum_bit_rate
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkSimPolicy:NetworkSimPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultSliceId".into(),
                    value: default_slice_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: mobile_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ratFrequencySelectionPriorityIndex".into(),
                    value: rat_frequency_selection_priority_index_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registrationTimerInSeconds".into(),
                    value: registration_timer_in_seconds_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "slices".into(),
                    value: slices_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "userEquipmentAggregateMaximumBitRate".into(),
                    value: user_equipment_aggregate_maximum_bit_rate_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkSimPolicyResult {
            default_slice_id: o.get_field("defaultSliceId"),
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            rat_frequency_selection_priority_index: o
                .get_field("ratFrequencySelectionPriorityIndex"),
            registration_timer_in_seconds: o.get_field("registrationTimerInSeconds"),
            slices: o.get_field("slices"),
            tags: o.get_field("tags"),
            user_equipment_aggregate_maximum_bit_rate: o
                .get_field("userEquipmentAggregateMaximumBitRate"),
        }
    }
}
