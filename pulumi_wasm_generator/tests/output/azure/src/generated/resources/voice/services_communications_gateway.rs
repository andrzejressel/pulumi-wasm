/// Manages a Voice Services Communications Gateways.
///
/// !> **NOTE:** You must have signed an Operator Connect agreement with Microsoft to use this resource. For more information, see [`Prerequisites`](https://learn.microsoft.com/en-us/azure/communications-gateway/prepare-to-deploy#prerequisites).
///
/// !> **NOTE:** Access to Azure Communications Gateway is restricted, see [`Get access to Azure Communications Gateway for your Azure subscription`](https://learn.microsoft.com/en-us/azure/communications-gateway/prepare-to-deploy#9-get-access-to-azure-communications-gateway-for-your-azure-subscription) for details.
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
///   exampleServicesCommunicationsGateway:
///     type: azure:voice:ServicesCommunicationsGateway
///     name: example
///     properties:
///       name: example-vcg
///       location: West Europe
///       resourceGroupName: ${example.name}
///       connectivity: PublicAddress
///       codecs: PCMA
///       e911Type: DirectToEsrp
///       platforms:
///         - OperatorConnect
///         - TeamsPhoneMobile
///       serviceLocations:
///         - location: eastus
///           allowedMediaSourceAddressPrefixes:
///             - 10.1.2.0/24
///           allowedSignalingSourceAddressPrefixes:
///             - 10.1.1.0/24
///           esrpAddresses:
///             - 198.51.100.3
///           operatorAddresses:
///             - 198.51.100.1
///         - location: eastus2
///           allowedMediaSourceAddressPrefixes:
///             - 10.2.2.0/24
///           allowedSignalingSourceAddressPrefixes:
///             - 10.2.1.0/24
///           esrpAddresses:
///             - 198.51.100.4
///           operatorAddresses:
///             - 198.51.100.2
///       autoGeneratedDomainNameLabelScope: SubscriptionReuse
///       apiBridge:
///         fn::toJSON: {}
///       emergencyDialStrings:
///         - '911'
///         - '933'
///       onPremMcpEnabled: false
///       tags:
///         key: value
///       microsoftTeamsVoicemailPilotNumber: '1'
/// ```
///
/// ## Import
///
/// Voice Services Communications Gateways can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:voice/servicesCommunicationsGateway:ServicesCommunicationsGateway example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.VoiceServices/communicationsGateways/communicationsGateway1
/// ```
///
pub mod services_communications_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayArgs {
        /// Details of API bridge functionality, if required.
        #[builder(into, default)]
        pub api_bridge: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the scope at which the auto-generated domain name can be re-used. Possible values are `TenantReuse`, `SubscriptionReuse`, `ResourceGroupReuse` and `NoReuse` . Changing this forces a new resource to be created. Defaults to `TenantReuse`.
        #[builder(into, default)]
        pub auto_generated_domain_name_label_scope: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The voice codecs expected for communication with Teams. Possible values are `PCMA`, `PCMU`,`G722`,`G722_2`,`SILK_8` and `SILK_16`.
        #[builder(into)]
        pub codecs: pulumi_wasm_rust::Output<String>,
        /// How to connect back to the operator network, e.g. MAPS. Possible values is `PublicAddress`. Changing this forces a new Voice Services Communications Gateways to be created.
        #[builder(into)]
        pub connectivity: pulumi_wasm_rust::Output<String>,
        /// How to handle 911 calls. Possible values are `Standard` and `DirectToEsrp`.
        #[builder(into)]
        pub e911_type: pulumi_wasm_rust::Output<String>,
        /// A list of dial strings used for emergency calling.
        #[builder(into, default)]
        pub emergency_dial_strings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Azure Region where the Voice Services Communications Gateways should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// This number is used in Teams Phone Mobile scenarios for access to the voicemail IVR from the native dialer.
        #[builder(into, default)]
        pub microsoft_teams_voicemail_pilot_number: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the name which should be used for this Voice Services Communications Gateways. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether an on-premises Mobile Control Point is in use.
        #[builder(into, default)]
        pub on_prem_mcp_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Voice Services Communications GatewaysAvailable supports platform types. Possible values are `OperatorConnect`, `TeamsPhoneMobile`.
        #[builder(into)]
        pub platforms: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name of the Resource Group where the Voice Services Communications Gateways should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `service_location` block as defined below.
        #[builder(into)]
        pub service_locations: pulumi_wasm_rust::Output<
            Vec<super::super::types::voice::ServicesCommunicationsGatewayServiceLocation>,
        >,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateways.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServicesCommunicationsGatewayResult {
        /// Details of API bridge functionality, if required.
        pub api_bridge: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the scope at which the auto-generated domain name can be re-used. Possible values are `TenantReuse`, `SubscriptionReuse`, `ResourceGroupReuse` and `NoReuse` . Changing this forces a new resource to be created. Defaults to `TenantReuse`.
        pub auto_generated_domain_name_label_scope: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// The voice codecs expected for communication with Teams. Possible values are `PCMA`, `PCMU`,`G722`,`G722_2`,`SILK_8` and `SILK_16`.
        pub codecs: pulumi_wasm_rust::Output<String>,
        /// How to connect back to the operator network, e.g. MAPS. Possible values is `PublicAddress`. Changing this forces a new Voice Services Communications Gateways to be created.
        pub connectivity: pulumi_wasm_rust::Output<String>,
        /// How to handle 911 calls. Possible values are `Standard` and `DirectToEsrp`.
        pub e911_type: pulumi_wasm_rust::Output<String>,
        /// A list of dial strings used for emergency calling.
        pub emergency_dial_strings: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies the Azure Region where the Voice Services Communications Gateways should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// This number is used in Teams Phone Mobile scenarios for access to the voicemail IVR from the native dialer.
        pub microsoft_teams_voicemail_pilot_number: pulumi_wasm_rust::Output<
            Option<String>,
        >,
        /// Specifies the name which should be used for this Voice Services Communications Gateways. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether an on-premises Mobile Control Point is in use.
        pub on_prem_mcp_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Voice Services Communications GatewaysAvailable supports platform types. Possible values are `OperatorConnect`, `TeamsPhoneMobile`.
        pub platforms: pulumi_wasm_rust::Output<Vec<String>>,
        /// Specifies the name of the Resource Group where the Voice Services Communications Gateways should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `service_location` block as defined below.
        pub service_locations: pulumi_wasm_rust::Output<
            Vec<super::super::types::voice::ServicesCommunicationsGatewayServiceLocation>,
        >,
        /// A mapping of tags which should be assigned to the Voice Services Communications Gateways.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ServicesCommunicationsGatewayArgs,
    ) -> ServicesCommunicationsGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let api_bridge_binding = args.api_bridge.get_inner();
        let auto_generated_domain_name_label_scope_binding = args
            .auto_generated_domain_name_label_scope
            .get_inner();
        let codecs_binding = args.codecs.get_inner();
        let connectivity_binding = args.connectivity.get_inner();
        let e911_type_binding = args.e911_type.get_inner();
        let emergency_dial_strings_binding = args.emergency_dial_strings.get_inner();
        let location_binding = args.location.get_inner();
        let microsoft_teams_voicemail_pilot_number_binding = args
            .microsoft_teams_voicemail_pilot_number
            .get_inner();
        let name_binding = args.name.get_inner();
        let on_prem_mcp_enabled_binding = args.on_prem_mcp_enabled.get_inner();
        let platforms_binding = args.platforms.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let service_locations_binding = args.service_locations.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:voice/servicesCommunicationsGateway:ServicesCommunicationsGateway"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "apiBridge".into(),
                    value: &api_bridge_binding,
                },
                register_interface::ObjectField {
                    name: "autoGeneratedDomainNameLabelScope".into(),
                    value: &auto_generated_domain_name_label_scope_binding,
                },
                register_interface::ObjectField {
                    name: "codecs".into(),
                    value: &codecs_binding,
                },
                register_interface::ObjectField {
                    name: "connectivity".into(),
                    value: &connectivity_binding,
                },
                register_interface::ObjectField {
                    name: "e911Type".into(),
                    value: &e911_type_binding,
                },
                register_interface::ObjectField {
                    name: "emergencyDialStrings".into(),
                    value: &emergency_dial_strings_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "microsoftTeamsVoicemailPilotNumber".into(),
                    value: &microsoft_teams_voicemail_pilot_number_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "onPremMcpEnabled".into(),
                    value: &on_prem_mcp_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "platforms".into(),
                    value: &platforms_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLocations".into(),
                    value: &service_locations_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "apiBridge".into(),
                },
                register_interface::ResultField {
                    name: "autoGeneratedDomainNameLabelScope".into(),
                },
                register_interface::ResultField {
                    name: "codecs".into(),
                },
                register_interface::ResultField {
                    name: "connectivity".into(),
                },
                register_interface::ResultField {
                    name: "e911Type".into(),
                },
                register_interface::ResultField {
                    name: "emergencyDialStrings".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "microsoftTeamsVoicemailPilotNumber".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "onPremMcpEnabled".into(),
                },
                register_interface::ResultField {
                    name: "platforms".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "serviceLocations".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServicesCommunicationsGatewayResult {
            api_bridge: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("apiBridge").unwrap(),
            ),
            auto_generated_domain_name_label_scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoGeneratedDomainNameLabelScope").unwrap(),
            ),
            codecs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("codecs").unwrap(),
            ),
            connectivity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectivity").unwrap(),
            ),
            e911_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("e911Type").unwrap(),
            ),
            emergency_dial_strings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("emergencyDialStrings").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            microsoft_teams_voicemail_pilot_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("microsoftTeamsVoicemailPilotNumber").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            on_prem_mcp_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("onPremMcpEnabled").unwrap(),
            ),
            platforms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("platforms").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            service_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceLocations").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}