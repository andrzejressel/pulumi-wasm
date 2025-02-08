/// Manages an App Service Virtual Network Association for [Regional VNet Integration](https://docs.microsoft.com/azure/app-service/web-sites-integrate-with-vnet#regional-vnet-integration).
///
/// This resource can be used for both App Services and Function Apps.
///
/// > **Note:** The following resources support associating the vNet for Regional vNet Integration directly on the resource and via the `azure.appservice.VirtualNetworkSwiftConnection` resource. You can't use both simultaneously.
///
/// - azure.appservice.LinuxFunctionApp
/// - azure.appservice.LinuxFunctionAppSlot
/// - azure.appservice.LinuxWebApp
/// - azure.appservice.LinuxWebAppSlot
/// - azure.logicapps.Standard
/// - azure.appservice.WindowsFunctionApp
/// - azure.appservice.WindowsFunctionAppSlot
/// - azure.appservice.WindowsWebApp
/// - azure.appservice.WindowsWebAppSlot
///
/// This resource requires the `Microsoft.Network/virtualNetworks/subnets/write` permission scope on the subnet.
///
/// The resource specific vNet integration requires the `Microsoft.Network/virtualNetworks/subnets/join/action` permission scope.
///
/// There is a hard limit of [one VNet integration per App Service Plan](https://docs.microsoft.com/azure/app-service/web-sites-integrate-with-vnet#regional-vnet-integration).
/// Multiple apps in the same App Service plan can use the same VNet.
///
/// ## Example Usage
///
/// ### With App Service)
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
///     let exampleAppService = app_service::create(
///         "exampleAppService",
///         AppServiceArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("example-app-service")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("example-delegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/action",])
///                     .name("Microsoft.Web/serverFarms").build_struct()).build_struct(),
///                 ],
///             )
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-virtual-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkSwiftConnection = virtual_network_swift_connection::create(
///         "exampleVirtualNetworkSwiftConnection",
///         VirtualNetworkSwiftConnectionArgs::builder()
///             .app_service_id("${exampleAppService.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ### With Function App)
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
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .account_replication_type("LRS")
///             .account_tier("Standard")
///             .location("${example.location}")
///             .name("functionsappexamplesa")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleFunctionApp = function_app::create(
///         "exampleFunctionApp",
///         FunctionAppArgs::builder()
///             .app_service_plan_id("${examplePlan.id}")
///             .location("${example.location}")
///             .name("example-function-app")
///             .resource_group_name("${example.name}")
///             .storage_account_access_key("${exampleAccount.primaryAccessKey}")
///             .storage_account_name("${exampleAccount.name}")
///             .build_struct(),
///     );
///     let examplePlan = plan::create(
///         "examplePlan",
///         PlanArgs::builder()
///             .location("${example.location}")
///             .name("example-app-service-plan")
///             .resource_group_name("${example.name}")
///             .sku(PlanSku::builder().size("S1").tier("Standard").build_struct())
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.1.0/24",])
///             .delegations(
///                 vec![
///                     SubnetDelegation::builder().name("example-delegation")
///                     .serviceDelegation(SubnetDelegationServiceDelegation::builder()
///                     .actions(vec!["Microsoft.Network/virtualNetworks/subnets/action",])
///                     .name("Microsoft.Web/serverFarms").build_struct()).build_struct(),
///                 ],
///             )
///             .name("example-subnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-virtual-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualNetworkSwiftConnection = virtual_network_swift_connection::create(
///         "exampleVirtualNetworkSwiftConnection",
///         VirtualNetworkSwiftConnectionArgs::builder()
///             .app_service_id("${exampleFunctionApp.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// App Service Virtual Network Associations can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/virtualNetworkSwiftConnection:VirtualNetworkSwiftConnection myassociation /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Web/sites/instance1/config/virtualNetwork
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod virtual_network_swift_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualNetworkSwiftConnectionArgs {
        /// The ID of the App Service or Function App to associate to the VNet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub app_service_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the subnet the app service will be associated to (the subnet must have a `service_delegation` configured for `Microsoft.Web/serverFarms`).
        #[builder(into)]
        pub subnet_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualNetworkSwiftConnectionResult {
        /// The ID of the App Service or Function App to associate to the VNet. Changing this forces a new resource to be created.
        pub app_service_id: pulumi_gestalt_rust::Output<String>,
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
        args: VirtualNetworkSwiftConnectionArgs,
    ) -> VirtualNetworkSwiftConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_service_id_binding = args.app_service_id.get_output(context).get_inner();
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/virtualNetworkSwiftConnection:VirtualNetworkSwiftConnection"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceId".into(),
                    value: &app_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualNetworkSwiftConnectionResult {
            app_service_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceId"),
            ),
            subnet_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("subnetId"),
            ),
        }
    }
}
