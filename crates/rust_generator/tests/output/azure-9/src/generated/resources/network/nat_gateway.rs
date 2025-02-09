/// Manages an Azure NAT Gateway.
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
///             .name("nat-gateway-example-rg")
///             .build_struct(),
///     );
///     let exampleNatGateway = nat_gateway::create(
///         "exampleNatGateway",
///         NatGatewayArgs::builder()
///             .idle_timeout_in_minutes(10)
///             .location("${example.location}")
///             .name("nat-gateway")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .zones(vec!["1",])
///             .build_struct(),
///     );
/// }
/// ```
///
/// For more complete examples, please see the azure.network.NatGatewayPublicIpAssociation and azure.network.NatGatewayPublicIpPrefixAssociation resources.
///
/// ## Import
///
/// NAT Gateway can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/natGateway:NatGateway test /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/natGateways/gateway1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod nat_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NatGatewayArgs {
        /// The idle timeout which should be used in minutes. Defaults to `4`.
        #[builder(into, default)]
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies the supported Azure location where the NAT Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the NAT Gateway. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Resource Group in which the NAT Gateway should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU which should be used. At this time the only supported value is `Standard`. Defaults to `Standard`.
        #[builder(into, default)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this NAT Gateway should be located. Changing this forces a new NAT Gateway to be created.
        ///
        /// > **NOTE:** Only one Availability Zone can be defined. For more information, please check out the [Azure documentation](https://learn.microsoft.com/en-us/azure/nat-gateway/nat-overview#availability-zones)
        #[builder(into, default)]
        pub zones: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct NatGatewayResult {
        /// The idle timeout which should be used in minutes. Defaults to `4`.
        pub idle_timeout_in_minutes: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Specifies the supported Azure location where the NAT Gateway should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the NAT Gateway. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group in which the NAT Gateway should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The resource GUID property of the NAT Gateway.
        pub resource_guid: pulumi_gestalt_rust::Output<String>,
        /// The SKU which should be used. At this time the only supported value is `Standard`. Defaults to `Standard`.
        pub sku_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A list of Availability Zones in which this NAT Gateway should be located. Changing this forces a new NAT Gateway to be created.
        ///
        /// > **NOTE:** Only one Availability Zone can be defined. For more information, please check out the [Azure documentation](https://learn.microsoft.com/en-us/azure/nat-gateway/nat-overview#availability-zones)
        pub zones: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NatGatewayArgs,
    ) -> NatGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let idle_timeout_in_minutes_binding = args
            .idle_timeout_in_minutes
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sku_name_binding = args.sku_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let zones_binding = args.zones.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/natGateway:NatGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "idleTimeoutInMinutes".into(),
                    value: idle_timeout_in_minutes_binding.get_id(),
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
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skuName".into(),
                    value: sku_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zones".into(),
                    value: zones_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NatGatewayResult {
            idle_timeout_in_minutes: o.get_field("idleTimeoutInMinutes"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            resource_guid: o.get_field("resourceGuid"),
            sku_name: o.get_field("skuName"),
            tags: o.get_field("tags"),
            zones: o.get_field("zones"),
        }
    }
}
