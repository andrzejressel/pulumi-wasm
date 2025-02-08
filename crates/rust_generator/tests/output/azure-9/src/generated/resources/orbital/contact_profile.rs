/// Manages a Contact profile.
///
/// > **Note:** The `azure.orbital.ContactProfile` resource has been deprecated and will be removed in v5.0 of the AzureRM Provider.
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
///             .name("rg-example")
///             .build_struct(),
///     );
///     let exampleContactProfile = contact_profile::create(
///         "exampleContactProfile",
///         ContactProfileArgs::builder()
///             .auto_tracking("disabled")
///             .links(
///                 vec![
///                     ContactProfileLink::builder()
///                     .channels(vec![ContactProfileLinkChannel::builder().bandwidthMhz(100)
///                     .centerFrequencyMhz(101)
///                     .endPoints(vec![ContactProfileLinkChannelEndPoint::builder()
///                     .endPointName("AQUA_command").ipAddress("10.0.1.0").port("49513")
///                     .protocol("TCP").build_struct(),]).name("channelname")
///                     .build_struct(),]).direction("Uplink").name("RHCP_UL")
///                     .polarization("RHCP").build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .minimum_variable_contact_duration("PT1M")
///             .name("example-contact-profile")
///             .network_configuration_subnet_id("${exampleSubnet.id}")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("orbitalgateway")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/publicIPAddresses/join/action",
///                     "Microsoft.Network/virtualNetworks/subnets/join/action",
///                     "Microsoft.Network/virtualNetworks/read",
///                     "Microsoft.Network/publicIPAddresses/read",])
///                     .name("Microsoft.Orbital/orbitalGateways").build_struct())
///                     .build_struct(),
///                 ],
///             )
///             .name("testsubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("testvnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Contact profile can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:orbital/contactProfile:ContactProfile example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Orbital/contactProfiles/contactProfile1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod contact_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ContactProfileArgs {
        /// Auto-tracking configurations for a spacecraft. Possible values are `disabled`, `xBand` and `sBand`.
        #[builder(into)]
        pub auto_tracking: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARM resource identifier of the Event Hub used for telemetry. Requires granting Orbital Resource Provider the rights to send telemetry into the hub.
        #[builder(into, default)]
        pub event_hub_uri: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A list of spacecraft links. A `links` block as defined below. Changing this forces a new resource to be created.
        #[builder(into)]
        pub links: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::orbital::ContactProfileLink>,
        >,
        /// The location where the contact profile exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Maximum elevation of the antenna during the contact in decimal degrees.
        #[builder(into, default)]
        pub minimum_elevation_degrees: pulumi_gestalt_rust::InputOrOutput<Option<f64>>,
        /// Minimum viable contact duration in ISO 8601 format. Used for listing the available contacts with a spacecraft at a given ground station.
        #[builder(into)]
        pub minimum_variable_contact_duration: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// The name of the contact profile. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARM resource identifier of the subnet delegated to the Microsoft.Orbital/orbitalGateways. Needs to be at least a class C subnet, and should not have any IP created in it. Changing this forces a new resource to be created.
        #[builder(into)]
        pub network_configuration_subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the contact profile exists. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ContactProfileResult {
        /// Auto-tracking configurations for a spacecraft. Possible values are `disabled`, `xBand` and `sBand`.
        pub auto_tracking: pulumi_gestalt_rust::Output<String>,
        /// ARM resource identifier of the Event Hub used for telemetry. Requires granting Orbital Resource Provider the rights to send telemetry into the hub.
        pub event_hub_uri: pulumi_gestalt_rust::Output<Option<String>>,
        /// A list of spacecraft links. A `links` block as defined below. Changing this forces a new resource to be created.
        pub links: pulumi_gestalt_rust::Output<
            Vec<super::super::types::orbital::ContactProfileLink>,
        >,
        /// The location where the contact profile exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Maximum elevation of the antenna during the contact in decimal degrees.
        pub minimum_elevation_degrees: pulumi_gestalt_rust::Output<Option<f64>>,
        /// Minimum viable contact duration in ISO 8601 format. Used for listing the available contacts with a spacecraft at a given ground station.
        pub minimum_variable_contact_duration: pulumi_gestalt_rust::Output<String>,
        /// The name of the contact profile. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARM resource identifier of the subnet delegated to the Microsoft.Orbital/orbitalGateways. Needs to be at least a class C subnet, and should not have any IP created in it. Changing this forces a new resource to be created.
        pub network_configuration_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the contact profile exists. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ContactProfileArgs,
    ) -> ContactProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_tracking_binding = args.auto_tracking.get_output(context).get_inner();
        let event_hub_uri_binding = args.event_hub_uri.get_output(context).get_inner();
        let links_binding = args.links.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let minimum_elevation_degrees_binding = args
            .minimum_elevation_degrees
            .get_output(context)
            .get_inner();
        let minimum_variable_contact_duration_binding = args
            .minimum_variable_contact_duration
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_configuration_subnet_id_binding = args
            .network_configuration_subnet_id
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:orbital/contactProfile:ContactProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoTracking".into(),
                    value: &auto_tracking_binding,
                },
                register_interface::ObjectField {
                    name: "eventHubUri".into(),
                    value: &event_hub_uri_binding,
                },
                register_interface::ObjectField {
                    name: "links".into(),
                    value: &links_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minimumElevationDegrees".into(),
                    value: &minimum_elevation_degrees_binding,
                },
                register_interface::ObjectField {
                    name: "minimumVariableContactDuration".into(),
                    value: &minimum_variable_contact_duration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkConfigurationSubnetId".into(),
                    value: &network_configuration_subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ContactProfileResult {
            auto_tracking: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoTracking"),
            ),
            event_hub_uri: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("eventHubUri"),
            ),
            links: pulumi_gestalt_rust::__private::into_domain(o.extract_field("links")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            minimum_elevation_degrees: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumElevationDegrees"),
            ),
            minimum_variable_contact_duration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("minimumVariableContactDuration"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            network_configuration_subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkConfigurationSubnetId"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
