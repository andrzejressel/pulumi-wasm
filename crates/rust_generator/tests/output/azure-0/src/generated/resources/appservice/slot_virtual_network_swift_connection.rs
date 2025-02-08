/// Manages an App Service Slot's Virtual Network Association (this is for the [Regional VNet Integration](https://docs.microsoft.com/azure/app-service/web-sites-integrate-with-vnet#regional-vnet-integration) which is still in preview).
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
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-virtual-network
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: example-subnet
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.1.0/24
///       delegations:
///         - name: example-delegation
///           serviceDelegation:
///             name: Microsoft.Web/serverFarms
///             actions:
///               - Microsoft.Network/virtualNetworks/subnets/action
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: example-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: example-app-service
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///   example-staging:
///     type: azure:appservice:Slot
///     properties:
///       name: staging
///       appServiceName: ${exampleAppService.name}
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///   exampleSlotVirtualNetworkSwiftConnection:
///     type: azure:appservice:SlotVirtualNetworkSwiftConnection
///     name: example
///     properties:
///       slotName: ${["example-staging"].name}
///       appServiceId: ${exampleAppService.id}
///       subnetId: ${exampleSubnet.id}
/// ```
///
/// ## Import
///
/// App Service Slot Virtual Network Associations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/slotVirtualNetworkSwiftConnection:SlotVirtualNetworkSwiftConnection myassociation /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1/slots/staging/config/virtualNetwork
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod slot_virtual_network_swift_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SlotVirtualNetworkSwiftConnectionArgs {
        /// The ID of the App Service or Function App to associate to the VNet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the App Service Slot or Function App Slot. Changing this forces a new resource to be created.
        #[builder(into)]
        pub slot_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the subnet the app service will be associated to (the subnet must have a `service_delegation` configured for `Microsoft.Web/serverFarms`).
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct SlotVirtualNetworkSwiftConnectionResult {
        /// The ID of the App Service or Function App to associate to the VNet. Changing this forces a new resource to be created.
        pub app_service_id: pulumi_gestalt_rust::Output<String>,
        /// The name of the App Service Slot or Function App Slot. Changing this forces a new resource to be created.
        pub slot_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the subnet the app service will be associated to (the subnet must have a `service_delegation` configured for `Microsoft.Web/serverFarms`).
        pub subnet_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SlotVirtualNetworkSwiftConnectionArgs,
    ) -> SlotVirtualNetworkSwiftConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_service_id_binding = args.app_service_id.get_output(context).get_inner();
        let slot_name_binding = args.slot_name.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/slotVirtualNetworkSwiftConnection:SlotVirtualNetworkSwiftConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceId".into(),
                    value: &app_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "slotName".into(),
                    value: &slot_name_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SlotVirtualNetworkSwiftConnectionResult {
            app_service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceId"),
            ),
            slot_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("slotName"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
        }
    }
}
