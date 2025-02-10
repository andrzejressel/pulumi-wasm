#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_network_sim_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetNetworkSimPolicyArgs {
        /// The ID of the Mobile Network which the Sim Policy belongs to.
        #[builder(into)]
        pub mobile_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim Policies.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetNetworkSimPolicyResult {
        /// The ID of default slice to use if the UE does not explicitly specify it.
        pub default_slice_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Mobile Network Sim Policy should exist.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub mobile_network_id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// RAT/Frequency Selection Priority Index, defined in 3GPP TS 36.413.
        pub rat_frequency_selection_priority_index: pulumi_gestalt_rust::Output<i32>,
        /// Interval for the UE periodic registration update procedure.
        pub registration_timer_in_seconds: pulumi_gestalt_rust::Output<i32>,
        /// An array of `slice` block as defined below. The allowed slices and the settings to use for them.
        pub slices: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::mobile::GetNetworkSimPolicySlice>,
        >,
        /// A mapping of tags which should be assigned to the Mobile Network Sim Policies.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// A `user_equipment_aggregate_maximum_bit_rate` block as defined below.
        pub user_equipment_aggregate_maximum_bit_rates: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mobile::GetNetworkSimPolicyUserEquipmentAggregateMaximumBitRate,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetNetworkSimPolicyArgs,
    ) -> GetNetworkSimPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let mobile_network_id_binding = args.mobile_network_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:mobile/getNetworkSimPolicy:getNetworkSimPolicy".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkId".into(),
                    value: mobile_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetNetworkSimPolicyResult {
            default_slice_id: o.get_field("defaultSliceId"),
            id: o.get_field("id"),
            location: o.get_field("location"),
            mobile_network_id: o.get_field("mobileNetworkId"),
            name: o.get_field("name"),
            rat_frequency_selection_priority_index: o
                .get_field("ratFrequencySelectionPriorityIndex"),
            registration_timer_in_seconds: o.get_field("registrationTimerInSeconds"),
            slices: o.get_field("slices"),
            tags: o.get_field("tags"),
            user_equipment_aggregate_maximum_bit_rates: o
                .get_field("userEquipmentAggregateMaximumBitRates"),
        }
    }
}
