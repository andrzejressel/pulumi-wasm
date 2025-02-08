/// Manages a Load Balancer Resource.
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
///             .name("LoadBalancerRG")
///             .build_struct(),
///     );
///     let exampleLoadBalancer = load_balancer::create(
///         "exampleLoadBalancer",
///         LoadBalancerArgs::builder()
///             .frontend_ip_configurations(
///                 vec![
///                     LoadBalancerFrontendIpConfiguration::builder()
///                     .name("PublicIPAddress").publicIpAddressId("${examplePublicIp.id}")
///                     .build_struct(),
///                 ],
///             )
///             .location("${example.location}")
///             .name("TestLoadBalancer")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePublicIp = public_ip::create(
///         "examplePublicIp",
///         PublicIpArgs::builder()
///             .allocation_method("Static")
///             .location("${example.location}")
///             .name("PublicIPForLB")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Load Balancers can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:lb/loadBalancer:LoadBalancer example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/loadBalancers/lb1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod load_balancer {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoadBalancerArgs {
        /// Specifies the Edge Zone within the Azure Region where this Load Balancer should exist. Changing this forces a new Load Balancer to be created.
        #[builder(into, default)]
        pub edge_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `frontend_ip_configuration` blocks as documented below.
        #[builder(into, default)]
        pub frontend_ip_configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::lb::LoadBalancerFrontendIpConfiguration>>,
        >,
        /// Specifies the supported Azure Region where the Load Balancer should be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Load Balancer. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group in which to create the Load Balancer. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The SKU of the Azure Load Balancer. Accepted values are `Basic`, `Standard` and `Gateway`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `Microsoft.Network/AllowGatewayLoadBalancer` feature is required to be registered in order to use the `Gateway` SKU. The feature can only be registered by the Azure service team, please submit an [Azure support ticket](https://azure.microsoft.com/en-us/support/create-ticket/) for that.
        #[builder(into, default)]
        pub sku: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// `sku_tier` - (Optional) The SKU tier of this Load Balancer. Possible values are `Global` and `Regional`. Defaults to `Regional`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub sku_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LoadBalancerResult {
        /// Specifies the Edge Zone within the Azure Region where this Load Balancer should exist. Changing this forces a new Load Balancer to be created.
        pub edge_zone: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `frontend_ip_configuration` blocks as documented below.
        pub frontend_ip_configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::lb::LoadBalancerFrontendIpConfiguration>>,
        >,
        /// Specifies the supported Azure Region where the Load Balancer should be created. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Load Balancer. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Private IP Address to assign to the Load Balancer.
        pub private_ip_address: pulumi_gestalt_rust::Output<String>,
        /// The list of private IP address assigned to the load balancer in `frontend_ip_configuration` blocks, if any.
        pub private_ip_addresses: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The name of the Resource Group in which to create the Load Balancer. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The SKU of the Azure Load Balancer. Accepted values are `Basic`, `Standard` and `Gateway`. Defaults to `Standard`. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** The `Microsoft.Network/AllowGatewayLoadBalancer` feature is required to be registered in order to use the `Gateway` SKU. The feature can only be registered by the Azure service team, please submit an [Azure support ticket](https://azure.microsoft.com/en-us/support/create-ticket/) for that.
        pub sku: pulumi_gestalt_rust::Output<Option<String>>,
        /// `sku_tier` - (Optional) The SKU tier of this Load Balancer. Possible values are `Global` and `Regional`. Defaults to `Regional`. Changing this forces a new resource to be created.
        pub sku_tier: pulumi_gestalt_rust::Output<Option<String>>,
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
        args: LoadBalancerArgs,
    ) -> LoadBalancerResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let edge_zone_binding = args.edge_zone.get_output(context).get_inner();
        let frontend_ip_configurations_binding = args
            .frontend_ip_configurations
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let sku_tier_binding = args.sku_tier.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:lb/loadBalancer:LoadBalancer".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "edgeZone".into(),
                    value: &edge_zone_binding,
                },
                register_interface::ObjectField {
                    name: "frontendIpConfigurations".into(),
                    value: &frontend_ip_configurations_binding,
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
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "skuTier".into(),
                    value: &sku_tier_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        LoadBalancerResult {
            edge_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("edgeZone"),
            ),
            frontend_ip_configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("frontendIpConfigurations"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            private_ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateIpAddresses"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            sku: pulumi_gestalt_rust::__private::into_domain(o.extract_field("sku")),
            sku_tier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuTier"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
