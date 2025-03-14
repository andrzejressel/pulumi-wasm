/// Manages a Mobile Network Sim.
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
///     let exampleNetwork = network::create(
///         "exampleNetwork",
///         NetworkArgs::builder()
///             .location("${example.location}")
///             .mobile_country_code("001")
///             .mobile_network_code("01")
///             .name("example-mn")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleNetworkAttachedDataNetwork = network_attached_data_network::create(
///         "exampleNetworkAttachedDataNetwork",
///         NetworkAttachedDataNetworkArgs::builder()
///             .dns_addresses(vec!["1.1.1.1",])
///             .location("${example.location}")
///             .mobile_network_data_network_name(
///                 "${exampleAzurermMobileNetworkDataNetwork.name}",
///             )
///             .mobile_network_packet_core_data_plane_id(
///                 "${exampleAzurermMobileNetworkPacketCoreDataPlane.id}",
///             )
///             .user_equipment_address_pool_prefixes(vec!["2.4.0.0/24",])
///             .user_equipment_static_address_pool_prefixes(vec!["2.4.1.0/24",])
///             .user_plane_access_ipv_4_address("10.204.141.4")
///             .user_plane_access_ipv_4_gateway("10.204.141.1")
///             .user_plane_access_ipv_4_subnet("10.204.141.0/24")
///             .user_plane_access_name("test")
///             .build_struct(),
///     );
///     let exampleNetworkSim = network_sim::create(
///         "exampleNetworkSim",
///         NetworkSimArgs::builder()
///             .authentication_key("00000000000000000000000000000000")
///             .integrated_circuit_card_identifier("8900000000000000000")
///             .international_mobile_subscriber_identity("000000000000000")
///             .mobile_network_sim_group_id("${exampleNetworkSimGroup.id}")
///             .name("example-sim")
///             .operator_key_code("00000000000000000000000000000000")
///             .static_ip_configurations(
///                 vec![
///                     NetworkSimStaticIpConfiguration::builder()
///                     .attachedDataNetworkId("${test.id}")
///                     .sliceId("${testAzurermMobileNetworkSlice.id}")
///                     .staticIpv4Address("2.4.0.1").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleNetworkSimGroup = network_sim_group::create(
///         "exampleNetworkSimGroup",
///         NetworkSimGroupArgs::builder()
///             .location("${example.location}")
///             .mobile_network_id("${exampleNetwork.id}")
///             .name("example-mnsg")
///             .build_struct(),
///     );
///     let exampleNetworkSlice = network_slice::create(
///         "exampleNetworkSlice",
///         NetworkSliceArgs::builder()
///             .location("${example.location}")
///             .mobile_network_id("${exampleNetwork.id}")
///             .name("example-slice")
///             .single_network_slice_selection_assistance_information(
///                 NetworkSliceSingleNetworkSliceSelectionAssistanceInformation::builder()
///                     .sliceServiceType(1)
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Mobile Network Sim can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mobile/networkSim:NetworkSim example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.MobileNetwork/simGroups/simGroup1/sims/sim1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod network_sim {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NetworkSimArgs {
        /// The Ki value for the SIM.
        #[builder(into)]
        pub authentication_key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// An optional free-form text field that can be used to record the device type this SIM is associated with, for example `Video camera`. The Azure portal allows SIMs to be grouped and filtered based on this value.
        #[builder(into, default)]
        pub device_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The integrated circuit card ID (ICCID) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub integrated_circuit_card_identifier: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The international mobile subscriber identity (IMSI) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub international_mobile_subscriber_identity: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into)]
        pub mobile_network_sim_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Mobile Network Sim. Changing this forces a new Mobile Network Sim to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Opc value for the SIM.
        #[builder(into)]
        pub operator_key_code: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of SIM policy used by this SIM.
        #[builder(into, default)]
        pub sim_policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `static_ip_configuration` block as defined below.
        #[builder(into, default)]
        pub static_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::mobile::NetworkSimStaticIpConfiguration>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NetworkSimResult {
        /// The Ki value for the SIM.
        pub authentication_key: pulumi_gestalt_rust::Output<String>,
        /// An optional free-form text field that can be used to record the device type this SIM is associated with, for example `Video camera`. The Azure portal allows SIMs to be grouped and filtered based on this value.
        pub device_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The integrated circuit card ID (ICCID) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        pub integrated_circuit_card_identifier: pulumi_gestalt_rust::Output<String>,
        /// The international mobile subscriber identity (IMSI) for the SIM. Changing this forces a new Mobile Network Sim to be created.
        pub international_mobile_subscriber_identity: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The ID of the Mobile Network which the Mobile Network Sim belongs to. Changing this forces a new Mobile Network Sim to be created.
        pub mobile_network_sim_group_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Mobile Network Sim. Changing this forces a new Mobile Network Sim to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Opc value for the SIM.
        pub operator_key_code: pulumi_gestalt_rust::Output<String>,
        /// The ID of SIM policy used by this SIM.
        pub sim_policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The state of the SIM resource.
        pub sim_state: pulumi_gestalt_rust::Output<String>,
        /// A `static_ip_configuration` block as defined below.
        pub static_ip_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::mobile::NetworkSimStaticIpConfiguration>>,
        >,
        /// The public key fingerprint of the SIM vendor who provided this SIM, if any.
        pub vendor_key_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name of the SIM vendor who provided this SIM, if any.
        pub vendor_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NetworkSimArgs,
    ) -> NetworkSimResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let authentication_key_binding = args.authentication_key.get_output(context);
        let device_type_binding = args.device_type.get_output(context);
        let integrated_circuit_card_identifier_binding = args
            .integrated_circuit_card_identifier
            .get_output(context);
        let international_mobile_subscriber_identity_binding = args
            .international_mobile_subscriber_identity
            .get_output(context);
        let mobile_network_sim_group_id_binding = args
            .mobile_network_sim_group_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let operator_key_code_binding = args.operator_key_code.get_output(context);
        let sim_policy_id_binding = args.sim_policy_id.get_output(context);
        let static_ip_configurations_binding = args
            .static_ip_configurations
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mobile/networkSim:NetworkSim".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticationKey".into(),
                    value: &authentication_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceType".into(),
                    value: &device_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integratedCircuitCardIdentifier".into(),
                    value: &integrated_circuit_card_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internationalMobileSubscriberIdentity".into(),
                    value: &international_mobile_subscriber_identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mobileNetworkSimGroupId".into(),
                    value: &mobile_network_sim_group_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatorKeyCode".into(),
                    value: &operator_key_code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "simPolicyId".into(),
                    value: &sim_policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticIpConfigurations".into(),
                    value: &static_ip_configurations_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NetworkSimResult {
            authentication_key: o.get_field("authenticationKey"),
            device_type: o.get_field("deviceType"),
            integrated_circuit_card_identifier: o
                .get_field("integratedCircuitCardIdentifier"),
            international_mobile_subscriber_identity: o
                .get_field("internationalMobileSubscriberIdentity"),
            mobile_network_sim_group_id: o.get_field("mobileNetworkSimGroupId"),
            name: o.get_field("name"),
            operator_key_code: o.get_field("operatorKeyCode"),
            sim_policy_id: o.get_field("simPolicyId"),
            sim_state: o.get_field("simState"),
            static_ip_configurations: o.get_field("staticIpConfigurations"),
            vendor_key_fingerprint: o.get_field("vendorKeyFingerprint"),
            vendor_name: o.get_field("vendorName"),
        }
    }
}
