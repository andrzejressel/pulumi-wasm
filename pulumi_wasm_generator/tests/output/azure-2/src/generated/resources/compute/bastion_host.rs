/// Manages a Bastion Host.
///
/// ## Example Usage
///
/// This example deploys an Azure Bastion Host Instance to a target virtual network.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleBastionHost = bastion_host::create(
///         "exampleBastionHost",
///         BastionHostArgs::builder()
///             .ip_configuration(
///                 BastionHostIpConfiguration::builder()
///                     .name("configuration")
///                     .publicIpAddressId("${examplePublicIp.id}")
///                     .subnetId("${exampleSubnet.id}")
///                     .build_struct(),
///             )
///             .location("${example.location}")
///             .name("examplebastion")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("examplepip")
///             .resource_group_name("${example.name}")
///             .sku("Standard")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["192.168.1.224/27",])
///             .name("AzureBastionSubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["192.168.1.0/24",])
///             .location("${example.location}")
///             .name("examplevnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Bastion Hosts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/bastionHost:BastionHost example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/bastionHosts/instance1
/// ```
///
pub mod bastion_host {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BastionHostArgs {
        /// Is Copy/Paste feature enabled for the Bastion Host. Defaults to `true`.
        #[builder(into, default)]
        pub copy_paste_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Is File Copy feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `file_copy_enabled` is only supported when `sku` is `Standard` or `Premium`.
        #[builder(into, default)]
        pub file_copy_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// A `ip_configuration` block as defined below. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub ip_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::compute::BastionHostIpConfiguration>,
        >,
        /// Is IP Connect feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `ip_connect_enabled` is only supported when `sku` is `Standard` or `Premium`.
        #[builder(into, default)]
        pub ip_connect_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Is Kerberos authentication feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `kerberos_enabled` is only supported when `sku` is `Standard` or `Premium`.
        #[builder(into, default)]
        pub kerberos_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Review [Azure Bastion Host FAQ](https://docs.microsoft.com/azure/bastion/bastion-faq) for supported locations.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Bastion Host. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the Bastion Host. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of scale units with which to provision the Bastion Host. Possible values are between `2` and `50`. Defaults to `2`.
        ///
        /// > **Note:** `scale_units` only can be changed when `sku` is `Standard` or `Premium`. `scale_units` is always `2` when `sku` is `Basic`.
        #[builder(into, default)]
        pub scale_units: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// Is Session Recording feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `session_recording_enabled` is only supported when `sku` is `Premium`.
        #[builder(into, default)]
        pub session_recording_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Is Shareable Link feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `shareable_link_enabled` is only supported when `sku` is `Standard` or `Premium`.
        #[builder(into, default)]
        pub shareable_link_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The SKU of the Bastion Host. Accepted values are `Developer`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        #[builder(into, default)]
        pub sku: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Is Tunneling feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `tunneling_enabled` is only supported when `sku` is `Standard` or `Premium`.
        #[builder(into, default)]
        pub tunneling_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Virtual Network for the Developer Bastion Host. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub virtual_network_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a list of Availability Zones in which this Public Bastion Host should be located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub zones: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct BastionHostResult {
        /// Is Copy/Paste feature enabled for the Bastion Host. Defaults to `true`.
        pub copy_paste_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The FQDN for the Bastion Host.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Is File Copy feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `file_copy_enabled` is only supported when `sku` is `Standard` or `Premium`.
        pub file_copy_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// A `ip_configuration` block as defined below. Changing this forces a new resource to be created.
        pub ip_configuration: pulumi_wasm_rust::Output<
            Option<super::super::types::compute::BastionHostIpConfiguration>,
        >,
        /// Is IP Connect feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `ip_connect_enabled` is only supported when `sku` is `Standard` or `Premium`.
        pub ip_connect_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Kerberos authentication feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `kerberos_enabled` is only supported when `sku` is `Standard` or `Premium`.
        pub kerberos_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created. Review [Azure Bastion Host FAQ](https://docs.microsoft.com/azure/bastion/bastion-faq) for supported locations.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Bastion Host. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the Bastion Host. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The number of scale units with which to provision the Bastion Host. Possible values are between `2` and `50`. Defaults to `2`.
        ///
        /// > **Note:** `scale_units` only can be changed when `sku` is `Standard` or `Premium`. `scale_units` is always `2` when `sku` is `Basic`.
        pub scale_units: pulumi_wasm_rust::Output<Option<i32>>,
        /// Is Session Recording feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `session_recording_enabled` is only supported when `sku` is `Premium`.
        pub session_recording_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Is Shareable Link feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `shareable_link_enabled` is only supported when `sku` is `Standard` or `Premium`.
        pub shareable_link_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The SKU of the Bastion Host. Accepted values are `Developer`, `Basic`, `Standard` and `Premium`. Defaults to `Basic`.
        ///
        /// > **Note** Downgrading the SKU will force a new resource to be created.
        pub sku: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Is Tunneling feature enabled for the Bastion Host. Defaults to `false`.
        ///
        /// > **Note:** `tunneling_enabled` is only supported when `sku` is `Standard` or `Premium`.
        pub tunneling_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The ID of the Virtual Network for the Developer Bastion Host. Changing this forces a new resource to be created.
        pub virtual_network_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies a list of Availability Zones in which this Public Bastion Host should be located. Changing this forces a new resource to be created.
        pub zones: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BastionHostArgs,
    ) -> BastionHostResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let copy_paste_enabled_binding = args
            .copy_paste_enabled
            .get_output(context)
            .get_inner();
        let file_copy_enabled_binding = args
            .file_copy_enabled
            .get_output(context)
            .get_inner();
        let ip_configuration_binding = args
            .ip_configuration
            .get_output(context)
            .get_inner();
        let ip_connect_enabled_binding = args
            .ip_connect_enabled
            .get_output(context)
            .get_inner();
        let kerberos_enabled_binding = args
            .kerberos_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let scale_units_binding = args.scale_units.get_output(context).get_inner();
        let session_recording_enabled_binding = args
            .session_recording_enabled
            .get_output(context)
            .get_inner();
        let shareable_link_enabled_binding = args
            .shareable_link_enabled
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let tunneling_enabled_binding = args
            .tunneling_enabled
            .get_output(context)
            .get_inner();
        let virtual_network_id_binding = args
            .virtual_network_id
            .get_output(context)
            .get_inner();
        let zones_binding = args.zones.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/bastionHost:BastionHost".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "copyPasteEnabled".into(),
                    value: &copy_paste_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "fileCopyEnabled".into(),
                    value: &file_copy_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ipConfiguration".into(),
                    value: &ip_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "ipConnectEnabled".into(),
                    value: &ip_connect_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "kerberosEnabled".into(),
                    value: &kerberos_enabled_binding,
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
                    name: "scaleUnits".into(),
                    value: &scale_units_binding,
                },
                register_interface::ObjectField {
                    name: "sessionRecordingEnabled".into(),
                    value: &session_recording_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "shareableLinkEnabled".into(),
                    value: &shareable_link_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "tunnelingEnabled".into(),
                    value: &tunneling_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "virtualNetworkId".into(),
                    value: &virtual_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "zones".into(),
                    value: &zones_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        BastionHostResult {
            copy_paste_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("copyPasteEnabled"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            file_copy_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("fileCopyEnabled"),
            ),
            ip_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipConfiguration"),
            ),
            ip_connect_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipConnectEnabled"),
            ),
            kerberos_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kerberosEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            scale_units: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("scaleUnits"),
            ),
            session_recording_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sessionRecordingEnabled"),
            ),
            shareable_link_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("shareableLinkEnabled"),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(o.extract_field("sku")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tunneling_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tunnelingEnabled"),
            ),
            virtual_network_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("virtualNetworkId"),
            ),
            zones: pulumi_wasm_rust::__private::into_domain(o.extract_field("zones")),
        }
    }
}
