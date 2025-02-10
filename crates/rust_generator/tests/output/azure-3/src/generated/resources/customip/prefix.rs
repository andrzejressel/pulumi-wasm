/// Manages a custom IPv4 prefix or custom IPv6 prefix.
///
/// ## Example Usage
///
/// *IPv4 custom prefix*
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   examplePrefix:
///     type: azure:customip:Prefix
///     name: example
///     properties:
///       name: example-CustomIPPrefix
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       cidr: 1.2.3.4/22
///       zones:
///         - '1'
///         - '2'
///         - '3'
///       commissioningEnabled: true
///       roaValidityEndDate: 2099-12-12
///       wanValidationSignedMessage: signed message for WAN validation
///       tags:
///         env: test
/// ```
///
/// *IPv6 custom prefix*
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   global:
///     type: azure:customip:Prefix
///     properties:
///       name: example-Global-CustomIPPrefix
///       location: ${test.location}
///       resourceGroupName: ${test.name}
///       cidr: 2001:db8:1::/48
///       roaValidityEndDate: 2199-12-12
///       wanValidationSignedMessage: signed message for WAN validation
///   regional:
///     type: azure:customip:Prefix
///     properties:
///       name: example-Regional-CustomIPPrefix
///       location: ${test.location}
///       resourceGroupName: ${test.name}
///       parentCustomIpPrefixId: ${global.id}
///       cidr:
///         fn::invoke:
///           function: std:cidrsubnet
///           arguments:
///             input: ${global.cidr}
///             newbits: 16
///             netnum: 1
///           return: result
///       zones:
///         - '1'
/// ```
///
/// ## Import
///
/// A Custom IP Prefix can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:customip/prefix:Prefix example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/customIPPrefixes/customIPPrefix1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod prefix {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PrefixArgs {
        /// The `cidr` of the Custom IP Prefix, either IPv4 or IPv6. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies that the custom IP prefix should be commissioned after provisioning in Azure. Defaults to `false`.
        ///
        /// !> **Warning** Changing the value of `commissioning_enabled` from `true` to `false` causes the IP prefix to stop being advertised by Azure and is functionally equivalent to deleting it when used in a production setting.
        #[builder(into, default)]
        pub commissioning_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies that the custom IP prefix should not be publicly advertised on the Internet when commissioned (regional commissioning feature). Defaults to `false`.
        ///
        /// !> **Warning** Changing the value of `internet_advertising_disabled` from `true` to `false` causes the IP prefix to stop being advertised by Azure and is functionally equivalent to deleting it when used in a production setting.
        #[builder(into, default)]
        pub internet_advertising_disabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The location where the Custom IP Prefix should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Custom IP Prefix. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the ID of the parent prefix. Only needed when creating a regional/child IPv6 prefix. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub parent_custom_ip_prefix_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the Resource Group in which to create the Custom IP Prefix. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The expiration date of the Route Origin Authorization (ROA) document which has been filed with the Routing Internet Registry (RIR) for this prefix. The expected format is `YYYY-MM-DD`. Required when provisioning an IPv4 prefix or IPv6 global prefix. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub roa_validity_end_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the Custom IP Prefix.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The signed base64-encoded authorization message, which will be sent to Microsoft for WAN verification. Required when provisioning an IPv4 prefix or IPv6 global prefix. Refer to [Azure documentation](https://learn.microsoft.com/en-us/azure/virtual-network/ip-services/create-custom-ip-address-prefix-cli#certificate-readiness) for more details about the process for your RIR. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub wan_validation_signed_message: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies a list of Availability Zones in which this Custom IP Prefix should be located. Should not be specified when creating an IPv6 global prefix. Changing this forces a new resource to be created.
        ///
        /// > **Note:** In regions with [availability zones](https://docs.microsoft.com/en-us/azure/availability-zones/az-overview), the Custom IP Prefix must be specified as either `Zone-redundant` or assigned to a specific zone. It can't be created with no zone specified in these regions. All IPs from the prefix must have the same zonal properties.
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct PrefixResult {
        /// The `cidr` of the Custom IP Prefix, either IPv4 or IPv6. Changing this forces a new resource to be created.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// Specifies that the custom IP prefix should be commissioned after provisioning in Azure. Defaults to `false`.
        ///
        /// !> **Warning** Changing the value of `commissioning_enabled` from `true` to `false` causes the IP prefix to stop being advertised by Azure and is functionally equivalent to deleting it when used in a production setting.
        pub commissioning_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies that the custom IP prefix should not be publicly advertised on the Internet when commissioned (regional commissioning feature). Defaults to `false`.
        ///
        /// !> **Warning** Changing the value of `internet_advertising_disabled` from `true` to `false` causes the IP prefix to stop being advertised by Azure and is functionally equivalent to deleting it when used in a production setting.
        pub internet_advertising_disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location where the Custom IP Prefix should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the Custom IP Prefix. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the ID of the parent prefix. Only needed when creating a regional/child IPv6 prefix. Changing this forces a new resource to be created.
        pub parent_custom_ip_prefix_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Resource Group in which to create the Custom IP Prefix. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The expiration date of the Route Origin Authorization (ROA) document which has been filed with the Routing Internet Registry (RIR) for this prefix. The expected format is `YYYY-MM-DD`. Required when provisioning an IPv4 prefix or IPv6 global prefix. Changing this forces a new resource to be created.
        pub roa_validity_end_date: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the Custom IP Prefix.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The signed base64-encoded authorization message, which will be sent to Microsoft for WAN verification. Required when provisioning an IPv4 prefix or IPv6 global prefix. Refer to [Azure documentation](https://learn.microsoft.com/en-us/azure/virtual-network/ip-services/create-custom-ip-address-prefix-cli#certificate-readiness) for more details about the process for your RIR. Changing this forces a new resource to be created.
        pub wan_validation_signed_message: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this Custom IP Prefix should be located. Should not be specified when creating an IPv6 global prefix. Changing this forces a new resource to be created.
        ///
        /// > **Note:** In regions with [availability zones](https://docs.microsoft.com/en-us/azure/availability-zones/az-overview), the Custom IP Prefix must be specified as either `Zone-redundant` or assigned to a specific zone. It can't be created with no zone specified in these regions. All IPs from the prefix must have the same zonal properties.
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PrefixArgs,
    ) -> PrefixResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cidr_binding = args.cidr.get_output(context);
        let commissioning_enabled_binding = args
            .commissioning_enabled
            .get_output(context);
        let internet_advertising_disabled_binding = args
            .internet_advertising_disabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_custom_ip_prefix_id_binding = args
            .parent_custom_ip_prefix_id
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let roa_validity_end_date_binding = args
            .roa_validity_end_date
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let wan_validation_signed_message_binding = args
            .wan_validation_signed_message
            .get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:customip/prefix:Prefix".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidr".into(),
                    value: cidr_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "commissioningEnabled".into(),
                    value: commissioning_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "internetAdvertisingDisabled".into(),
                    value: internet_advertising_disabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentCustomIpPrefixId".into(),
                    value: parent_custom_ip_prefix_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roaValidityEndDate".into(),
                    value: roa_validity_end_date_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wanValidationSignedMessage".into(),
                    value: wan_validation_signed_message_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PrefixResult {
            cidr: o.get_field("cidr"),
            commissioning_enabled: o.get_field("commissioningEnabled"),
            internet_advertising_disabled: o.get_field("internetAdvertisingDisabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            parent_custom_ip_prefix_id: o.get_field("parentCustomIpPrefixId"),
            resource_group_name: o.get_field("resourceGroupName"),
            roa_validity_end_date: o.get_field("roaValidityEndDate"),
            tags: o.get_field("tags"),
            wan_validation_signed_message: o.get_field("wanValidationSignedMessage"),
            zones: o.get_field("zones"),
        }
    }
}
